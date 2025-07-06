use crate::events::event::Event_Meta;
use crate::events::{
    event::{self, Event},
    haw_event::HawEventEntry,
};
use crate::json_parser::Config;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    command: EventCommands,

    #[arg(short, long, global = true)]
    filter: Option<String>,

    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Subcommand, Debug)]
enum EventCommands {
    /// Get events for a specific date / module
    Get {
        /// The date to get events for
        date: String,
        module: Option<String>,
        department: Option<String>,
    },
    /// Fetches all event data and stores it in the cache directory.
    Cache,
    /// List departments that have events.
    ListDepartments,
    /// List modules for a specific department
    ListModules {
        /// The department to list modules for
        department: String,
    },
    /// Add module to config
    Add {
        /// The module to add
        module: String,
        /// The department of the module
        department: String,
    },
    /// Remove module from config
    Remove {
        /// The module to remove
        module: String,
        /// The department of the module
        department: String,
    },
}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            EventCommands::Get { date, module, department } => {
                // Parse date string to NaiveDate
                let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| {
                    format!(
                        "Failed to parse date '{}'. Expected format: YYYY-MM-DD",
                        date
                    )
                })?;

                // Build Event_Meta vector (department is required, adjust as needed)
                let mut event_meta = Vec::new();
                if let Some(module) = module {
                    if module.is_empty() {
                        return Err("Module cannot be empty".into());
                    }

                    if let Some(department) = &department {
                        if department.is_empty() {
                            return Err("Department cannot be empty".into());
                        }
                    } else {
                        return Err("Department must be provided when specifying a module".into());
                    }

                    let config = Config::load_config();
                    config.get_events();
                    event_meta.push(Event_Meta {
                        department: department.expect("No department provided").to_lowercase(),
                        module: module.to_lowercase(),
                    });
                }

                // Call the trait method via fully qualified syntax
                let events: Vec<HawEventEntry>;
                if event_meta.is_empty() {
                    // If no module or date is provided, default to config
                    events = Event::get_all_events_for_date(date)?;
                } else {
                    // Use the provided module and date
                    events = Event::get_events_for_date(event_meta, date)?;
                }

                // Output events (as JSON or plain)
                if self.json {
                    println!("{}", serde_json::to_string_pretty(&events)?);
                } else {
                    for event in events {
                        println!("{}\n", event);
                    }
                }
            }
            EventCommands::Cache => {
                // Fetch all event data and store it in the cache directory
                HawEventEntry::fetch_event_data(&HawEventEntry::get_cache_dir()?)?;
                println!("Event data fetched and stored successfully.");
            }
            EventCommands::ListDepartments => {
                // TODO: Use timecode val to reload event data if needed
                HawEventEntry::fetch_event_data(&HawEventEntry::get_cache_dir()?)?;
                println!("");

                // List all departments that have events
                let departments = HawEventEntry::get_departments()?;
                if self.json {
                    println!("{}", serde_json::to_string_pretty(&departments)?);
                } else {
                    for department in departments {
                        println!("{}", department);
                    }
                }
            }
            EventCommands::ListModules { department } => {
                // TODO: Use timecode val to reload event data if needed
                HawEventEntry::fetch_event_data(&HawEventEntry::get_cache_dir()?)?;
                println!("");

                // List all modules for the specified department
                let modules = HawEventEntry::get_modules_for_department(&department, self.filter.as_deref())?;
                if self.json {
                    println!("{}", serde_json::to_string_pretty(&modules)?);
                } else {
                    for module in modules {
                        println!("{}", module);
                    }
                }
            }
            EventCommands::Add { module, department } => {
                // Check if module is valid
                let valid_modules = HawEventEntry::get_modules_for_department(&department, None)?;

                if !valid_modules.contains(&module) {
                    return Err(format!(
                        "Invalid module '{}' for department '{}'",
                        module, department
                    ).into());
                }

                // Add a module to the config
                println!("Setting module '{}' in department '{}'...", module, department);

                let mut cfg = Config::load_config();
                cfg.add_module(&module, &department)?;
                Config::save_config_json(&cfg);
            }
            EventCommands::Remove { module, department } => {
                // Remove a module from the config
                println!("Removing module '{}' in department '{}'...", module, department);

                let mut cfg = Config::load_config();
                cfg.remove_module(&module, &department)?;
                Config::save_config_json(&cfg);
            }
        }
        Ok(())
    }
}
