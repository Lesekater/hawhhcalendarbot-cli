use std::error::Error;
use std::path::PathBuf;

use chrono::NaiveDate;
use dirs::cache_dir;
use std::fs;

/// Event describing a module within a department.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct EventMeta {
    /// The department name.
    pub department: String,
    /// The module name.
    pub module: String,
}

/// Trait for event data sources, providing methods for loading, fetching, and caching event data.
pub trait Event: Sized {

    /// Return all events configured in config for a given date.
    fn get_all_events_for_date(date: NaiveDate) -> Result<Vec<Self>, Box<dyn Error>>;

    /// Returns all events for a module in a given Department.
    /// Attempts to load from local cache, falling back to remote fetch if unavailable.
    fn get_events_for_module(event: &EventMeta) -> Result<Vec<Self>, Box<dyn Error>> {
        let cache_dir = Self::get_cache_dir()?;
        Self::load_from_local(event, cache_dir).or_else(|_| Self::fetch_events_for_module(event, &event.department))
    }

    /// Returns all events for the given descriptors on a specific date.
    fn get_events_for_date(
        event_descriptor: Vec<EventMeta>,
        date: NaiveDate,
    ) -> Result<Vec<Self>, Box<dyn Error>>;

    /// List all possible modules for a given department.
    fn get_modules_for_department(department: &str, filter: Option<&str>) -> Result<Vec<String>, Box<dyn Error>>;

    /// List all departments that have events.
    fn get_departments() -> Result<Vec<String>, Box<dyn Error>>;

    /// Loads events for a module from the local cache directory.
    fn load_from_local(event: &EventMeta, cache_dir: PathBuf) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;

    /// Fetches events for a single module from a remote source.
    fn fetch_events_for_module(event: &EventMeta, department: &str) -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;

    /// Fetches all event data and stores it in the cache directory.
    fn fetch_event_data(cache_dir: &PathBuf) -> Result<(), Box<dyn Error>>;

    /// Returns the cache directory path for event data.
    fn get_cache_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
        let cache_dir = cache_dir().ok_or("Could not find cache directory")?;
        Ok(cache_dir.join(env!("CARGO_PKG_NAME")))
    }

    /// Returns the directory path for event data within the cache.
    /// Creates the directory if it does not exist.
    fn get_eventdata_dir(cache_dir: &PathBuf) -> Result<std::path::PathBuf, Box<dyn Error>> {
        let eventdata_path = cache_dir.join("eventdata");

        if !eventdata_path.exists() {
            fs::create_dir_all(&eventdata_path).expect("Could not create event data directory");
        }

        Ok(eventdata_path)
    }
}
