use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use chrono::NaiveDate;
use regex::Regex;
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};

use crate::{
    json_parser::Occupations,
    //mensa::meal::{Contents, Meal, Prices},
};

const DATA_URL: &str =
    "https://raw.githubusercontent.com/HAWHHCalendarBot/eventfiles/refs/heads/main/"; // /faculty/event.json

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventEntry {
    pub name: String,
    pub location: String,
    pub description: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct EventEntryV4 {
    pub name: String,
    pub location: String,
    pub description: String,
    #[serde(serialize_with = "serialize_date_time")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize_date_time")]
    pub end_time: NaiveDateTime,
}

fn serialize_date_time<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&Berlin.from_local_datetime(dt).unwrap().to_rfc3339())
}

impl From<EventEntry> for EventEntryV4 {
    fn from(value: EventEntry) -> Self {
        Self {
            name: value.name,
            location: value.location,
            description: value.description,
            start_time: value.start,
            end_time: value.end,
        }
    }
}

impl Event for EventEntry {
    fn get_contents(&self) -> &Contents {
        &self.contents
    }

    fn load_from_local(
        department: &str,
        module: &str,
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

        if !eventdata_path.exists() {
            return Err(format!("Local event data not available at {:?}", eventdata_path).into());
        }

        let path_str = format!("./{}/{}.json", &department, &module,);

        let path = Path::join(&eventdata_path, Path::new(&path_str));

        if !path.exists() {
            return Err(format!(
                "No data found for module '{}' in department '{}'",
                &modue, &department
            )
            .into());
        }

        // Read data
        let file_content = fs::read_to_string(&path)?;

        Ok(serde_json::from_str(&file_content)?)
    }

    fn fetch_event_for_module(department: &str, module: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let url = format!("{DATA_URL}/{}/{}.json", &department, &module);

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

        print!("fetching into: {:?}", &eventdata_path);

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

        // If the clone was successful, return Ok
        println!("Event data cloned successfully.");

        // Refresh Timestamp
        let mut file = File::create(Path::join(&eventdata_path, "./timestamp"))?;
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes())
            .expect("couldnt write timestamp");

        Ok(())
    }
}
