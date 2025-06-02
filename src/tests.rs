#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use chrono::NaiveDate;

    use crate::mensa_data::mensa_data::{get_food_for_date, load_local_data, MensaData};
    use crate::meal::{Meal, Prices, Contents};

    #[test]
    fn test_get_food_for_date() {
        let mut data = MensaData::new();
        let test_meal1 = Meal { name: "Meal1".to_string(), date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(), category: "".to_string(), additives: BTreeMap::new(), prices: Prices { price_attendant: 5.0, price_guest: 6.0, price_student: 4.0 }, contents: Contents::default() };
        let test_meal2 = Meal { name: "Meal2".to_string(), date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(), category: "".to_string(), additives: BTreeMap::new(), prices: Prices { price_attendant: 5.0, price_guest: 6.0, price_student: 4.0 }, contents: Contents::default() };
        data.insert(
            "TestMensa".to_string(),
            vec![
                (
                    "2023".to_string(),
                    vec![
                        (
                            "10".to_string(),
                            vec![
                                ("01".to_string(), vec![test_meal1, test_meal2]),
                            ].into_iter().collect(),
                        ),
                    ].into_iter().collect(),
                ),
            ].into_iter().collect(),
        );
        

        let date = chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
        let mensa_name = "TestMensa";

        let meals = get_food_for_date(&data, date, mensa_name).unwrap();
        assert_eq!(meals.len(), 2);
        assert_eq!(meals[0].name, "Meal1");
        assert_eq!(meals[1].name, "Meal2");
    }

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

        let result = load_local_data();

        assert!(result.is_ok(), "Failed to load local data: {:?}", result.err());
        let data = result.unwrap();
        assert_eq!(data.get("TestMensa").unwrap().get("2025").unwrap().get("06").unwrap().get("01").unwrap(), &vec![test_meal], "Loaded data does not match the expected test meal");
    }
}