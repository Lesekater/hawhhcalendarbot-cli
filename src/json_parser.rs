use std::fs;
use json::JsonValue;

#[derive(Debug)]
pub enum Occupations {
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
pub struct Config {
    primary_mensa: Option<String>,
    mensa_list: Option<Vec<String>>,
    occupation: Option<Occupations>,
    extras: Option<Vec<Extras>>,
}

impl Config {
    pub fn config_from_json_file(path: &str) -> Result<Config, String> {
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let parsed = json::parse(&data).map_err(|e| e.to_string())?;

        // primary_mensa
        let primary_mensa = parsed["primary_mensa"]
            .as_str()
            .map(|s| s.to_string());

        // mensa_list
        let mensa_list = parsed["mensa_list"]
            .members()
            .map(|m| m.as_str().map(|s| s.to_string()))
            .collect::<Option<Vec<_>>>();

        // occupation
        let occupation = match parsed["occupation"].as_str() {
            Some("student") => Some(Occupations::Student),
            Some("employee") => Some(Occupations::Employee),
            Some("guest") => Some(Occupations::Guest),
            Some(_) => None,
            None => None,
        };

        // extras
        let extras = if parsed.has_key("extras") {
            let mut ex_list = Vec::new();
            for item in parsed["extras"].members() {
                match item.as_str() {
                    Some("Vegan") => ex_list.push(Extras::Vegan),
                    Some("Vegetarisch") => ex_list.push(Extras::Vegetarisch),
                    Some("LactoseFree") => ex_list.push(Extras::LactoseFree),
                    Some("AlcoholFree") => ex_list.push(Extras::AlcoholFree),
                    Some("BeefFree") => ex_list.push(Extras::BeefFree),
                    Some("FishFree") => ex_list.push(Extras::FishFree),
                    Some("GelatineFree") => ex_list.push(Extras::GelatineFree),
                    Some("LambFree") => ex_list.push(Extras::LambFree),
                    Some("PigFree") => ex_list.push(Extras::PigFree),
                    Some("PoultryFree") => ex_list.push(Extras::PoultryFree),
                    Some(other) => ex_list.push(Extras::Other(other.to_string())),
                    None => {}
                }
            }
            Some(ex_list)
        } else {
            None
        };

        Ok(Config {
            primary_mensa,
            mensa_list,
            occupation,
            extras,
        })
    }

    pub fn json_file_from_config(&self, path: &str) -> Result<(), String>  {
        let mut obj = JsonValue::new_object();

        // primary_mensa
        if let Some(ref pm) = self.primary_mensa {
            obj["primary_mensa"] = pm.as_str().into();
        }

        // mensa_list
        if let Some(ref list) = self.mensa_list {
            let arr = list.iter().map(|s| s.as_str().into()).collect();
            obj["mensa_list"] = JsonValue::Array(arr);
        }

        // occupation
        if let Some(ref occ) = self.occupation {
            let occ_str = match occ {
                Occupations::Student => "student",
                Occupations::Employee => "employee",
                Occupations::Guest => "guest",
            };
            obj["occupation"] = occ_str.into();
        }

        // extras
        if let Some(ref extras) = self.extras {
            let arr = extras.iter().map(|e| {
                match e {
                    Extras::Vegan => "Vegan",
                    Extras::Vegetarisch => "Vegetarisch",
                    Extras::LactoseFree => "LactoseFree",
                    Extras::AlcoholFree => "AlcoholFree",
                    Extras::BeefFree => "BeefFree",
                    Extras::FishFree => "FishFree",
                    Extras::GelatineFree => "GelatineFree",
                    Extras::LambFree => "LambFree",
                    Extras::PigFree => "PigFree",
                    Extras::PoultryFree => "PoultryFree",
                    Extras::Other(s) => s,
                }.into()
            }).collect();
            obj["extras"] = JsonValue::Array(arr);
        }

        // Speichern
        std::fs::write(path, obj.dump()).map_err(|e| e.to_string())
    }
}
