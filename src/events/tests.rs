#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    use chrono::{NaiveDateTime};
    use tempfile::tempdir;

    use crate::events::event::{Event, EventMeta};
    use crate::events::haw_event::HawEventEntry;

    fn standard_event() -> HawEventEntry {
        HawEventEntry {
            name: "Test Event".to_string(),
            location: "BT101".to_string(),
            description: "Test description for event".to_string(),
            start: NaiveDateTime::parse_from_str("2025-06-01 08:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            end: NaiveDateTime::parse_from_str("2025-06-01 09:45:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }

    fn standard_event_meta() -> EventMeta {
        EventMeta {
            department: "test-department".to_string(),
            module: "test-module".to_string(),
        }
    }

    #[test]
    fn test_load_local_data_no_data() {
        // arrange
        let temp_dir = tempdir().unwrap();
        let invalid_path = temp_dir.path().join("non_existent_data");
        let event_meta = standard_event_meta();

        // act
        let result = HawEventEntry::load_from_local(&event_meta, invalid_path);

        // assert
        assert!(result.is_err(), "Expected error when loading local data for non-existent event data");
    }

    #[test]
    fn test_load_local_data_with_test_data() {
        // arrange
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path();
        let eventdata_dir = create_test_eventdata(test_path);
        let event_meta = EventMeta {
            department: "test-department".to_string(),
            module: "test-module".to_string(),
        };

        // Create timestamp file
        let mut file = File::create(eventdata_dir.join("timestamp")).unwrap();
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes())
            .expect("couldn't write timestamp");

        // act
        let result = HawEventEntry::load_from_local(&event_meta, test_path.to_path_buf());

        // assert
        assert!(result.is_ok(), "Failed to load local event data: {:?}", result.err());
        let events = result.unwrap();
        assert_eq!(events, vec![standard_event()], "Expected to find the standard test event");
    }

    #[test]
    fn test_load_local_data_outdated_timestamp() {
        // arrange
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path();
        let eventdata_dir = create_test_eventdata(test_path);
        let event_meta = standard_event_meta();

        // Create old timestamp (more than 1 day ago)
        let old_timestamp = chrono::Local::now().timestamp() - (2 * 24 * 60 * 60); // 2 days ago
        let mut file = File::create(eventdata_dir.join("timestamp")).unwrap();
        file.write_all(&old_timestamp.to_string().into_bytes())
            .expect("couldn't write timestamp");

        // act
        let result = HawEventEntry::load_from_local(&event_meta, test_path.to_path_buf());

        // assert
        assert!(result.is_err(), "Expected error due to outdated data and network fetch failure");
    }

    fn create_test_eventdata(test_path: &std::path::Path) -> PathBuf {
        let eventdata_dir = test_path.join("eventdata");
        let department_dir = eventdata_dir.join("test-department");
        
        fs::create_dir_all(&department_dir).unwrap();

        // Create test event JSON file
        let test_events = vec![standard_event()];
        let json_content = serde_json::to_string(&test_events).unwrap();
        
        let module_file = department_dir.join("test-module.json");
        fs::write(module_file, json_content).unwrap();

        eventdata_dir
    }
}