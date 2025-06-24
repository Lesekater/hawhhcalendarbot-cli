use std::error::Error;
use std::path::PathBuf;

use chrono::NaiveDate;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

use crate::json_parser::Extras;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Meta {
    pub department: String,
    pub nodule: String,
}

pub trait Event: Sized {
    //// Fetching data ////

    fn get_event_for_module(department: &str, module: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        // Check if the mensa data is available locally
        // -> if so, load it
        // -> else load for single date directly
        let cache_dir = Self::get_cache_dir()?;
        Self::load_from_local(department, module, cache_dir)
            .or_else(|_| Self::fetch_event_for_module(department, module))
    }

    /// Local loading ///

    fn load_from_local(
        department: &str,
        module: &str,
        cache_dir: PathBuf,
    ) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;

    //// Fetching remote data ////

    /// Fetches data for a single module
    fn fetch_event_for_module(
        date: chrono::NaiveDate,
        mensa_name: &str,
    ) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;

    /// Fetches Mensadata and stores it in the cache dir
    fn fetch_event_data(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>>;

    /// UTIL ///

    fn get_cache_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
        let cache_dir = cache_dir().ok_or("Could not find cache directory")?;
        Ok(cache_dir.join(env!("CARGO_PKG_NAME")))
    }

    fn get_mensadata_dir(cache_dir: &PathBuf) -> Result<std::path::PathBuf, Box<dyn Error>> {
        let eventdata_path = cache_dir.join("eventdata");

        if !eventdata_path.exists() {
            fs::create_dir_all(&eventdata_path).expect("Could not create event data directory");
        }

        Ok(mensadata_path)
    }
}
