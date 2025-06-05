#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use chrono::NaiveDate;

    use crate::mensa_data::load_local_data;
    use crate::meal::{Meal, Prices, Contents};
    use crate::mensa_commands::{filter_food_by_extras, filter_food_by_extras_single};
    use crate::config_managment::Extras;

    #[test]
    fn test_load_local_data() {
        // arrange
        let test_meal = Meal {
            name: "Testgericht".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            category: "Hauptgericht".to_string(),
            additives: BTreeMap::new(),
            prices: Prices {
                price_attendant: 5.0,
                price_guest: 6.0,
                price_student: 4.0,
            },
            contents: Contents{
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
        };

        let test_path = PathBuf::from("./test_data");

        let mut file = File::create(test_path.join("mensadata/timestamp")).unwrap();
        file.write_all(&chrono::Local::now().timestamp().to_string().into_bytes()).expect("couldnt write timestamp");

        // act
        let result = load_local_data(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "TestMensa", test_path.clone());

        // assert
        assert!(result.is_ok(), "Failed to load local data: {:?}", result.err());
        let data = result.unwrap();
        assert_eq!(data, vec![test_meal], "Loaded data does not match expected data");

        // Clean up
        let _ = std::fs::remove_file(test_path.join("mensadata/timestamp"));
    }

    #[test]
    fn test_filter_food_by_extras() {
        let meal = Meal {
            name: "Testgericht".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            category: "Hauptgericht".to_string(),
            additives: BTreeMap::new(),
            prices: Prices {
                price_attendant: 5.0,
                price_guest: 6.0,
                price_student: 4.0,
            },
            contents: Contents{
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
        };

        let meal2 = Meal {
            name: "Nicht veganes Gericht".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            category: "Hauptgericht".to_string(),
            additives: BTreeMap::new(),
            prices: Prices {
                price_attendant: 5.0,
                price_guest: 6.0,
                price_student: 4.0,
            },
            contents: Contents{
                alcohol: true,
                beef: true,
                fish: false,
                game: false,
                gelatine: false,
                vegetarian: false,
                vegan: false,
                lactose_free: false,
                lamb: false,
                pig: false,
                poultry: false,
            },
        };

        let extras = vec![Extras::Vegan, Extras::Vegetarisch];
        let filtered_meals = filter_food_by_extras(vec![meal, meal2], &extras);

        assert_eq!(filtered_meals.len(), 1, "Filtered meals count does not match expected");
    }

    #[test]
    fn test_filter_food_by_extras_single() {
        let meal = Meal {
            name: "Testgericht".to_string(),
            date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
            category: "Hauptgericht".to_string(),
            additives: BTreeMap::new(),
            prices: Prices {
                price_attendant: 5.0,
                price_guest: 6.0,
                price_student: 4.0,
            },
            contents: Contents{
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
        };

        let extras = vec![Extras::Vegan];
        let result = filter_food_by_extras_single(&meal, &extras);
        assert!(result, "Meal should match the vegan extra filter");

        let non_matching_extras = vec![Extras::AlcoholFree];
        let result_non_matching = filter_food_by_extras_single(&meal, &non_matching_extras);
        assert!(!result_non_matching, "Meal should not match the alcohol-free extra filter");
    }
}