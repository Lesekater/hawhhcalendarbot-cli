use clap::builder::Str;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use getset::{Getters};

//enum of possibel extras to choose from:
#[derive(Serialize, Deserialize, PartialEq)]
enum Extras {
    Vegan,
    Vegetarisch,
    LactoseFree,
    AlcoholFree,
    BeefFree,
    FishFree,
}

//enum of possibel occupations
#[derive(Serialize, Deserialize)]
pub enum Occupations {
    Student,
    Employee,
    Guest,
}

//Struct of the Cofig to load form the JSON File
#[derive(Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Config {
    primary_mensa: Option<String>,
    mensa_list: Option<Vec<String>>,
    occupation: Option<Occupations>,
    extras: Option<Vec<Extras>>,
}

impl Config {
    fn new() -> Config {
        Self {
            primary_mensa: None,
            mensa_list: Some(Vec::new()),
            occupation: None,
            extras: Some(Vec::new()),
        }
    }
}

//Funktionen für Custom Json Parser:
fn json_to_struct(load_path: String) /* -> Result<Config, Box<dyn Error>> */
{
    let json_file = fs::read_to_string(load_path);
    //print!("{:?}", json_file);
    string_seperation(json_file.unwrap());
}

fn struct_to_json(save_path: String, config: Config) /*  -> Result<()> */ {}

fn string_seperation(input_string: String) {
    //let chars_to_trimm: &char = &[' '];
    let trimmed_str: Vec<&str> = input_string.split(' ').collect();

    let mut clean_str: Vec<&str> = Vec::new();

    

    println!("{:?}", trimmed_str);

    //print!("{:?}\n", trimmed_str);
    //let primary_mensa = trimmed_str.trim_start()
}

//Funktionen für das Config Managment:
pub fn new_config() -> Config {
    let extra_vec = vec![Extras::Vegan, Extras::LactoseFree];
    let config = Config {
        primary_mensa: Some("Berliner Tor".to_string()),
        mensa_list: None,
        occupation: Some(Occupations::Student),
        extras: Some(extra_vec),
    };

    save_config_json(&config);

    config
}

pub fn load_config() -> Config {
    match fs::read_to_string(
        dirs::config_local_dir()
            .unwrap()
            .join("hawhhcalendarbot/cfg.json"),
    ) {
        Ok(json_config) => serde_json::from_str(&json_config).expect("Fehler beim Parsen der JSON"),
        Err(_) => Config::new(),
    }
}

pub fn save_config_json(user_config: &Config) {
    let conf_dir = dirs::config_local_dir().unwrap().join("hawhhcalendarbot");

    let json_string = serde_json::to_string_pretty(user_config).expect("Fehler beim Serialisieren");
    let _ = fs::create_dir_all(conf_dir);

    fs::write(
        dirs::config_local_dir()
            .unwrap()
            .join("hawhhcalendarbot/cfg.json"),
        json_string,
    )
    .expect("Fehler beim Schreiben der Datei");
}

pub fn add_mensa(mensa: String) {
    let mut user_config = load_config();

    //Prüfen ob ein Vec Vorhanden ist
    if let Some(list) = user_config.mensa_list.as_mut() {
        list.push(mensa);

    //keine Vec vorhanden, also neue erstellen:
    } else {
        let mensa_list_new: Option<Vec<String>> = Some(Vec::new());
        user_config.mensa_list = mensa_list_new;
    }

    save_config_json(&user_config);
}

pub fn set_occupation(occupation: Occupations) {
    let mut conf = load_config();
    conf.occupation = Some(occupation);
    save_config_json(&conf);
}

pub fn set_extras(extras: Vec<Extras>) {
    let mut conf = load_config();
    let mut conf_ex = conf.extras.unwrap_or(Vec::new());

    for ex in extras {
        if conf_ex.contains(&ex) {
            conf_ex.push(ex);
        }
    }

    conf.extras = Some(conf_ex);
    save_config_json(&conf);
}

pub fn set_primary_mensa(primary_mensa: String) {
    let mut conf = load_config();
    conf.primary_mensa = Some(primary_mensa);
    save_config_json(&conf);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn create_config() {
        let user_config = new_config();
        json_to_struct("./data/user_config.json".to_string());

        let extra_vec = vec![Extras::Vegan, Extras::LactoseFree];
        set_extras(extra_vec);
        add_mensa("mensa".to_string());
        add_mensa("mensa2".to_string());
        add_mensa("3".to_string());
    }

    #[test]
    fn test_new_config() {
        let conf = Config::new();
        save_config_json(&conf);
    }
}
