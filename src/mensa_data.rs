pub mod mensa_data {
    use std::fs;
    use std::path::Path;
    use std::error::Error;
    use std::process::Command;
    use std::collections::HashMap;

    /// Represents the mensa data
    /// <mensa-name>/<year>/<month>/<day>.json
    type MensaData = HashMap<String, String>;

    pub fn load_local_data() -> Result<MensaData, Box<dyn Error>> {
        // Check locally if the data is available
        let path = Path::new("./data/mensadata/");
        
        if !path.exists() {
            // If the path does not exist, return an error
            return Err(format!("Local mensa data not available at {:?}", path).into());
        }

        // If the path exists, read the data from the directory
        let mut data: MensaData = HashMap::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            if !entry.path().is_dir() || entry.path().file_name().unwrap() == "mensas" || entry.path().file_name().unwrap() == ".git" {
                // If the entry is not a directory, skip it
                continue;
            }

            let mensa_name = entry.file_name().into_string().unwrap();
            let mensa_path = entry.path();

            data.insert(mensa_name.clone(), mensa_path.to_str().unwrap().to_string());

            // TODO: Read and parse the JSON files in the mensa directory
        }

        // Return the mensa data
        Ok(data)
    }
    
    pub fn fetch_mensa_data() -> Result<(), Box<dyn Error>> {
        // Fetch Mensa data from git repo (https://github.com/HAWHHCalendarBot/mensa-data.git) and save it locally

        // Create the mensa data directory if it doesn't exist
        if !Path::new("./data/mensadata/").exists() {
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
            Ok(())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to clone mensa data: {}", error_message).into());
        }
    }
}