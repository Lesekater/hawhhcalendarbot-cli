use std::{fmt::{format, Error}, fs::{self, File}, vec};

#[derive(Debug)]
pub(crate) enum Occupations {
    Student,
    Employee,
    Guest,
}

#[derive(Debug)]
pub(crate) enum Extras {
    Vegan,
    Vegetarisch,
    LactoseFree,
    AlcoholFree,
    BeefFree,
    FishFree,
    GelatineFree,
    LambFree,
    PigFree,
    PoultryFree,
    Other(String),
}

#[derive(Debug)]
pub(crate) enum ConfigName {
    primary_mensa,
    mensa_list,
    occupation,
    extras,
}


#[derive(Debug)]
pub struct Config {
    primary_mensa: String,
    mensa_list: Option<Vec<String>>,
    occupation: Option<Occupations>,
    extras: Option<Vec<Extras>>,
}

/*
    Implementierung des Json Parser:
*/

impl Config {

//Config:
    fn new() -> Self{
        Self {
            primary_mensa: String::new(),
            mensa_list: Some(vec![String::new()]),
            occupation: Some(Occupations::Employee),
            extras: Some(vec![Extras::Vegan]),
        }

    }

    
    pub fn update_primary_mensa(&mut self, mensa_to_add: String) {
        self.primary_mensa = mensa_to_add;
    }

    pub fn update_mensa_list(&mut self, mensa_to_add: String) {

        let mensa_list = self.mensa_list.as_mut().unwrap();
        mensa_list.push(mensa_to_add);
    }

    pub fn get_mensa_list(&mut self) -> &Vec<String>{

        self.mensa_list.as_ref().unwrap()
    }

    pub fn remove_mensa(&mut self, mensa_to_remove: String) {
        if let Some(mensa_list) = self.mensa_list.as_mut() {
            mensa_list.retain(|e| e.as_str() != mensa_to_remove.as_str());
        }
    }

    pub fn update_occupation(&mut self, new_occ: Occupations) {
        self.occupation = Some(new_occ);
    }

    pub fn add_extra(&mut self, extra_to_add: Extras) {
        let extra_list = self.extras.as_mut().unwrap();
        extra_list.push(extra_to_add);
    }

    pub fn remove_extra(&mut self, extra_to_remove: Extras) {
    if let Some(extra_list) = self.extras.as_mut() {
        extra_list.retain(|e| e.as_str() != extra_to_remove.as_str());
        }
    }

    pub fn load_config() -> Config {
        match fs::read_to_string(
            dirs::config_local_dir()
                .unwrap()
                .join("hawhhcalendarbot/cfg.json"),
        ) {
            Ok(json_config) => Config::struct_from_json_file(&json_config).expect("Fehler beim Parsen der JSON"),
            Err(_) => Config::new(),
        }
    }

    pub fn save_config_json(user_config: &Config) {
        let conf_dir = dirs::config_local_dir().unwrap().join("hawhhcalendarbot");

        let json_string = json_file_from_struct(user_config).expect("Fehler beim Serialisieren");
        let _ = fs::create_dir_all(conf_dir);

        fs::write(
            dirs::config_local_dir()
                .unwrap()
                .join("hawhhcalendarbot/cfg.json"),
        json_string,
        )
        .expect("Fehler beim Schreiben der Datei");
    }



//Json Parser
    fn struct_from_json_file(/*path: &str*/ json_config: &String) -> Result<Config, Box<dyn std::error::Error>> {

        //let mut config = Config::new();

        //let config_content_raw = fs::read_to_string(path)?;
        let config_content_raw = json_config;


        //Whitespace, Leerzeichen und Zeilenumbrüche entfernen
        let config_content_cleaned =  config_content_raw
                                            .chars()
                                            .filter(|c| !c.is_whitespace()) 
                                            .collect::<String>();
        
        //End index, der gesucht berechnen:
        let pm_end = config_content_cleaned.find(ConfigName::primary_mensa.as_str()).unwrap() + ConfigName::primary_mensa.as_str().len();
        let ml_end = config_content_cleaned.find(ConfigName::mensa_list.as_str()).unwrap() + ConfigName::mensa_list.as_str().len();
        let op_end = config_content_cleaned.find(ConfigName::occupation.as_str()).unwrap() + ConfigName::occupation.as_str().len();
        let et_end = config_content_cleaned.find(ConfigName::extras.as_str()).unwrap() + ConfigName::extras.as_str().len();

        //Inhalt der primary mensa extrahieren:

        let primary_mensa= config_content_cleaned[pm_end+3..pm_end+3 + config_content_cleaned[pm_end+3..].find('"').unwrap()].to_string();

        //Inhalt der mensa list extrahieren:

        let mensa_list_all =  &config_content_cleaned[ml_end+3..ml_end+3 + config_content_cleaned[ml_end+3..].find(']').unwrap()];

        let  mensa_list: Vec<String> = mensa_list_all
                                            .chars()
                                            .filter(|&c| c != '"')
                                            .collect::<String>()
                                            .split(',')
                                            .map(|s| s.to_string())
                                            .collect();

        //Inhalt der Occupation extrahieren:

        let occupations_string= &config_content_cleaned[op_end+3..op_end+3 + config_content_cleaned[op_end+3..].find('"').unwrap()].to_string();
        let occupations = Occupations::from_str(occupations_string);

        //Inhalt der Extras extrahieren:
        let extra_list_all =  &config_content_cleaned[et_end+3..et_end+3 + config_content_cleaned[et_end+3..].find(']').unwrap()];

        let extra_list: Vec<Extras> = extra_list_all
                                            .chars()
                                            .filter(|&c| c != '"')
                                            .collect::<String>()
                                            .split(',')
                                            .map(|s| s.to_string())
                                            .map(|e| Extras::from_str(&e))
                                            .collect();

        //Config zurückkgeben:
        Ok(Config { primary_mensa: primary_mensa,
                    mensa_list: Some(mensa_list),
                    occupation: (occupations),
                    extras: Some(extra_list) })

    }

    fn json_file_from_struct(config: &Config) -> Result<String, Box<dyn std::error::Error>>  {

        let primary_mensa = format!("{:?}", config.primary_mensa);

        let mensa_list:String = config.mensa_list
                                                        .iter()
                                                        .map(|s|  format!("{:?}", s))
                                                        .collect::<Vec<String>>()            // in Vec sammeln
                                                        .join(", ");  
        //mensa_list = format!("[{}]", mensa_list);                                    
    
        let occupations = format!("\"{}\"", config.occupation.as_ref().unwrap().as_str());

        let mut extra_list: String = config.extras.as_ref().unwrap()
                                                        .iter()
                                                        .map(|e| format!("\"{}\"", e.as_str()))
                                                        .collect::<Vec<String>>()
                                                        .join(", ");
        extra_list = format!("[{}]", extra_list);

        let json_string = format!("{{ \n   \"{}\": {},\n   \"{}\": {},\n   \"{}\": {},\n   \"{}\": {}\n}}", ConfigName::primary_mensa.as_str(), primary_mensa, ConfigName::mensa_list.as_str(), mensa_list, ConfigName::occupation.as_str(), occupations, ConfigName::extras.as_str(), extra_list);

        //fs::write(path, json_string)?;

        Ok(json_string)


    }

}
/*
    Um an die Namen der Enums zu kommen, wird diese als impl implementiert, dies gibt den Namen als &str aus.
*/

impl Extras {
    pub fn as_str(&self) -> &str {
        match self {
            Extras::Vegan => "vegan",
            Extras::Vegetarisch => "vegetarisch",
            Extras::LactoseFree => "lactoseFree",
            Extras::AlcoholFree => "alcoholFree",
            Extras::BeefFree => "beeffree",
            Extras::FishFree => "fishfree",
            Extras::GelatineFree => "gelatinefree",
            Extras::LambFree => "lambfree",
            Extras::PigFree => "pigfree",
            Extras::PoultryFree => "poultryfree",
            Extras::Other(_) => "other",
        }
    }

    pub fn from_str(s: &str) -> Extras {
        match s.to_lowercase().as_str() {
            "vegan" => Extras::Vegan,
            "vegetarisch" => Extras::Vegetarisch,
            "lactosefree" => Extras::LactoseFree,
            "alcoholfree" => Extras::AlcoholFree,
            "beeffree" => Extras::BeefFree,
            "fishfree" => Extras::FishFree,
            "gelatinefree" => Extras::GelatineFree,
            "lambfree" => Extras::LambFree,
            "pigfree" => Extras::PigFree,
            "poultryfree" => Extras::PoultryFree,
            other => Extras::Other(other.to_string()),
        }
    }
}

impl ConfigName {
    pub fn as_str(&self) -> &str {
        match self {
            ConfigName::primary_mensa => "primary_mensa",
            ConfigName::mensa_list => "mensa_list",
            ConfigName::occupation => "occupation",
            ConfigName::extras => "extras"
        }
    }
}

impl Occupations {
    pub fn as_str(&self) -> &str {
        match self {
            Occupations::Student => "student",
            Occupations::Employee => "employee",
            Occupations::Guest => "guest",
        }
    }

    pub fn from_str(s: &str) -> Option<Occupations> {
        match s.to_lowercase().as_str() {
            "student" => Some(Occupations::Student),
            "employee" => Some(Occupations::Employee),
            "guest" => Some(Occupations::Guest),
            _ => None,
        }
    }
}

