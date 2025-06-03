use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::error::Error;

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
enum Occupations {
    Student,
    Employee,
    Guest,
}

//Struct of the Cofig to load form the JSON File
#[derive(Serialize, Deserialize)]
struct Config {
    primary_mensa: Option<String>,
    mensa_list: Option<Vec<String>>,
    occupation: Option<Occupations>,
    extras: Option<Vec<Extras>>,
}


//Funktionen für Custom Json Parser:
fn json_to_struct(load_path: String) -> Result<Config, Box<dyn Error>>{}

fn struct_to_json(save_path: String, config: Config) -> Result<()> {}


//Funktionen für das Config Managment:
pub fn new_config() -> Config {


}

pub fn load_config() -> Config {
    let json_config = fs::read_to_string("../data/user_config.json").expect("");

    serde_json::from_str(&json_config).expect("Fehler beim Parsen der JSON")
}

pub fn save_config_json(user_config: &Config) {
    let json_string = serde_json::to_string_pretty(user_config).expect("Fehler beim Serialisieren");
    fs::write("config.json", json_string).expect("Fehler beim Schreiben der Datei");
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
