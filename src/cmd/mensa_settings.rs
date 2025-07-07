use clap::{Parser, Subcommand};
use crate::json_parser::Config;
use crate::json_parser::Extras;
use crate::json_parser::Occupations;
use std::fs;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    pub command: SettingsCommands,
}

#[derive(Subcommand, Debug)]
pub enum SettingsCommands {
    /// Sets the primary mensa
    Primary {
        /// The mensa to set as primary
        mensa: String,
    },
    /// Adds a mensa
    Add {
        /// The mensa to add
        mensa: String,
    },
    /// Removes a mensa
    Remove {
        /// The mensa to remove
        mensa: String,
    },
    /// Lists all mensas configured in the settings
    List,
    /// Sets the occupation (student, employee, guest)
    Occupation {
        /// The occupation to set
        occupation: String,
    },
    /// Sets the extras (vegan, vegetarian, lactose-free, no alcohol, no beef, no fish...)
    Extras {
        /// The extras to set
        extras: String,
    },
    /// Sets the username for the MuP Plan site
    Username {
        /// The username to set
        username: String,
    },
    /// Sets the password for the MuP Plan site
    Password {
        /// The password to set
        password: String,
    },
    /// Shows the Path to the Config.json file.
    Config,

    Delet,
}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            SettingsCommands::Primary { mensa } => {
                println!("Setting primary mensa to: {}", mensa);

                let mut cfg = Config::load_config();
                cfg.update_primary_mensa(mensa);
                Config::save_config_json(&cfg);
                Ok(())
            }
            SettingsCommands::Add { mensa } => {
                println!("Adding mensa: {}", mensa);

                let mut cfg = Config::load_config();
                cfg.update_mensa_list(mensa);
                Config::save_config_json(&cfg);

                Ok(())
            }
            SettingsCommands::Remove { mensa } => {
                println!("Removing mensa: {}", mensa);

                let mut cfg = Config::load_config();
                cfg.remove_mensa(mensa);
                Config::save_config_json(&cfg);

                Ok(())
            }
            SettingsCommands::List => {
                let mut cfg = Config::load_config();
                let mensa_list = cfg.get_mensa_list();

                match mensa_list {
                    Some(list) if !list.is_empty() => {
                        println!("Additional mensas configured in the settings:");
                        for mensa in list {
                            println!("- {}", mensa);
                        }
                    }
                    _ => {
                        println!("No additional mensas configured in the settings.");
                        return Ok(());
                    }
                }

                Ok(())
            }
            SettingsCommands::Occupation { occupation } => {
                println!("Setting occupation to: {}", occupation);

                let mut cfg = Config::load_config();
                let o = Occupations::from_str(&occupation).unwrap();
                cfg.update_occupation(o);
                Config::save_config_json(&cfg);
                
                Ok(())
            }
            SettingsCommands::Extras { extras } => {
                println!("Setting extras to: {}", extras);

                let mut cfg = Config::load_config();

                let e = Extras::from_str(&extras);

                cfg.add_extra(e);
                
                Config::save_config_json(&cfg);

                Ok(())
            } 
            SettingsCommands::Username { username } => {
                println!("Setting Username to: {}", username);

                let mut cfg = Config::load_config();
                cfg.update_username(username);

                Config::save_config_json(&cfg);
                Ok(())
            }
            SettingsCommands::Password { password } => {
                println!("Setting Password to: {}", password);

                let mut cfg = Config::load_config();

                //TODO: Passwort verschlüsselung einbauen

                cfg.update_password(password);

                Config::save_config_json(&cfg);

                Ok(())
            }

            SettingsCommands::Config {  } => {

                let path = dirs::config_local_dir()
                        .unwrap()
                        .join("hawhhcalendarbot-cli/cfg.json");

                match fs::read_to_string(&path) {
                    Ok(_) => println!("Config file is here: {}", path.display()),
                    Err(_) => println!("No config file found!"),
                }
                

                Ok(())
            }

            SettingsCommands::Delet {  } => {
                let path = dirs::config_local_dir()
                            .unwrap()
                            .join("hawhhcalendarbot-cli/cfg.json");

                if path.exists() {
                    match fs::remove_file(&path) {
                        Ok(_) => println!("Config file deleted: {}", path.display()),
                        Err(e) => eprintln!("Failed to delete config file: {}", e),
                    }
                } else {
                    println!("No config file to delete.");
                }

                Ok(())
            }
        }
    }
}
