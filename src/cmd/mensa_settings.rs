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
                // Hier würdest du die primäre Mensa setzen
            }
            SettingsCommands::Add { mensa } => {
                println!("Adding mensa: {}", mensa);
                // Hier würdest du eine Mensa hinzufügen
            }
            SettingsCommands::Remove { mensa } => {
                println!("Removing mensa: {}", mensa);
                // Hier würdest du eine Mensa entfernen
            }
            SettingsCommands::List => {
                println!("Listing all mensas.");
                // Hier würdest du alle verfügbaren Mensas auflisten
            }
            SettingsCommands::Occupation { occupation } => {
                println!("Setting occupation to: {}", occupation);
                // Hier würdest du die Occupation setzen
            }
            SettingsCommands::Extras { extras } => {
                println!("Setting extras to: {}", extras);
                // Hier würdest du die Extras setzen
            }
        }
        Ok(())
    }
}
