use std::{collections::BTreeMap, error::Error, fmt, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}, process::Command};

use chrono::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use reqwest::blocking as reqwest;

use crate::{json_parser::Occupations, mensa::meal::{Contents, Meal, Prices}};

const DATA_URL:&str = "https://raw.githubusercontent.com/HAWHHCalendarBot/mensa-data/main";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct HawMeal {
    pub name: String,
    pub category: String,
    pub date: NaiveDate,
    pub additives: BTreeMap<String, String>,

    #[serde(flatten)]
    pub prices: Prices,

    #[serde(flatten)]
    pub contents: Contents,
}

impl fmt::Display for HawMeal {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // remove parentheses (if they contain ',') from the name - e.g. "Pizza (o,b,v)" 
        // and single words in parentheses - e.g. "Pizza (o)".
        let re = Regex::new(r"\s*\((?:[^(),]*,[^()]*|\w+)\)\s*").unwrap();
        let filtered_name = re.replace_all(&self.name, "").trim().to_string();

        let config = crate::json_parser::Config::load_config();
        let price = match config.get_occupation() {
            Some(Occupations::Student) => self.prices.price_student,
            Some(Occupations::Employee) => self.prices.price_attendant,
            Some(Occupations::Guest) => self.prices.price_guest,
            _ => self.prices.price_student, // Default to student price if occupation is unknown
        };

        write!(
            f,
            "{}\n{}€ [{}]",
            filtered_name, price, self.contents
        )
    }
}

impl Meal for HawMeal {
    fn get_contents(&self) -> &Contents {
        &self.contents
    }

    fn load_from_local(date: NaiveDate, mensa_name: &str, cache_dir: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        // Read timestamp of local mensa data
        let cache_dir_str = cache_dir.to_str().ok_or("Cache directory path is not valid")?;
        let mut file: File = File::open(format!("{}/mensadata/timestamp", &cache_dir_str))?;
        let mut contents = String::new();
        
        file.read_to_string(&mut contents)?;
        let last_change = chrono::DateTime::from_timestamp(contents.parse()?, 0).unwrap();
        let last_change_date = last_change.date_naive();
        
        // If the data is older than 1 day, fetch new data
        if chrono::Local::now().date_naive().signed_duration_since(last_change_date) > chrono::Duration::days(1) {
            println!("Local mensa data is outdated. Fetching new data...");
            Self::fetch_mensa_data(&cache_dir)?;
        }

        // Load new data
        let mensadata_path = Self::get_mensadata_dir(&cache_dir)?;

        if !mensadata_path.exists() {
            return Err(format!("Local mensa data not available at {:?}", mensadata_path).into());
        }
        
        let path_str = format!("./{}/{}/{}/{}.json",
            &mensa_name,
            &date.format("%Y").to_string(),
            &date.format("%m").to_string(),
            &date.format("%d").to_string()
        );
        let path = Path::join(&mensadata_path, Path::new(&path_str));

        if !path.exists() {
            return Err(format!("No data found for mensa '{}' on date '{}'", &mensa_name, date).into());
        }

        // Read data
        let file_content = fs::read_to_string(&path)?;
        
        Ok(serde_json::from_str(&file_content)?)
    }

    fn fetch_data_for_date(date: NaiveDate, mensa_name: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let url = format!("{DATA_URL}/{}/{}/{}/{}.json",
            &mensa_name,
            &date.format("%Y"),
            &date.format("%m"),
            &date.format("%d")
        );

        let result = reqwest::get(url)?;

        // Handle HTTP errors
        if result.status().is_client_error() || result.status().is_server_error() {
            return Err(result.status().to_string().into());
        }

        Ok(serde_json::from_str(&result.text()?)?)
    }

    /// Fetches Mensadata and stores it in the cache dir
    /// Fetch Mensa data from git repo (https://github.com/HAWHHCalendarBot/mensa-data.git) and save it locally
    fn fetch_mensa_data(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Check locally if the data is available
        let mensadata_path = Self::get_mensadata_dir(&cache_dir)?;

        // Create the mensa data directory if it doesn't exist
        if !mensadata_path.exists() {
            fs::create_dir_all(&mensadata_path)?;
        } else {
            // If the directory already exists, remove it to ensure a fresh clone
            fs::remove_dir_all(&mensadata_path)?;
            fs::create_dir_all(&mensadata_path)?;
        }

        println!("fetching into: {:?}", &mensadata_path);

        // Clone the mensa data repository
        let output = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/HAWHHCalendarBot/mensa-data.git")
            .arg(&mensadata_path)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clone mensa data: {}", error_message).into());
        }
        
        // If the clone was successful, return Ok
        println!("Mensa data cloned successfully.");

        // Refresh Timestamp
        let mut file = File::create(Path::join(&mensadata_path, "./timestamp"))?;
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

        Ok(())
    }
}