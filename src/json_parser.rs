use std::{clone, fmt::{format, Error}, fs::{self, File}, vec};

#[derive(Debug)]
pub(crate) enum Occupations {
    Student,
    Employee,
    Guest,
}

#[derive(Debug)]
pub(crate) enum Extras {
    Vegan,
    Vegetarian,
    LactoseFree,
    Alcohol,
    BeefFree,
    Fish,
    GelatineFree,
    LambFree,
    PigFree,
    PoultryFree,
    Unknown,
}

impl clone::Clone for Occupations {
    fn clone(&self) -> Self {
        match self {
            Occupations::Student => Occupations::Student,
            Occupations::Employee => Occupations::Employee,
            Occupations::Guest => Occupations::Guest,
        }
    }
}

impl clone::Clone for Extras {
    fn clone(&self) -> Self {
        match self {
            Extras::Vegan => Extras::Vegan,
            Extras::Vegetarian => Extras::Vegetarian,
            Extras::LactoseFree => Extras::LactoseFree,
            Extras::Alcohol => Extras::Alcohol,
            Extras::BeefFree => Extras::BeefFree,
            Extras::Fish => Extras::Fish,
            Extras::GelatineFree => Extras::GelatineFree,
            Extras::LambFree => Extras::LambFree,
            Extras::PigFree => Extras::PigFree,
            Extras::PoultryFree => Extras::PoultryFree,
            Extras::Unknown => Extras::Unknown,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ConfigName {
    primary_mensa,
    mensa_list,
    occupation,
    extras,
    events,
}


#[derive(Debug)]
pub struct Config {
    primary_mensa: Option<String>,
    mensa_list: Option<Vec<String>>,
    occupation: Option<Occupations>,
    extras: Option<Vec<Extras>>,
    events: Option<Vec<String>>,
}

impl clone::Clone for Config {
    fn clone(&self) -> Self {
        Config {
            primary_mensa: self.primary_mensa.clone(),
            mensa_list: self.mensa_list.clone(),
            occupation: self.occupation.clone(),
            extras: self.extras.clone(),
            events: self.events.clone(),
        }
    }
}

/*
    Implementierung des Json Parser:
*/

impl Config {

//Config:
    fn new() -> Self{
        Self {
            primary_mensa: Some(String::new()),
            mensa_list: Some(Vec::new()),
            occupation: None,
            extras: Some(Vec::new()),
            events: Some(Vec::new()),
        }

    }

    
    pub fn update_primary_mensa(&mut self, mensa_to_add: String) {
        self.primary_mensa = Some(mensa_to_add);
    }

    pub fn get_primary_mensa(&self) -> Option<String>{
        self.primary_mensa.clone()
    }

    pub fn update_mensa_list(&mut self, mensa_to_add: String) {

        let mensa_list = self.mensa_list.as_mut().unwrap();
        mensa_list.push(mensa_to_add);
    }

    pub fn get_mensa_list(&mut self) -> Option<&Vec<String>>{

        Some(self.mensa_list.as_ref().unwrap())
    }

    pub fn remove_mensa(&mut self, mensa_to_remove: String) {
        if let Some(mensa_list) = self.mensa_list.as_mut() {
            mensa_list.retain(|e| e.as_str() != mensa_to_remove.as_str());
        }
    }

    pub fn update_occupation(&mut self, new_occ: Occupations) {
        self.occupation = Some(new_occ);
    }

    pub fn get_occupation(&self) -> Option<&Occupations> {

        self.occupation.as_ref()
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

    pub fn get_extras(&self) -> Option<&Vec<Extras>> {

        self.extras.as_ref()
    }

    pub fn load_config() -> Config {
        let path = dirs::config_local_dir()
                .unwrap()
                .join("hawhhcalendarbot/cfg.json");
        match fs::read_to_string(path,
        ) {
            Ok(json_config) => Config::struct_from_json_file(&json_config).expect("Fehler beim Parsen der JSON"),
            Err(_) => Config::new(),
        }
        
    }

    pub fn save_config_json(user_config: &Config) {
        let conf_dir = dirs::config_local_dir().unwrap().join("hawhhcalendarbot");

        let json_string = Config::json_file_from_struct(user_config).expect("Fehler beim Serialisieren");
        let _ = fs::create_dir_all(conf_dir);

        fs::write(
            dirs::config_local_dir()
                .unwrap()
                .join("hawhhcalendarbot/cfg.json"),
        json_string,
        )
        .expect("Fehler beim Schreiben der Datei");
    }

//Hilfs funktionen:
    fn strip_leading_null(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some('\0') => chars.collect(), // Rest ab dem zweiten Zeichen
            _ => s.to_string(),            // Unverändert zurückgeben
        }
    }   

//Json Parser
    fn struct_from_json_file(/*path: &str*/ json_config: &String) -> Result<Config, Box<dyn std::error::Error>> {
        let search_offset: usize = 4;
        //let mut config = Config::new();

        //let config_content_raw = fs::read_to_string(path)?;
        let config_content_raw = json_config;


        //Whitespace, Leerzeichen und Zeilenumbrüche entfernen
        let config_content_cleaned =  config_content_raw
                                            .chars()
                                            //.filter(|c| !c.is_whitespace()) 
                                            .collect::<String>();
        

        //End index, der gesucht berechnen:
        let pm_end = config_content_cleaned.find(ConfigName::primary_mensa.as_str()).unwrap() + ConfigName::primary_mensa.as_str().len();
        let ml_end = config_content_cleaned.find(ConfigName::mensa_list.as_str()).unwrap() + ConfigName::mensa_list.as_str().len();
        let op_end = config_content_cleaned.find(ConfigName::occupation.as_str()).unwrap() + ConfigName::occupation.as_str().len();
        let et_end = config_content_cleaned.find(ConfigName::extras.as_str()).unwrap() + ConfigName::extras.as_str().len();
        let ev_end = config_content_cleaned.find(ConfigName::events.as_str()).unwrap() + ConfigName::events.as_str().len();

        //Inhalt der primary mensa extrahieren:

        let slice = &config_content_cleaned[pm_end + search_offset..];
        let pm = slice.find('"').map(|end| {
            slice[..end].to_string()
        });

        let primary_mensa = match pm {
            Some(pm_str) => format!("{}", pm_str),
            None => "null".to_string(), // oder "" wenn du leeren String willst
        };
        
        //Inhalt der mensa list extrahieren:

        let slice = &config_content_cleaned[ml_end + search_offset..];
        let mensa_list_all = slice.find(']').map(|end| {&slice[..end]});

        let mensa_list_all = match mensa_list_all {
            Some(list) => list,
            None => "", // oder ein anderer Default-Wert
        };



        let mut mensa_list: Vec<String> = mensa_list_all
                                            .chars()
                                            .filter(|&c| c != '"')
                                            .collect::<String>()
                                            .split(',')
                                            .map(|s| s.to_string())
                                            .collect();

        if mensa_list.first().map_or(false, |s| s.is_empty()) {
            mensa_list.remove(0);
        };

        //Inhalt der Occupation extrahieren:

        let slice = &config_content_cleaned[op_end + search_offset..];
        let occupations_string = slice.find('"').map(|end| {slice[..end].to_string()});
        let occupations = match occupations_string {
            Some(s) => Occupations::from_str(&s),
            None => Some(Occupations::Employee), // oder wie du standardmäßig damit umgehen möchtest
        };


        //Inhalt der Extras extrahieren:
        let extra_list_all =  &config_content_cleaned[et_end+search_offset..et_end+search_offset + config_content_cleaned[et_end+search_offset..].find(']').unwrap()];

        let mut extra_list_string: Vec<String> = extra_list_all
                                            .chars()
                                            .filter(|&c| c != '"')
                                            .collect::<String>()
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            //.map(|e| Extras::from_str(&e))
                                            .collect();
        
        if extra_list_string.first().map_or(false, |s| s.is_empty()) {
            extra_list_string.remove(0);
        };

        let extra_list:Vec<Extras> = extra_list_string.into_iter().map(|e| Extras::from_str(&e)).collect();

        
        
        //Inhalte der Events extrahieren:
        let slice = &config_content_cleaned[ev_end + search_offset..];
        let event_list_all = slice.find(']').map(|end| {&slice[..end]});

        let mut event_list: Vec<String> = match event_list_all {
            Some(raw) => raw
                .chars()
                .filter(|&c| c != '"')
                .collect::<String>()
                .split(',')
                .map(|s| s.trim().to_string()) // trim() für Sicherheit
                .collect(),
            None => Vec::new(), // Fallback: leere Liste
        };

        if event_list.first().map_or(false, |s| s.is_empty()) {
            event_list.remove(0);
        };


        //Config zurückkgeben:
        Ok(Config { primary_mensa: Some(primary_mensa),
                    mensa_list: Some(mensa_list),
                    occupation: (occupations),
                    extras: Some(extra_list),
                    events: Some(event_list)
                })

    }

    fn json_file_from_struct(config: &Config) -> Result<String, Box<dyn std::error::Error>>  {

        let primary_mensa = match &config.primary_mensa {
            Some(pm) => format!("\"{}\"", pm),
            None => "null".to_string(), // oder "" falls du leere Strings willst
        };

        let mut mensa_list = config.mensa_list
                                                        .iter()
                                                        .map(|s|  format!("{:?}", s))
                                                        .collect::<Vec<String>>()            // in Vec sammeln
                                                        .join(", ");                                      
    

        let occupations = match &config.occupation {
            Some(occ) => format!("\"{:?}\"", occ),
            None => "null".to_string(), // oder "" falls du leere Strings willst
        };

        let mut extra_list = config.extras.as_ref().map_or(
    "null".to_string(), // oder "[]".to_string()
          |extras| {
                let joined = extras
                    .iter()
                    .map(|e| format!("\"{}\"", e.as_str()))
                    .collect::<Vec<String>>()
                    .join(", ");
        format!("[{}]", joined)
            },
        );


        let event_list:String = config.events
                                                        .iter()
                                                        .map(|s|  format!("{:?}", s))
                                                        .collect::<Vec<String>>()            // in Vec sammeln
                                                        .join(", "); 

        let json_string = format!("{{ \n   \"{}\": {},\n   \"{}\": {},\n   \"{}\": {},\n   \"{}\": {},\n   \"{}\": {}\n}}", ConfigName::primary_mensa.as_str(), primary_mensa, ConfigName::mensa_list.as_str(), mensa_list, ConfigName::occupation.as_str(), occupations, ConfigName::extras.as_str(), extra_list, ConfigName::events.as_str(), event_list);

        //fs::write(path, json_string)?;

        Ok(json_string)


    }

}
/*
    Um an die Namen der Enums zu kommen, wird diese als impl implementiert, dies gibt den Namen als &str aus.
*/

impl Extras {
    fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Extras::Vegan => "Vegan",
            Extras::Vegetarian => "Vegetarian",
            Extras::LactoseFree => "LactoseFree",
            Extras::Alcohol => "Alcohol",
            Extras::BeefFree => "Beeffree",
            Extras::Fish => "Fish",
            Extras::GelatineFree => "Gelatinefree",
            Extras::LambFree => "Lambfree",
            Extras::PigFree => "Pigfree",
            Extras::PoultryFree => "Poultryfree",
            Extras::Unknown => "Unknown"
        }
    }

   pub fn from_str(s: &str) -> Extras {
        match Self::capitalize_first(s) {
            ref s if s == "Vegan" => Extras::Vegan,
            ref s if s == "Vegetarian" => Extras::Vegetarian,
            ref s if s == "Lactosefree" => Extras::LactoseFree,
            ref s if s == "Alcohol" => Extras::Alcohol,
            ref s if s == "Beeffree" => Extras::BeefFree,
            ref s if s == "Fish" => Extras::Fish,
            ref s if s == "Gelatinefree" => Extras::GelatineFree,
            ref s if s == "Lambfree" => Extras::LambFree,
            ref s if s == "Pigfree" => Extras::PigFree,
            ref s if s == "Poultryfree" => Extras::PoultryFree,
            _ => Extras::Unknown,
        }
}


    
}

impl ConfigName {
    pub fn as_str(&self) -> &str {
        match self {
            ConfigName::primary_mensa => "primary_mensa",
            ConfigName::mensa_list => "mensa_list",
            ConfigName::occupation => "occupation",
            ConfigName::extras => "extras",
            ConfigName::events => "events",
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

