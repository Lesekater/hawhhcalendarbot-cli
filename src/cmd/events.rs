use clap::{Parser};

#[derive(Debug, Parser)]
pub struct Cmd {}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Implement the events command");
    }
}