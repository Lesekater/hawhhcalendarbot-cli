use std::error::Error;
use std::path::PathBuf;

use chrono::NaiveDate;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

use crate::json_parser::Extras;

/// Event describing a module within a department.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Event_Meta {
    /// The department name.
    pub department: String,
    /// The module name.
    pub module: String,
}

/// Trait for event data sources, providing methods for loading, fetching, and caching event data.
pub trait Event {
    /// Return all events configured in config for a given date.
    fn get_all_events_for_date(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<Box<dyn Event>>, Box<dyn Error>>;

    /// Returns all events for a module in a given Department.
    /// Attempts to load from local cache, falling back to remote fetch if unavailable.
    fn get_events_for_module(
        &self,
        event: &Event_Meta,
    ) -> Result<Vec<Box<dyn Event>>, Box<dyn Error>> {
        let cache_dir = self.get_cache_dir()?;
        self.load_from_local(event, cache_dir)
            .or_else(|_| self.fetch_events_for_module(event, &event.department))
    }

    /// Returns all events for the given descriptors on a specific date.
    fn get_events_for_date(
        &self,
        event_descriptor: Vec<Event_Meta>,
        date: NaiveDate,
    ) -> Result<Vec<Box<dyn Event>>, Box<dyn Error>>;

    /// List all possible modules for a given department.
    fn get_modules_for_department(
        &self,
        department: &str,
        filter: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>>;

    /// List all departments that have events.
    fn get_departments(&self) -> Result<Vec<String>, Box<dyn Error>>;

    /// Loads events for a module from the local cache directory.
    fn load_from_local(
        &self,
        event: &Event_Meta,
        cache_dir: PathBuf,
    ) -> Result<Vec<Box<dyn Event>>, Box<dyn Error>>;

    /// Fetches events for a single module from a remote source.
    fn fetch_events_for_module(
        &self,
        event: &Event_Meta,
        department: &str,
    ) -> Result<Vec<Box<dyn Event>>, Box<dyn Error>>;

    /// Fetches all event data and stores it in the cache directory.
    fn fetch_event_data(&self, cache_dir: &PathBuf) -> Result<(), Box<dyn Error>>;

    /// Returns the cache directory path for event data.
    fn get_cache_dir(&self) -> Result<std::path::PathBuf, Box<dyn Error>> {
        let cache_dir = cache_dir().ok_or("Could not find cache directory")?;
        Ok(cache_dir.join(env!("CARGO_PKG_NAME")))
    }

    /// Returns the directory path for event data within the cache.
    /// Creates the directory if it does not exist.
    fn get_eventdata_dir(&self, cache_dir: &PathBuf) -> Result<std::path::PathBuf, Box<dyn Error>> {
        let eventdata_path = cache_dir.join("eventdata");

        if !eventdata_path.exists() {
            fs::create_dir_all(&eventdata_path).expect("Could not create event data directory");
        }

        Ok(eventdata_path)
    }
}
