use crate::{Cli, MensaCommands, SettingsCommands, config_managment::load_config, mensa_data::*};

pub fn match_mensa_commands(
    command: &Option<MensaCommands>,
    currentdate: chrono::NaiveDate,
    cli: &Cli,
    number: &Option<i32>,
) {
    // let local_data = match mensa_data::load_local_data() {
    //     Ok(data) => data,
    //     Err(e) => {
    //         println!("Local data not available: {}", e);
    //         println!("Attempting to fetch data from the server...");

    //         // Attempt to fetch the data if not available locally
    //         if mensa_data::fetch_mensa_data().is_err() {
    //             println!("Error fetching mensa data: {}", e);
    //             std::process::exit(1);
    //         }

    //         println!("Data fetched successfully.\n");

    //         mensa_data::load_local_data().unwrap()
    //     }
    // };

    match &command {
        Some(MensaCommands::Today { .. })
        | Some(MensaCommands::Tomorrow { .. })
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
            date_command(&Some(MensaCommands::Today { number: *number }), currentdate, &cli);
        }
    }
}

fn date_command(command: &Option<MensaCommands>, currentdate: chrono::NaiveDate, cli: &Cli) {
    // Determine the date to use based on the command
    let date_to_use = match &command {
        Some(MensaCommands::Today { .. }) => currentdate,
        Some(MensaCommands::Tomorrow { .. }) => currentdate
            .succ_opt()
            .expect("Failed to get tomorrow's date"),
        Some(MensaCommands::Date { date, .. }) => {
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

    // Additional Mensa to use if specified
    let additional_mensa: &Option<i32> = match command {
        Some(MensaCommands::Today { number }) => number,
        Some(MensaCommands::Tomorrow { number }) => number,
        Some(MensaCommands::Date { number, .. }) => number,
        _ => &None,
    };

    // Load Config
    let config = load_config();
    let mut mensa_name = "";

    // Check primary mensa
    if additional_mensa.is_none() {
        mensa_name = match config.primary_mensa() {
            Some(name) => name.as_str(),
            None => {
                println!("Primary Mensa is not set - please set it in the config");
                return;
            }
        };
    }

    // If an additional mensa is specified, use it
    if let Some(mensa_num) = additional_mensa {
        match config.mensa_list() {
            Some(list) => {
                // Check if the index is valid (1-based index)
                if *mensa_num < 1 || (*mensa_num as usize) > list.len() {
                    println!("Invalid mensa number specified.");
                    return;
                }
            }
            None => {
                println!("No additional mensas configured.");
                return;
            }
        }

        let mensa_list = config.mensa_list().as_ref().expect("No additional mensas configured.");
        mensa_name = mensa_list
            .get((*mensa_num - 1) as usize)
            .expect("Failed to get mensa name from list")
            .as_str();
    }

    // Find the food for the specified date
    let food_for_date = match get_food_for_date(date_to_use, mensa_name) {
        Ok(food) => food,
        Err(e) => {
            println!(
                "Error fetching food for mensa '{}' on date '{}': {}",
                mensa_name, date_to_use, e
            );
            return;
        }
    };

    // If json option is set, print the food in JSON format
    if cli.json {
        println!("{}", serde_json::to_string(&food_for_date).unwrap());
        return;
    }

    // output formatted date and food items
    println!("{}\n{}", mensa_name, date_to_use.format("%Y-%m-%d"));
    for food in food_for_date {
        println!("\n{}", food);
    }

    // Show options for additional mensas
    if let Some(additional_mensas) = config.mensa_list() {
        print!("\n---------\n\nAdditional Mensas (use argument --number <index> to select):\n");
        for (i, mensa) in additional_mensas.iter().enumerate() {
            print!("- {}: {}\n", i + 1, mensa);
        }
    } else {
        println!("\nNo additional mensas configured.");
    }
}
