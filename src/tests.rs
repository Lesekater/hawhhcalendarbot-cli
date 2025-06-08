#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use chrono::NaiveDate;

    use crate::mensa::mensa_data::load_local_data;
    use crate::mensa::meal::{Meal, Prices, Contents};
    use crate::cmd::mensa::{filter_food_by_extras, filter_food_by_extras_single};
    use crate::config_managment::Extras;

    fn standard_meal() -> Meal {
        Meal {
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

    #[test]
    fn test_load_local_data() {
        // arrange
        let test_meal = standard_meal();

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
        // arrange
        let meal = standard_meal();
        let mut meal2 = standard_meal();
        meal2.name = "Nicht veganes Gericht".to_string();
        meal2.contents.alcohol = true;
        meal2.contents.beef = true;
        meal2.contents.vegetarian = false;
        meal2.contents.vegan = false;
        let extras = vec![Extras::Vegan, Extras::Vegetarisch];

        // act
        let filtered_meals = filter_food_by_extras(vec![meal, meal2], &extras);

        // assert
        assert_eq!(filtered_meals.len(), 1, "Filtered meals count does not match expected");
    }

    #[test]
    fn test_filter_food_by_extras_single() {
        // arrange
        let meal = standard_meal();
        let extras = vec![Extras::Vegan];
        let result = filter_food_by_extras_single(&meal, &extras);
        assert!(result, "Meal should match the vegan extra filter");
        let non_matching_extras = vec![Extras::AlcoholFree];

        // act
        let result_non_matching = filter_food_by_extras_single(&meal, &non_matching_extras);

        // assert
        assert!(!result_non_matching, "Meal should not match the alcohol-free extra filter");
    }

    #[test]
    fn test_empty_extras() {
        // arrange
        let meal = standard_meal();
        let extras = vec![];

        // act
        let result = filter_food_by_extras_single(&meal, &extras);

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
        let extras = vec![Extras::FishFree];

        // act
        let result = filter_food_by_extras_single(&meal, &extras);

        // assert
        assert!(!result, "Meal should not match the fish-free extra filter");
    }
}