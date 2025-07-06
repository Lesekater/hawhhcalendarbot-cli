use crate::events::event::Event_Meta;
use crate::events::{
    event::{self, Event},
    haw_event::HawEventEntry,
};
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
    },
    /// Fetches all event data and stores it in the cache directory.
    Fetch,
    /// List departments that have events.
    ListDepartments,
    /// List modules for a specific department
    ListModules {
        /// The department to list modules for
        department: String,
    },
}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            EventCommands::Get { date, module } => {
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
                    event_meta.push(Event_Meta {
                        department: String::from("informatik"), // TODO: set department from config or input
                        module,
                    });
                }

                // Call the trait method via fully qualified syntax
                let events: Vec<HawEventEntry> = Event::get_events_for_date(event_meta, date)?;

                // Output events (as JSON or plain)
                if self.json {
                    println!("{}", serde_json::to_string_pretty(&events)?);
                } else {
                    for event in events {
                        println!("{:?}", event);
                    }
                }
            }
            EventCommands::Fetch => {
                // Fetch all event data and store it in the cache directory
                HawEventEntry::fetch_event_data(&HawEventEntry::get_cache_dir()?)?;
                println!("Event data fetched and stored successfully.");
            }
            EventCommands::ListDepartments => {
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
        }
        Ok(())
    }
}
