/*
 * Commands:
 * 
 * calendarbot --help / calendarbot help
 * calendarbot --version / calendarbot version
 * ## Mensa
 * calendarbot mensa: shows the mensa menu for today
 * calendarbot mensa tomorrow: shows the mensa menu for tomorrow
 * calendarbot mensa 2023-10-01: shows the mensa menu for the given date
 * calendarbot mensa settings: shows the mensa settings
 *   calendarbot mensa settings <setting> <value>: sets the mensa setting
 *   avalible settings:
 *    - primary: sets the primary mensa
 *    - add: adds a mensa
 *    - remove: removes a mensa
 *    - list: lists all mensas
 *    - occupation: sets the occupation (student, employee, guest)
 *    - extras: sets the extras (vegan, vegetarian, lactose-free, no alcohol, no beef, no fish...)
 *    - show ingredients: shows the ingredients when showing the menu
 * ## Events
 * calendarbot events: shows the selected events
 * calendarbot events list: lists all available events
 * calendarbot events list --filter <filter>: lists all available events with the given filter
 * calendarbot events add <event>: adds the event to the calendar
 * calendarbot events remove <event>: removes the event from the calendar
*/  

mod mensa_data;
mod meal;

use clap::{Parser, Subcommand};
use meal::Meal;

#[derive(Parser)]
#[clap(name = "calendarbot", version = "1.0", author = "Your Name")]
#[clap(about = "A simple calendar bot for the mensa and events")]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,
    /// Whether to output the results in JSON format
    #[arg(long, short, default_value = "false")]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows the mensa menu for today
    Mensa {
        #[clap(subcommand)]
        command: MensaCommands,
    },
    /// Shows the selected events
    Events {
        /// The event to show
        event: Option<String>,
    },
}

#[derive(Subcommand)]
#[derive(Debug)]
enum MensaCommands {
    /// Shows the mensa menu for today
    Today,
    /// Shows the mensa menu for tomorrow
    Tomorrow,
    /// Shows the mensa menu for the given date
    Date {
        /// The date to show the mensa menu for
        date: String,
    },
    /// Force update of the mensa data
    Update,
    /// Shows the mensa settings
    Settings {
        #[clap(subcommand)]
        command: SettingsCommands,
    },
}

#[derive(Subcommand)]
#[derive(Debug)]
enum SettingsCommands {
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
    /// Lists all mensas
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
}

fn main() {
    let cli = Cli::parse();

    let local_data = match mensa_data::mensa_data::load_local_data() {
        Ok(data) => data,
        Err(e) => {
            println!("Local data not available: {}", e);
            println!("Attempting to fetch data from the server...");

            // Attempt to fetch the data if not available locally
            if mensa_data::mensa_data::fetch_mensa_data().is_err() {
                println!("Error fetching mensa data: {}", e);
                std::process::exit(1);
            }

            println!("Data fetched successfully.\n");

            mensa_data::mensa_data::load_local_data().unwrap()
        },
    };

    let currentdate = chrono::Local::now().date_naive();

    match cli.command {
        Commands::Mensa { command } => {
            match &command {
                MensaCommands::Today | MensaCommands::Tomorrow | MensaCommands::Date { .. } => {
                    let date_to_use = match &command {
                        MensaCommands::Today => currentdate,
                        MensaCommands::Tomorrow => currentdate.succ_opt().expect("Failed to get tomorrow's date"),
                        MensaCommands::Date { date } => {
                            match chrono::NaiveDate::parse_from_str(date, "%d.%m.%Y") {
                                Ok(parsed_date) => parsed_date,
                                Err(_) => {
                                    println!("Invalid date format. Please use DD.MM.YYYY.");
                                    return;
                                },
                            }
                        },
                        _ => panic!("Unexpected command variant"),
                    };

                    let food_for_date = local_data.get("Mensa Berliner Tor")
                .and_then(|mensa| mensa.get(&date_to_use.format("%Y").to_string()))
                .and_then(|year| year.get(&date_to_use.format("%m").to_string()))
                .and_then(|month| month.get(&date_to_use.format("%d").to_string()))
                .expect("Data for today not found")
                .iter()
                .collect::<Vec<&Meal>>();
                    if cli.json {
                        println!("{}", serde_json::to_string(&food_for_date).unwrap());
                        return;
                    }

                    println!("Mensa Berliner Tor");
                    println!("{}", date_to_use.format("%Y-%m-%d"));
                    for food in food_for_date {
                        println!();
                        println!("{}", food);
                    }
                },
                MensaCommands::Update => {
                    println!("Updating mensa data...");
                    match mensa_data::mensa_data::fetch_mensa_data() {
                        Ok(_) => println!("Mensa data updated successfully."),
                        Err(e) => println!("Error updating mensa data: {}", e),
                    };
                },
                MensaCommands::Settings { command } => {
                    match command {
                        SettingsCommands::Primary { mensa } => {
                            println!("Setting primary mensa to: {}", mensa);
                            // Here you would set the primary mensa
                        },
                        SettingsCommands::Add { mensa } => {
                            println!("Adding mensa: {}", mensa);
                            // Here you would add a mensa
                        },
                        SettingsCommands::Remove { mensa } => {
                            println!("Removing mensa: {}", mensa);
                            // Here you would remove a mensa
                        },
                        SettingsCommands::List => {
                            println!("Listing all mensas.");
                            // Here you would list all available mensas
                        },
                        SettingsCommands::Occupation { occupation } => {
                            println!("Setting occupation to: {}", occupation);
                            // Here you would set the occupation
                        },
                        SettingsCommands::Extras { extras } => {
                            println!("Setting extras to: {}", extras);
                            // Here you would set the extras
                        },
                    }
                },
            }
        },
        Commands::Events { event } => {
            println!("Events command (event: {:?})", event);
        },
    }
}
