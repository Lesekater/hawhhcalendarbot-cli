use crate::cmd::mensa_settings;
use crate::mensa::meal::Meal;
use crate::mensa::haw_meal::HawMeal;
use crate::json_parser::Config;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    command: Option<MensaCommands>,

    #[arg(short, long, global = true)]
    number: Option<i32>,

    #[arg(short, long, global = true)]
    json: bool,
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
    Settings(mensa_settings::Cmd),
}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let handle = HawMeal::update_mensa_data();

        let currentdate = chrono::Local::now().date_naive();

        match self.command {
            // Date commands
            Some(MensaCommands::Today { .. })
            | Some(MensaCommands::Tomorrow { .. })
            | Some(MensaCommands::Date { .. }) => {
                self.date_command(&self.command, currentdate);
            }
            // Update/ Cache commands
            Some(MensaCommands::Update) | Some(MensaCommands::Cache) => {
                println!("Updating mensa data...");
                let cache_dir = dirs::cache_dir().expect("Could not find cache directory");
                match HawMeal::fetch_mensa_data(&cache_dir) {
                    Ok(_) => println!("Mensa data updated successfully."),
                    Err(e) => println!("Error updating mensa data: {}", e),
                };
            }
            // Settings command
            Some(MensaCommands::Settings(cmd)) => cmd.run()?,
            // Default case for today if no command is specified
            None => {
                self.date_command(&Some(MensaCommands::Today { number: self.number }), currentdate);
            }
        }

        if !handle.is_finished() {
            println!("\nWaiting for mensa data update to finish...");
            let _ = handle.join();
        }

        Ok(())
    }

    fn date_command(&self, command: &Option<MensaCommands>, currentdate: chrono::NaiveDate) {
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
        let mut config = Config::load_config();
        let mut mensa_name = String::new();

        // Check primary mensa
        if additional_mensa.is_none() {
            mensa_name = match config.get_primary_mensa() {
                Some(name) => name,
                None => {
                    println!("Primary Mensa is not set - please set it in the config");
                    return;
                }
            };
        }

        // If an additional mensa is specified, use it
        if let Some(mensa_num) = additional_mensa {
            match config.get_mensa_list() {
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

            let mensa_list = match config.get_mensa_list() {
                Some(list) => list,
                None => {
                    println!("No additional mensas configured.");
                    return;
            }};
            mensa_name = mensa_list
                .get((*mensa_num - 1) as usize)
                .expect("Failed to get mensa name from list")
                .clone();
        }

        // Find the food for the specified date
        let food_for_date:Vec<HawMeal> = match Meal::get_food_for_date(date_to_use, mensa_name.as_str()) {
            Ok(food) => food,
            Err(e) => {
                println!(
                    "Error fetching food for mensa '{}' on date '{}': {}",
                    mensa_name, date_to_use, e
                );
                return;
            }
        };

        // Filter food items based on extras
        let food_for_date = match config.get_extras() {
            Some(extras) => Meal::filter_food_by_extras(food_for_date, extras),
            None => food_for_date,
        };

        // If json option is set, print the food in JSON format
        if self.json {
            println!("{}", serde_json::to_string(&food_for_date).unwrap());
            return;
        }

        // output formatted date and food items
        println!("{}\n{}", &mensa_name, date_to_use.format("%Y-%m-%d"));

        // Print each food item
        for food in food_for_date {
            println!("\n{}", food);
        }

        // Show options for additional mensas
        if let Some(mensa_list) = config.get_mensa_list() {
            if !mensa_list.is_empty() {
                println!("\n---------\n\nAdditional Mensas (use argument --number <index> to select):");
                for (i, mensa) in mensa_list.iter().enumerate() {
                    println!("- {}: {}", i + 1, mensa);
                }
            }
        }

    }
}