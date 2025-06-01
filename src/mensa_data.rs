pub mod mensa_data {
    use std::fs;
    use std::path::Path;
    use std::error::Error;
    use std::process::Command;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;

    use crate::meal::Meal;

    /// Represents the mensa data
    /// <mensa-name>/<year>/<month>/<day>.json
    pub type MensaData = HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Meal>>>>>;

    pub fn load_local_data() -> Result<MensaData, Box<dyn Error>> {
        // Check locally if the data is available
        let path = Path::new("./data/mensadata/");
        
        if !path.exists() {
            // If the path does not exist, return an error
            return Err(format!("Local mensa data not available at {:?}", path).into());
        }

        let currentdate = chrono::Local::now().date_naive();
        
        let mut file = File::open("./data/mensadata/timestamp")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let last_change = chrono::DateTime::from_timestamp(contents.parse()?, 0).unwrap();
        let last_change_date = last_change.date_naive();
        
        if currentdate.signed_duration_since(last_change_date) > chrono::Duration::days(1) {
            // If the data is older than 1 day, fetch new data
            println!("Local mensa data is outdated. Fetching new data...");
            fetch_mensa_data()?;
        }

        // Read data
        let mut data: MensaData = HashMap::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            if !entry.path().is_dir() || entry.path().file_name().unwrap() == "mensas" || entry.path().file_name().unwrap() == ".git" {
                // If the entry is not a directory, skip it
                continue;
            }

            let mensa_name = entry.file_name().into_string().unwrap();
            let mensa_path = entry.path();

            let mut folder_to_process = vec![mensa_path.clone()];

            while let Some(folder) = folder_to_process.pop() {
                // Read the contents of the current directory
                for entry in fs::read_dir(&folder)? {
                    let entry = entry?;

                    if entry.path().is_dir() {
                        // If the entry is a directory, add it to the list of folders to process
                        folder_to_process.push(entry.path());
                    } else if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                        // If the entry is a JSON file, read it and add it to the data
                        let entry_path = entry.path();
                        let file_path = entry_path.to_str().ok_or_else(|| format!("Invalid file path - entry: {:?}", entry_path))?;
                        let file_content = fs::read_to_string(&entry_path)?;
                        let parts = file_path
                            .strip_suffix(".json")
                            .unwrap()
                            .split('/')
                            .rev()
                            .take(3)
                            .collect::<Vec<&str>>();
                        if parts.len() != 3 {
                            return Err(format!("Invalid file path structure: {}", file_path).into());
                        }
                        let day = parts[0].to_string();
                        let month = parts[1].to_string();
                        let year = parts[2].to_string();

                        // Insert or use existing nested maps to avoid overwriting existing data
                        let mensa_entry = data.entry(mensa_name.clone()).or_insert_with(HashMap::new);
                        let year_entry = mensa_entry.entry(year.clone()).or_insert_with(HashMap::new);
                        let month_entry = year_entry.entry(month.clone()).or_insert_with(HashMap::new);
                        
                        let entry = serde_json::from_str::<Vec<Meal>>(&file_content)
                            .map_err(|e| format!("Failed to parse JSON for {}: {}", file_path, e))?;

                        month_entry.insert(day, entry);
                    }
                }
            }
        }

        // Return the mensa data
        Ok(data)
    }

    pub fn get_food_for_date<'a>(data: &'a MensaData, date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<&'a Meal>, Box<dyn Error>> {
        Ok(
            data.get(mensa_name)
                .and_then(|mensa| mensa.get(&date.format("%Y").to_string()))
                .and_then(|year| year.get(&date.format("%m").to_string()))
                .and_then(|month| month.get(&date.format("%d").to_string()))
                .ok_or_else(|| "No data found for the given date")?
                .iter()
                .collect::<Vec<&Meal>>(),
        )
    }

    pub fn fetch_mensa_data() -> Result<(), Box<dyn Error>> {
        // Fetch Mensa data from git repo (https://github.com/HAWHHCalendarBot/mensa-data.git) and save it locally

        // Create the mensa data directory if it doesn't exist
        if !Path::new("./data/mensadata/").exists() {
            fs::create_dir_all("./data/mensadata/")?;
        } else {
            // If the directory already exists, remove it to ensure a fresh clone
            fs::remove_dir_all("./data/mensadata/")?;
            fs::create_dir_all("./data/mensadata/")?;
        }

        // Clone the mensa data repository
        let output = Command::new("git")
            .arg("clone")
            .arg("https://github.com/HAWHHCalendarBot/mensa-data.git")
            .arg("./data/mensadata/")
            .output()?;        

        if output.status.success() {
            // If the clone was successful, return Ok
            println!("Mensa data cloned successfully.");

            // Refresh Timestamp
            let mut file = File::create("./data/mensadata/timestamp")?;
            file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

            Ok(())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clone mensa data: {}", error_message).into());
        }
    }
}