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

mod meal;
mod mensa_commands;
mod mensa_data;
mod tests;

mod config_managment;

use clap::{Parser, Subcommand};
use mensa_commands::match_mensa_commands;

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
        command: Option<MensaCommands>,

        #[arg(short, long)]
        number: Option<i32>,
    },
    /// Shows the selected events
    Events {
        /// The event to show
        event: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum MensaCommands {
    /// Shows the mensa menu for today
    #[clap(alias = "tod")]
    Today {
        #[arg(short, long)]
        number: Option<i32>,
    },
    /// Shows the mensa menu for tomorrow
    #[clap(alias = "tom")]
    Tomorrow {
        #[arg(short, long)]
        number: Option<i32>,
    },
    /// Shows the mensa menu for the given date
    #[clap(alias = "d")]
    Date {
        /// The date to show the mensa menu for
        date: String,

        #[arg(short, long)]
        number: Option<i32>,
    },
    /// Force full update of the mensa data
    #[clap(alias = "u")]
    Update,
    /// Force full cache load of the mensa data
    #[clap(alias = "c")]
    Cache,
    /// Shows the mensa settings
    #[clap(alias = "s")]
    Settings {
        #[clap(subcommand)]
        command: SettingsCommands,
    },
}

#[derive(Subcommand, Debug)]
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

    let currentdate = chrono::Local::now().date_naive();

    match &cli.command {
        Commands::Mensa { command , number } => {
            match_mensa_commands(command, currentdate, &cli, number);
        }
        Commands::Events { event: _ } => {}
    }
}
