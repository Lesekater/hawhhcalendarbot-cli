use clap::{builder::Str, Parser, Subcommand};


#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    pub command: SettingsCommands,
}

#[derive(Subcommand, Debug)]
pub enum SettingsCommands {
    /// Shows the Events, selected in the Config
    Events,

    ///Shows a list of all availabel Events
    List {
        /// Filter nach z. B. Kategorie, Datum, etc.
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Adds the Event to the Calender
    Add {
        event: String,
    },

    /// Removes the Event to the Calender
    Remove {
        event: String,
    },


}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            SettingsCommands::Events {} => {
                println!("Setting primary mensa to:");
                Ok(())
            }

            SettingsCommands::List { filter } => {

                Ok(())
            }

            SettingsCommands::Add { event } => {

                Ok(())
            }

            SettingsCommands::Remove { event } => {

                Ok(())
            }
        }
    }
}