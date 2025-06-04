#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use chrono::NaiveDate;

    use crate::mensa_data::mensa_data::load_local_data;
    use crate::meal::{Meal, Prices, Contents};

    #[test]
    fn test_load_local_data() {
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

        let result = load_local_data(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(), "TestMensa");

        assert!(result.is_ok(), "Failed to load local data: {:?}", result.err());
        let data = result.unwrap();
        assert_eq!(data, vec![test_meal], "Loaded data does not match expected data");
    }
}