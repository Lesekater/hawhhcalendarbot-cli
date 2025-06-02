use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

//enum of possibel extras to choose from:
#[derive(Serialize, Deserialize)]
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

pub fn new_config() -> Config {}

pub fn load_config() -> Config{
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

pub fn set_occupation(occupation: Occupations) {}

pub fn set_extras(extras: Vec<Extras>) {}

pub fn set_primary_mensa() {}
