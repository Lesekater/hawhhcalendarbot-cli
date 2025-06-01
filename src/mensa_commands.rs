use crate::{mensa_data::{self, mensa_data::{fetch_mensa_data, MensaData}}, Cli, MensaCommands, SettingsCommands};

const MENSA_NAME: &str = "Mensa Berliner Tor";

pub fn match_mensa_commands(
    command: &Option<MensaCommands>,
    local_data: &MensaData,
    currentdate: chrono::NaiveDate,
    cli: &Cli,
) {
    match &command {
        Some(MensaCommands::Today)
        | Some(MensaCommands::Tomorrow)
        | Some(MensaCommands::Date { .. }) => {
            today_command(&command, &local_data, currentdate, &cli);
        }
        Some(MensaCommands::Update) => {
            println!("Updating mensa data...");
            match fetch_mensa_data() {
                Ok(_) => println!("Mensa data updated successfully."),
                Err(e) => println!("Error updating mensa data: {}", e),
            };
        }
        Some(MensaCommands::Settings { command }) => {
            match command {
                SettingsCommands::Primary { mensa } => {
                    println!("Setting primary mensa to: {}", mensa);
                    // Here you would set the primary mensa
                }
                SettingsCommands::Add { mensa } => {
                    println!("Adding mensa: {}", mensa);
                    // Here you would add a mensa
                }
                SettingsCommands::Remove { mensa } => {
                    println!("Removing mensa: {}", mensa);
                    // Here you would remove a mensa
                }
                SettingsCommands::List => {
                    println!("Listing all mensas.");
                    // Here you would list all available mensas
                }
                SettingsCommands::Occupation { occupation } => {
                    println!("Setting occupation to: {}", occupation);
                    // Here you would set the occupation
                }
                SettingsCommands::Extras { extras } => {
                    println!("Setting extras to: {}", extras);
                    // Here you would set the extras
                }
            }
        }
        None => {
            today_command(&Some(MensaCommands::Today), &local_data, currentdate, &cli);
        }
    }
}

fn today_command(
    command: &Option<MensaCommands>,
    local_data: &MensaData,
    currentdate: chrono::NaiveDate,
    cli: &Cli,
) {
    let date_to_use = match &command {
        Some(MensaCommands::Today) => currentdate,
        Some(MensaCommands::Tomorrow) => currentdate
            .succ_opt()
            .expect("Failed to get tomorrow's date"),
        Some(MensaCommands::Date { date }) => {
            match chrono::NaiveDate::parse_from_str(date, "%d.%m.%Y") {
                Ok(parsed_date) => parsed_date,
                Err(_) => {
                    println!("Invalid date format. Please use DD.MM.YYYY.");
                    return;
                }
            }
        }
        _ => panic!("Unexpected command variant"),
    };

    let food_for_date = match mensa_data::mensa_data::get_food_for_date(local_data, date_to_use, MENSA_NAME) {
        Ok(food) => food,
        Err(e) => {
            println!("Error fetching food for date: {}", e);
            return;
        }
    };
    
    if cli.json {
        println!("{}", serde_json::to_string(&food_for_date).unwrap());
        return;
    }

    println!("{}", MENSA_NAME);
    println!("{}", date_to_use.format("%Y-%m-%d"));
    for food in food_for_date {
        println!();
        println!("{}", food);
    }
}
