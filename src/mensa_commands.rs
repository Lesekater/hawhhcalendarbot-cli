use crate::{
    Cli, MensaCommands, SettingsCommands,
    mensa_data::{
        self,
        mensa_data::fetch_mensa_data,
    },
};

const MENSA_NAME: &str = "Mensa Berliner Tor";

pub fn match_mensa_commands(
    command: &Option<MensaCommands>,
    currentdate: chrono::NaiveDate,
    cli: &Cli,
) {
    // let local_data = match mensa_data::mensa_data::load_local_data() {
    //     Ok(data) => data,
    //     Err(e) => {
    //         println!("Local data not available: {}", e);
    //         println!("Attempting to fetch data from the server...");

    //         // Attempt to fetch the data if not available locally
    //         if mensa_data::mensa_data::fetch_mensa_data().is_err() {
    //             println!("Error fetching mensa data: {}", e);
    //             std::process::exit(1);
    //         }

    //         println!("Data fetched successfully.\n");

    //         mensa_data::mensa_data::load_local_data().unwrap()
    //     }
    // };

    match &command {
        Some(MensaCommands::Today)
        | Some(MensaCommands::Tomorrow)
        | Some(MensaCommands::Date { .. }) => {
            date_command(&command, currentdate, &cli);
        }
        Some(MensaCommands::Update) | Some(MensaCommands::Cache) => {
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
            date_command(&Some(MensaCommands::Today), currentdate, &cli);
        }
    }
}

fn date_command(
    command: &Option<MensaCommands>,
    currentdate: chrono::NaiveDate,
    cli: &Cli,
) {
    // Determine the date to use based on the command
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

    // Find the food for the specified date
    let food_for_date =
        match mensa_data::mensa_data::get_food_for_date(date_to_use, MENSA_NAME) {
            Ok(food) => food,
            Err(e) => {
                println!("Error fetching food for date: {}", e);
                return;
            }
        };

    // If json option is set, print the food in JSON format
    if cli.json {
        println!("{}", serde_json::to_string(&food_for_date).unwrap());
        return;
    }

    // output formatted date and food items
    println!("{}\n{}", MENSA_NAME, date_to_use.format("%Y-%m-%d"));
    for food in food_for_date {
        println!("\n{}", food);
    }
}
