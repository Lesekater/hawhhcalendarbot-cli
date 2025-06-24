#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use chrono::NaiveDate;
    use tempfile::tempdir;

    use crate::mensa::test_meal::TestMeal;
    use crate::mensa::meal::{Contents, Meal, Prices};
    use crate::mensa::haw_meal::HawMeal;
    use crate::json_parser::Extras;
use std::fs;
use std::io;

    fn standard_meal() -> HawMeal {
        HawMeal {
            name: "Testgericht".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            category: "Hauptgericht".to_string(),
            additives: BTreeMap::new(),
            prices: Prices {
                price_attendant: 5.0,
                price_guest: 6.0,
                price_student: 4.0,
            },
            contents: Contents {
                alcohol: false,
                beef: false,
                fish: false,
                game: false,
                gelatine: false,
                vegetarian: true,
                vegan: true,
                lactose_free: false,
                lamb: false,
                pig: false,
                poultry: false,
            },
        }
    }

    fn test_meal() -> TestMeal {
        TestMeal {
            title: "Königsberger Klopse mit Kartoffeln und Erbsen".to_string(),
            description: "Hackfleischbällchen in einer würzigen Sauce, serviert mit Kartoffeln und Erbsen.".to_string(),
            price: 4.50,
            category: "Hauptgericht".to_string(),
            date: NaiveDate::parse_from_str("09-06-2025", "%d-%m-%Y").unwrap(),
            additives: vec!["A".to_string(), "C".to_string(), "D".to_string()],
        }
    }

    #[test]
    fn test_load_local_data() {
        // arrange
        let test_meal = standard_meal();

        let test_path = PathBuf::from("./test_data");

        let mut file = File::create(test_path.join("mensadata/timestamp")).unwrap();
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

        // act
        let result = HawMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "TestMensa", test_path.clone());

        // assert
        assert!(result.is_ok(), "Failed to load local data: {:?}", result.err());
        let data = result.unwrap();
        assert_eq!(data, vec![test_meal], "Loaded data does not match expected data");

        // Clean up
        let _ = std::fs::remove_file(test_path.join("mensadata/timestamp"));
    }

    #[test]
    fn test_load_local_data_invalid_date() {
        // arrange
        let test_path = PathBuf::from("./test_data");

        // act
        let result = HawMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(), "TestMensa", test_path.clone());

        // assert
        assert!(result.is_err(), "Expected error when loading local data with invalid date");
    }

    #[test]
    fn test_load_local_data_no_data() {
        // arrange
        let invalid_path = PathBuf::from("./non_existent_data");

        // act
        let result = HawMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "NonExistentMensa", invalid_path);

        // assert
        assert!(result.is_err(), "Expected error when loading local data for non-existent mensa");
    }

    #[test]
    fn test_filter_food_by_extras() {
        // arrange
        let meal = standard_meal();
        let mut meal2 = standard_meal();
        meal2.name = "Nicht veganes Gericht".to_string();
        meal2.contents.alcohol = true;
        meal2.contents.beef = true;
        meal2.contents.vegetarian = false;
        meal2.contents.vegan = false;
        let extras = vec![Extras::Vegan, Extras::Vegetarian];

        // act
        let filtered_meals = Meal::filter_food_by_extras(vec![meal, meal2], &extras);

        // assert
        assert_eq!(filtered_meals.len(), 1, "Filtered meals count does not match expected");
    }

    #[test]
    fn test_filter_food_by_extras_single() {
        // arrange
        let meal = standard_meal();
        let extras = vec![Extras::Vegan];
        let result = Meal::filter_food_by_extras_single(&meal, &extras);
        assert!(result, "Meal should match the vegan extra filter");
        let non_matching_extras = vec![Extras::Alcohol];

        // act
        let result_non_matching = Meal::filter_food_by_extras_single(&meal, &non_matching_extras);

        // assert
        assert!(!result_non_matching, "Meal should not match the alcohol extra filter");
    }

    #[test]
    fn test_empty_extras() {
        // arrange
        let meal = standard_meal();
        let extras = vec![];

        // act
        let result = Meal::filter_food_by_extras_single(&meal, &extras);

        // assert
        assert!(result, "Meal should match when no extras are specified");
    }

    #[test]
    fn test_negative_extras() {
        // arrange
        let mut meal = standard_meal();
        meal.contents.alcohol = true;
        meal.contents.fish = true;
        meal.contents.vegetarian = false;
        meal.contents.vegan = false;
        let extras = vec![Extras::Fish];

        // act
        let result = Meal::filter_food_by_extras_single(&meal, &extras);

        // assert
        assert!(!result, "Meal should not match the fish-free extra filter");
    }

    ///////// Test data format
    
    #[test]
    fn test_load_local_data_testdata() {
        // arrange
        let test_meal = test_meal();
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path();
        let dst = copy_testdata_into(test_path);
        let mut file = File::create(dst.join("timestamp")).unwrap();
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");
        // act
        let result = TestMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "TestMensa", test_path.to_path_buf());
        // assert
        assert!(result.is_ok(), "Failed to load local data: {:?}", result.err());
        let data = result.unwrap();
        assert_eq!(data, vec![test_meal], "Loaded data does not match expected data");
    }
    
    #[test]
    fn test_load_local_data_invalid_date_testdata() {
        // arrange
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path();
        // copy test data to temp directory
        copy_testdata_into(test_path);
        // act
        let result = TestMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(), "TestMensa", test_path.to_path_buf());
        // assert
        assert!(result.is_err(), "Expected error when loading local data with invalid date");
    }

    #[test]
    fn test_load_local_data_no_data_testdata() {
        // arrange
        let temp_dir = tempdir().unwrap();
        let invalid_path = temp_dir.path().join("non_existent_data");
        // act
        let result = TestMeal::load_from_local(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "NonExistentMensa", invalid_path);
        // assert
        assert!(result.is_err(), "Expected error when loading local data for non-existent mensa");
    }

    fn copy_testdata_into(test_path: &std::path::Path) -> PathBuf {
        fn copy_recursively(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
            fs::create_dir_all(dst)?;
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                if entry.file_type()?.is_dir() {
                    copy_recursively(&src_path, &dst_path)?;
                } else {
                    fs::copy(&src_path, &dst_path)?;
                }
            }
            Ok(())
        }

        let src = PathBuf::from("./test_data/mensadata");
        let dst = test_path.join("mensadata");
        copy_recursively(&src, &dst).unwrap_or_else(|e| panic!("Error copying test data: {}", e));
        dst
    }
}