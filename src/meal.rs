use std::{collections::BTreeMap};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use regex::Regex;

use crate::config_managment::Occupations;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Meta {
    pub canteen: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Meal {
    pub name: String,
    pub category: String,
    pub date: NaiveDate,
    pub additives: BTreeMap<String, String>,

    #[serde(flatten)]
    pub prices: Prices,

    #[serde(flatten)]
    pub contents: Contents,
}

impl fmt::Display for Meal {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // remove parentheses (if they contain ',') from the name - e.g. "Pizza (o,b,v)" 
        // and single words in parentheses - e.g. "Pizza (o)".
        let re = Regex::new(r"\s*\((?:[^(),]*,[^()]*|\w+)\)\s*").unwrap();
        let filtered_name = re.replace_all(&self.name, "").trim().to_string();

        let config = crate::config_managment::load_config();
        let price = match config.occupation() {
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