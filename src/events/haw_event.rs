use std::{
    collections::BTreeMap,
    env::VarError,
    error::Error,
    ffi::NulError,
    fmt,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use regex::Regex;
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};

use super::mup_scraper::MupLecture;

use crate::{events::{event::*, mup_scraper}, json_parser::{Config, Occupations}};


const DATA_URL: &str =
    "https://raw.githubusercontent.com/HAWHHCalendarBot/eventfiles/refs/heads/main/"; // /faculty/event.json

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HawEventEntry {
    pub name: String,
    pub location: String,
    pub description: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl Event for HawEventEntry {
    fn load_from_local(
        event: &Event_Meta,
        cache_dir: PathBuf,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        // Read timestamp of local event data
        let cache_dir_str = cache_dir
            .to_str()
            .ok_or("Cache directory path is not valid")?;
        let mut file: File = File::open(format!("{}/eventadata/timestamp", &cache_dir_str))?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        let last_change = chrono::DateTime::from_timestamp(contents.parse()?, 0).unwrap();
        let last_change_date = last_change.date_naive();

        // If the data is older than 1 day, fetch new data
        if chrono::Local::now()
            .date_naive()
            .signed_duration_since(last_change_date)
            > chrono::Duration::days(1)
        {
            println!("Local event data is outdated. Fetching new data...");
            Self::fetch_event_data(&cache_dir)?;
        }

        // Load new data
        let eventdata_path = Self::get_eventdata_dir(&cache_dir)?;

        let path_str = format!("{}/{}.json", &event.department, &event.module);

        let path = eventdata_path.join(Path::new(&path_str));

        if !path.exists() {
            return Err(format!(
                "No data found for module '{}' in department '{}'",
                &event.module, &event.department
            )
            .into());
        }

        // Read data
        let file_content = fs::read_to_string(&path)?;

        Ok(serde_json::from_str(&file_content)?)
    }

    fn get_all_events_for_date(
        date: NaiveDate,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let config = Config::load_config();
        let event_descriptors = config.get_events();

        if event_descriptors.is_none() {
            return Err("No event descriptors found in config".into());
        }

        let event_descriptors = event_descriptors.unwrap();

        let mut events: Vec<Self> = vec![];
        for meta in event_descriptors.iter() {
            // Split format "department:module"
            let (department, module) = meta.split_once(':').ok_or("Invalid event descriptor format")?;

            // Build event_meta
            let event_meta = Event_Meta {
                department: department.to_string(),
                module: module.to_string(),
            };

            let module_events = Self::get_events_for_module(&event_meta)?;
            events.extend(module_events);
        }

        if events.is_empty() {
            return Err(format!("No events found for date {}", date).into());
        }

        // Filter events for the specified date
        events.retain(|event| event.start.date() == date);

        Ok(events)
    }

    fn get_events_for_date(
        event_descriptor: Vec<Event_Meta>,
        date: NaiveDate,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut events: Vec<Self> = vec![];
        for module in event_descriptor.iter() {
            let module_events = Self::get_events_for_module(module)?;

            for event in module_events.iter() {
                if date.day() == event.start.day() {
                    events.push(event.clone());
                }
            }
        }

        if events.is_empty() {
            return Err(format!(
                "No events found for date {} in modules: {:?}",
                date, event_descriptor
            )
            .into());
        }

        Ok(events)
    }

    fn fetch_events_for_module(event: &Event_Meta, department: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        if event.department.is_empty() || event.module.is_empty() {
            return Err(format!(
                "Department and module must be specified. Got: department='{}', module='{}'",
                event.department, event.module
            ).into());
        }

        // Check if module is valid
        let valid_modules = Self::get_modules_for_department(department, None)?;

        if !valid_modules.contains(&event.module) {
            return Err(format!(
                "Invalid module '{}' for department '{}'",
                event.module, department
            ).into());
        }

        let url = format!("{DATA_URL}/{}/{}.json", &event.department, &event.module);

        let result = reqwest::get(url)?;

        // Handle HTTP errors
        if result.status().is_client_error() || result.status().is_server_error() {
            return Err(result.status().to_string().into());
        }

        Ok(serde_json::from_str(&result.text()?)?)
    }

    /// Fetches Eventdata and stores it in the cache dir
    /// Fetch Event data from git repo (https://github.com/HAWHHCalendarBot/eventfiles.git) and save it locally
    fn fetch_event_data(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Check locally if the data is available
        let eventdata_path = Self::get_eventdata_dir(&cache_dir)?;

        // Create the event data directory if it doesn't exist
        if !eventdata_path.exists() {
            fs::create_dir_all(&eventdata_path)?;
        } else {
            // If the directory already exists, remove it to ensure a fresh clone
            fs::remove_dir_all(&eventdata_path)?;
            fs::create_dir_all(&eventdata_path)?;
        }

        println!("fetching into: {:?}", &eventdata_path);

        // Clone the event data repository
        let output = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/HAWHHCalendarBot/eventfiles.git")
            .arg(&eventdata_path)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clone event data: {}", error_message).into());
        }

        //fetching Event Data that are not in repo (MuP):
        let conf = Config::load_config();
        match conf.get_username() {
            Some(user) => {
                match conf.get_password() {
                    Some(password) => {
                        match  MupLecture::fetch_all_mup_plans_to_cache(user, password){
                            Ok(_) => {println!("Mup Lectures fetched!")},
                            Err(e) => {println!("Cant fetch Mup Lectures, cause: {}", e)}
                        }
                    },
                    None => {println!("found no Password, cant fetch MuP Lectures")},
                    
                };

            },
            None => {println!("found no Username, cant fetch MuP Lectures")}  
        };

        

        // If the clone was successful, return Ok
        println!("Event data cloned successfully.");

        // Refresh Timestamp
        let mut file = File::create(Path::join(&eventdata_path, "./timestamp"))?;
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes())
            .expect("couldnt write timestamp");

        Ok(())
    }
    
    fn get_modules_for_department(department: &str, filter: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
        let cache_dir = Self::get_cache_dir()?;
        let eventdata_path = Self::get_eventdata_dir(&cache_dir)?;

        if !eventdata_path.exists() {
            return Err("Event data directory does not exist".into());
        }

        let department_path = eventdata_path.join(department);
        if !department_path.exists() {
            return Err(format!("Department '{}' does not exist", department).into());
        }

        let mut modules = vec![];
        for entry in fs::read_dir(department_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(module) = entry.file_name().to_str() {
                    if module.ends_with(".json") && !module.starts_with('.') {
                        modules.push(module.trim_end_matches(".json").to_string());
                    }
                }
            }
        }

        if let Some(filter) = filter {
            modules.retain(|module| module.contains(filter));
        }

        if modules.is_empty() {
            return Err(format!("No modules found for department '{}'", department).into());
        }

        Ok(modules)
    }
    
    fn get_departments() -> Result<Vec<String>, Box<dyn Error>> {
        let cache_dir = Self::get_cache_dir()?;
        let eventdata_path = Self::get_eventdata_dir(&cache_dir)?;

        if !eventdata_path.exists() {
            return Err("Event data directory does not exist".into());
        }

        let mut departments = vec![];
        for entry in fs::read_dir(eventdata_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                continue; // Skip files
            }

            
            if let Some(department) = entry.file_name().to_str() {
                if !department.starts_with('.') {
                    departments.push(department.to_string());
                }
            }

             
        }

        if departments.is_empty() {
            return Err("No departments found".into());
        }

        Ok(departments)
    }
}

impl fmt::Display for HawEventEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nLocation: {}\nDescription: {}\nStart: {}\nEnd: {}",
            self.name, self.location, self.description, self.start, self.end
        )
    }
}