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
        Ok(data) => {
            println!("Local data loaded successfully.");
            data
        },
        Err(e) => {
            println!("Local data not available: {}", e);
            println!("Attempting to fetch data from the server...");

            // Attempt to fetch the data if not available locally
            if mensa_data::mensa_data::fetch_mensa_data().is_err() {
                println!("Error fetching mensa data: {}", e);
                std::process::exit(1);
            }

            println!("Data fetched successfully.");
            mensa_data::mensa_data::load_local_data().unwrap()
        },
    };

    match cli.command {
        Commands::Mensa { command } => {
            println!("Mensa command (command: {:?})", command);
            // println!("Loaded Mensas: {:?}", local_data.keys());
            // println!("Years of Berliner Tor: {:?}", local_data.get("Mensa Berliner Tor").expect("Name not found").keys());

            print!("\n");
            let currentdate = chrono::Local::now().date_naive();
            let food_today = local_data.get("Mensa Berliner Tor")
                .and_then(|mensa| mensa.get(&currentdate.format("%Y").to_string()))
                .and_then(|year| year.get(&currentdate.format("%m").to_string()))
                .and_then(|month| month.get(&currentdate.format("%d").to_string()))
                .expect("Data for today not found")
                .iter()
                .collect::<Vec<&Meal>>();

            println!("Mensa Berliner Tor");
            println!("{}", &currentdate.format("%d.%m.%Y"));
            for food in food_today {
                println!();
                println!("{}", food);
            }
        },
        Commands::Events { event } => {
            println!("Events command (event: {:?})", event);
        },
    }
}
