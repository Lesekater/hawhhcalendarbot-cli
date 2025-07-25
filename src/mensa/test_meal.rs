use std::{fmt, fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}, process::Command};

use chrono::{Datelike, IsoWeek, NaiveDate, Weekday};
use regex::Regex;
use serde::{Deserialize, Serialize};
use reqwest::blocking as reqwest;

use crate::mensa::meal::{Contents, Meal};

const DATA_URL:&str = "https://raw.githubusercontent.com/testdata/";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TestMeal {
    pub title: String,
    pub description: String,
    pub price: f32,
    pub category: String,
    #[serde(with = "custom_date_format")]
    pub date: NaiveDate,
    pub additives: Vec<String>,
}

mod custom_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%d-%m-%Y";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

impl fmt::Display for TestMeal {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // remove parentheses (if they contain ',') from the name - e.g. "Pizza (o,b,v)" 
        // and single words in parentheses - e.g. "Pizza (o)".
        let re = Regex::new(r"\s*\((?:[^(),]*,[^()]*|\w+)\)\s*").unwrap();
        let filtered_name = re.replace_all(&self.title, "").trim().to_string();

        let formatted_additives = self.additives
            .iter()
            .map(|additive| format!("\"{}\"", additive))
            .collect::<Vec<String>>()
            .join(", ");

        write!(
            f,
            "{}\n{}€ [{}]",
            filtered_name, self.price, formatted_additives
        )
    }
}

impl Meal for TestMeal {
    fn get_contents(&self) -> &Contents {
        todo!()
    }

    fn load_from_local(date: NaiveDate, mensa_name: &str, cache_dir: PathBuf) -> Result<Vec<Self>, std::io::Error> {
        // Read timestamp of local mensa data
        let cache_dir_str = cache_dir.to_str().ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cache directory path is not valid"))?;
        let mut file: File = File::open(format!("{}/mensadata/timestamp", &cache_dir_str))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let timestamp = contents.trim().parse::<i64>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let last_change = chrono::DateTime::from_timestamp(timestamp, 0).unwrap();
        let last_change_date = last_change.date_naive();

        // If the data is older than 1 day, fetch new data
        if chrono::Local::now().date_naive().signed_duration_since(last_change_date) > chrono::Duration::days(1) {
            println!("Local mensa data is outdated. Fetching new data...");
            Self::fetch_mensa_data(&cache_dir)?;
        }

        // Load new data
        let mensadata_path = Self::get_mensadata_dir(&cache_dir)?;

        if !mensadata_path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Local mensa data not available at {:?}", mensadata_path)));
        }

        let iso_date: IsoWeek = date.iso_week();
        let iso_weekday = date.weekday();

        let path_str = format!("./{}/{}/W{}/{}.json",
            &mensa_name,
            &iso_date.year().to_string(),
            &iso_date.week().to_string(),
            &format_weekday(iso_weekday)
        );
        let path = Path::join(&mensadata_path, Path::new(&path_str));

        if !path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("No data found for mensa '{}' on date '{}'", &mensa_name, path_str)));
        }

        // Read data
        let file_content = fs::read_to_string(&path)?;
        
        Ok(serde_json::from_str(&file_content)?)
    }

    fn fetch_data_for_date(date: NaiveDate, mensa_name: &str) -> Result<Vec<Self>, std::io::Error> {
        let iso_date: IsoWeek = date.iso_week();
        let iso_weekday = date.weekday();

        let url = format!("{DATA_URL}/{}/{}/W{}/{}.json",
            &mensa_name,
            &iso_date.year().to_string(),
            &iso_date.week().to_string(),
            &format_weekday(iso_weekday)
        );

        let result = reqwest::get(url)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // Handle HTTP errors
        if result.status().is_client_error() || result.status().is_server_error() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, result.status().to_string()));
        }

        Ok(serde_json::from_str(&result.text().unwrap())?)
    }

    /// Fetches Mensadata and stores it in the cache dir
    /// Fetch Mensa data from git repo (https://github.com/HAWHHCalendarBot/mensa-data.git) and save it locally
    fn fetch_mensa_data(cache_dir: &PathBuf) -> Result<(), std::io::Error> {
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

        // Clone the mensa data repository
        let output = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/testdata.git")
            .arg(&mensadata_path)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to clone mensa data: {}", error_message)));
        }

        // If the clone was successful, return Ok
        println!("Mensa data cloned successfully.");

        // Refresh Timestamp
        let mut file = File::create(Path::join(&mensadata_path, "./timestamp"))?;
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

        Ok(())
    }
}

fn format_weekday(day: Weekday) -> String {
    match day {
        Weekday::Mon => "MO".to_string(),
        Weekday::Tue => "DI".to_string(),
        Weekday::Wed => "MI".to_string(),
        Weekday::Thu => "DO".to_string(),
        Weekday::Fri => "FR".to_string(),
        Weekday::Sat => "SA".to_string(),
        Weekday::Sun => "SO".to_string(),
    }
}