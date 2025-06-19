use clap::{Parser, Subcommand};

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

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            SettingsCommands::Primary { mensa } => {
                println!("Setting primary mensa to: {}", mensa);
                todo!("Implement the logic to set the primary mensa in the settings");
            }
            SettingsCommands::Add { mensa } => {
                println!("Adding mensa: {}", mensa);
                todo!("Implement the logic to add a mensa in the settings");
            }
            SettingsCommands::Remove { mensa } => {
                println!("Removing mensa: {}", mensa);
                todo!("Implement the logic to remove a mensa in the settings");
            }
            SettingsCommands::List => {
                println!("Listing all mensas.");
                todo!("Implement the logic to list all mensas in the settings");
            }
            SettingsCommands::Occupation { occupation } => {
                println!("Setting occupation to: {}", occupation);
                todo!("Implement the logic to set the occupation in the settings");
            }
            SettingsCommands::Extras { extras } => {
                println!("Setting extras to: {}", extras);
                todo!("Implement the logic to set the extras in the settings");
            }
        }
    }
}
