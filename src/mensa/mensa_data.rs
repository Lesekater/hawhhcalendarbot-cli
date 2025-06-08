use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use dirs::cache_dir;
use reqwest::blocking as reqwest;

use crate::mensa::meal::Meal;

///////////////////////////////////////////////////////////////////////////
///////////                 Local Loading
///////////////////////////////////////////////////////////////////////////

pub fn load_local_data(date: chrono::NaiveDate, mensa_name: &str, cache_dir: PathBuf) -> Result<Vec<Meal>, Box<dyn Error>> {
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
        fetch_mensa_data(&cache_dir)?;
    }

    // Load new data
    let mensadata_path = get_mensadata_dir(&cache_dir)?;

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

pub fn get_food_for_date(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Meal>, Box<dyn Error>> {

    // Check if the mensa data is available locally
    // -> if so, load it
    // -> else load for single date directly
    let cache_dir = get_cache_dir()?;
    load_local_data(date, mensa_name, cache_dir).or_else(|_| fetch_data_for_date(date, mensa_name))
}

///////////////////////////////////////////////////////////////////////////
///////////                 Fetching Mensa Data   
///////////////////////////////////////////////////////////////////////////

/// Fetches data for a single date
pub fn fetch_data_for_date(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Meal>, Box<dyn Error>> {
    let url = format!("https://raw.githubusercontent.com/HAWHHCalendarBot/mensa-data/refs/heads/main/{}/{}/{}/{}.json",
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
pub fn fetch_mensa_data(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Check locally if the data is available
    let mensadata_path = get_mensadata_dir(&cache_dir)?;

    // Create the mensa data directory if it doesn't exist
    if !mensadata_path.exists() {
        fs::create_dir_all(&mensadata_path)?;
    } else {
        // If the directory already exists, remove it to ensure a fresh clone
        fs::remove_dir_all(&mensadata_path)?;
        fs::create_dir_all(&mensadata_path)?;
    }

    print!("fetching into: {:?}", &mensadata_path);

    // Clone the mensa data repository
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("https://github.com/HAWHHCalendarBot/mensa-data.git")
        .arg(&mensadata_path)
        .output()?;        

    if output.status.success() {
        // If the clone was successful, return Ok
        println!("Mensa data cloned successfully.");

        // Refresh Timestamp
        let mut file = File::create(Path::join(&mensadata_path, "./timestamp"))?;
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to clone mensa data: {}", error_message).into());
    }
}

///////////////////////////////////////////////////////////////////////////
///////////                      Util
///////////////////////////////////////////////////////////////////////////

fn get_cache_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let cache_dir = cache_dir().ok_or("Could not find cache directory")?;
    Ok(cache_dir.join(env!("CARGO_PKG_NAME")))
}

fn get_mensadata_dir(cache_dir: &PathBuf) -> Result<std::path::PathBuf, Box<dyn Error>> {
    let mensadata_path = cache_dir.join("mensadata");

    if !mensadata_path.exists() {
        fs::create_dir_all(&mensadata_path).expect("Could not create mensa data directory");
    }

    Ok(mensadata_path)
}