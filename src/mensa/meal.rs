use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::mpsc;
use std::{thread, thread::JoinHandle};

use chrono::NaiveDate;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

use crate::json_parser::Extras;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Meta {
    pub canteen: String,
    pub date: NaiveDate,
}

pub trait Meal: Sized + Debug {

    //// Getter ////
    fn get_contents(&self) -> &Contents;

    //// Fetching data ////

    fn get_food_for_date(date: NaiveDate, mensa_name: &str) -> Result<Vec<Self>, std::io::Error> {
        // Check if the mensa data is available locally
        // -> if so, load it
        // -> else load for single date directly
        let cache_dir = Self::get_cache_dir()?;
        Self::load_from_local(date, mensa_name, cache_dir).or_else(|_| Self::fetch_data_for_date(date, mensa_name))
    }

    /// Local loading ///
    
    fn load_from_local(date: chrono::NaiveDate, mensa_name: &str, cache_dir: PathBuf) -> Result<Vec<Self>, std::io::Error> where Self: Sized;

    //// Fetching remote data ////
    
    /// Fetches data for a single date
    fn fetch_data_for_date(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Self>, std::io::Error> where Self: Sized;

    fn update_mensa_data() -> JoinHandle<Result<(), std::io::Error>> {
        thread::spawn(|| {
            let cache_dir = Self::get_cache_dir()?;
            let mensadata_path = Self::get_mensadata_dir(&cache_dir)?;
            
            // Check if timestamp file exists & is older than 1 day
            let timestamp_path = mensadata_path.join("timestamp");

            if !timestamp_path.exists() {
                // If timestamp file does not exist, fetch new data
                Self::fetch_mensa_data(&cache_dir)?;
            }

            let mut file:PathBuf = timestamp_path;
            let contents = fs::read_to_string(&mut file)?;
            let last_change = chrono::DateTime::from_timestamp(contents.parse().unwrap(), 0).unwrap();
            let last_change_date = last_change.date_naive();

            if chrono::Local::now().date_naive().signed_duration_since(last_change_date) <= chrono::Duration::days(1) {
                return Ok(()); // Data is up-to-date
            }
            
            Self::fetch_mensa_data(&cache_dir)?;

            Ok(())
        })
    }

    /// Fetches Mensadata and stores it in the cache dir
    fn fetch_mensa_data(cache_dir: &PathBuf) -> Result<(), std::io::Error>;

    /// UTIL ///
    
    fn get_cache_dir() -> Result<std::path::PathBuf, std::io::Error> {
        let cache_dir = cache_dir().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find cache directory"))?;
        Ok(cache_dir.join(env!("CARGO_PKG_NAME")))
    }

    fn get_mensadata_dir(cache_dir: &PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
        let mensadata_path = cache_dir.join("mensadata");

        if !mensadata_path.exists() {
            fs::create_dir_all(&mensadata_path).expect("Could not create mensa data directory");
        }

        Ok(mensadata_path)
    }

    fn filter_food_by_extras(
        foods: Vec<Self>,
        extras: &Vec<Extras>,
    ) -> Vec<Self> where Self: Sized + Send + 'static {

        if extras.is_empty() {
            return foods;
        }

        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        // +1 because the last receiver collects the filtered food
        for _ in 0..extras.len() + 1 {
            let (tx, rx) = mpsc::channel::<Self>();
            senders.push(tx);
            receivers.push(rx);
        }

        // Spawn filter threads
        let mut handles = Vec::new();
        for filter in extras.iter().cloned() {

            let rx= receivers.remove(0);
            let tx = senders.remove(1);

            // Filter threads
            handles.push(thread::spawn(move || {
                // println!("Thread {} for extra {:?}", i, filter);
                for food in rx {
                    if Self::filter_food_by_extra(&food, &filter) {
                        // println!("Thread {} for extra {:?} sending food: {:?}", i, filter, food.get_title());
                        let _ = tx.send(food);
                    }
                }
            }));
        }

        // Send Food into filter chain
        let sender = senders.remove(0);
        for food in foods {
            let _ = sender.send(food);
        }

        // drop sender to close the channel
        drop(sender);

        // Collect food
        let filtered: Vec<Self> = receivers[0].iter().map(|food| {
            // println!("Collecting food: {:?}", food.get_title());
            food
        }).collect();

        // Wait for threads to finish
        for handle in handles {
            let _ = handle.join();
        }

        filtered
    }

    fn filter_food_by_extra(
        food: &Self,
        extra: &Extras,
    ) -> bool {
        // Check if the food item has any of the specified extras
        let contains = food.get_contents().to_string().contains(&extra.as_str());
        match extra {
            // POSITIVE EXTRAS
            Extras::Vegan | Extras::Vegetarian | Extras::LactoseFree | Extras::Alcohol => {
                if !contains {
                    return false; // If the food does not contain the extra, skip it
                }
            }
            // NEGATIVE EXTRAS
            _ => {
                if contains {
                    return false; // If the food contains a negative extra, skip it
                }
            }
        }

        true // Keep the food item if it passes all checks
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Contents {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")] // only serialize if true
    pub alcohol: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub beef: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fish: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub game: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub gelatine: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub lactose_free: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub lamb: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub pig: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub poultry: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub vegan: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub vegetarian: bool,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct Prices {
    pub price_attendant: f32,
    pub price_guest: f32,
    pub price_student: f32,
}

impl Clone for Prices {
    fn clone(&self) -> Self {
        Self {
            price_attendant: self.price_attendant,
            price_guest: self.price_guest,
            price_student: self.price_student,
        }
    }
}

impl Clone for Contents {
    fn clone(&self) -> Self {
        Self {
            alcohol: self.alcohol,
            beef: self.beef,
            fish: self.fish,
            game: self.game,
            gelatine: self.gelatine,
            lactose_free: self.lactose_free,
            lamb: self.lamb,
            pig: self.pig,
            poultry: self.poultry,
            vegan: self.vegan,
            vegetarian: self.vegetarian,
        }
    }
}

impl std::fmt::Debug for Contents {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Contents { ")?;
        if self.alcohol {
            fmt.write_str("Alcohol ")?;
        }
        if self.beef {
            fmt.write_str("Beef ")?;
        }
        if self.fish {
            fmt.write_str("Fish ")?;
        }
        if self.game {
            fmt.write_str("Game ")?;
        }
        if self.gelatine {
            fmt.write_str("Gelatine ")?;
        }
        if self.lactose_free {
            fmt.write_str("LactoseFree ")?;
        }
        if self.lamb {
            fmt.write_str("Lamb ")?;
        }
        if self.pig {
            fmt.write_str("Pig ")?;
        }
        if self.poultry {
            fmt.write_str("Poultry ")?;
        }
        if self.vegan {
            fmt.write_str("Vegan ")?;
        }
        if self.vegetarian {
            fmt.write_str("Vegetarian ")?;
        }
        fmt.write_str("}")
    }
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut contents = Vec::new();
        if self.alcohol {
            contents.push("Alcohol");
        }
        if self.beef {
            contents.push("Beef");
        }
        if self.fish {
            contents.push("Fish");
        }
        if self.game {
            contents.push("Game");
        }
        if self.gelatine {
            contents.push("Gelatine");
        }
        if self.lactose_free {
            contents.push("Lactose Free");
        }
        if self.lamb {
            contents.push("Lamb");
        }
        if self.pig {
            contents.push("Pig");
        }
        if self.poultry {
            contents.push("Poultry");
        }
        if self.vegan {
            contents.push("Vegan");
        }
        if self.vegetarian {
            contents.push("Vegetarian");
        }

        write!(f, "{}", contents.join(", "))
    }
}
