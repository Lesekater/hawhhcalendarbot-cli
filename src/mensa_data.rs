pub mod mensa_data {
    use std::fs;
    use std::path::Path;
    use std::error::Error;
    use std::process::Command;
    use std::fs::File;
    use std::io::prelude::*;
    use reqwest::blocking as reqwest;

    use crate::meal::Meal;

    ///////////////////////////////////////////////////////////////////////////
    ///////////                 Local Loading
    ///////////////////////////////////////////////////////////////////////////

    pub fn load_local_data(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Meal>, Box<dyn Error>> {
        // Check locally if the data is available
        let path = Path::new("./data/mensadata/");
        
        // If the path does not exist, return an error
        if !path.exists() {
            return Err(format!("Local mensa data not available at {:?}", path).into());
        }
        
        // Read timestamp
        let mut file: File = File::open("./data/mensadata/timestamp")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let last_change = chrono::DateTime::from_timestamp(contents.parse()?, 0).unwrap();
        let last_change_date = last_change.date_naive();
        
        // If the data is older than 1 day, fetch new data
        if chrono::Local::now().date_naive().signed_duration_since(last_change_date) > chrono::Duration::days(1) {
            println!("Local mensa data is outdated. Fetching new data...");
            fetch_mensa_data()?;
        }

        // Load new data
        let path_str = format!("./data/mensadata/{}/{}/{}/{}.json",
            &mensa_name,
            &date.format("%Y").to_string(),
            &date.format("%m").to_string(),
            &date.format("%d").to_string()
        );
        let path = Path::new(&path_str);

        // If the path does not exist, return an error
        if !path.exists() {
            return Err(format!("No data found for mensa '{}' on date '{}'", &mensa_name, date).into());
        }

        // Read data
        let file_content = fs::read_to_string(&path)?;
        
        Ok(serde_json::from_str(&file_content)?)
    }

    pub fn get_food_for_date(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Meal>, Box<dyn Error>> {

        // Check if the mensa data is available locally
        // -> if so, load it
        // -> else load for single date directly
        load_local_data(date, mensa_name).or_else(|_| fetch_data_for_date(date, mensa_name))
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////                 Fetching Mensa Data   
    ///////////////////////////////////////////////////////////////////////////

    /// Fetches data for a single date
    pub fn fetch_data_for_date(date: chrono::NaiveDate, mensa_name: &str) -> Result<Vec<Meal>, Box<dyn Error>> {
        // https://raw.githubusercontent.com/HAWHHCalendarBot/mensa-data/refs/heads/main/Caf%C3%A9%20Mittelweg/2022/02/02.json
        let url = format!("https://raw.githubusercontent.com/HAWHHCalendarBot/mensa-data/refs/heads/main/{}/{}/{}/{}.json",
            &mensa_name,
            &date.format("%Y"),
            &date.format("%m"),
            &date.format("%d")
        );

        let data = reqwest::get(url)?.text()?;

        return Ok(serde_json::from_str(&data)?);
    }

    /// Fetches Mensadata and stores it in the cache dir
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
            .arg("--depth")
            .arg("1")
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