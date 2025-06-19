use clap::{Parser, Subcommand};

pub(crate) mod mensa;
pub(crate) mod events;
pub mod mensa_settings;

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Shows the mensa menu for today
    Mensa(mensa::Cmd),
    /// Shows the selected events
    Events(events::Cmd),
}

#[derive(Debug, Parser)]
#[clap(name = "calendarbot", version = "1.0", author = "Your Name")]
#[clap(about = "A simple calendar bot for the mensa and events")]
pub struct Cli {
    #[clap(subcommand)]
    action: Action,

    /// Whether to output the results in JSON format
    #[arg(long, short, default_value = "false", global = true)]
    json: bool,
}

impl Cli {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        use Action::*;

        match self.action {
            Mensa(cmd) => cmd.run(),
            Events(cmd) => cmd.run(),
        }
    }
}