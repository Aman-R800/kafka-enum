use once_cell::sync::Lazy;
use serde_json::Value;
    
static JSON_DATA: Lazy<Value> = Lazy::new(|| {
    let url = std::env::var("CONFIG_URL").unwrap();
    let body = reqwest::blocking::get(&url)
                .unwrap()
                .text()
                .unwrap();
    
    serde_json::from_str(&body).unwrap()
});

pub fn get_json() -> Value {
    JSON_DATA.clone()
}

pub fn get_topics(json: &Value) -> Vec<String> {
    let variants: Vec<String> = {
        match json {
            Value::Object(a) => {
                a.keys()
                    .map(|i| {
                        i.to_string()
                    })
                    .collect()
            }

            _ => Vec::new()
        }
    };

    variants
}


pub fn get_topics_str(json: &Value) -> Vec<String> {
    let variants_str: Vec<String> = {
        match json {
            Value::Object(a) => {
                a.values()
                    .filter(|i| {
                        if let Value::String(_s) = i {
                            return true;
                        }

                        false
                    })
                    .map(|i| {
                        match i {
                            Value::String(s) => {
                                s.to_string()
                            }

                            _ => "Not possible".to_string()
                        }
                    })
                    .collect()
            }

            _ => Vec::new()
        }
    };

    variants_str
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_get_topics() {
        // Test case with valid object
        let json_data: Value = json!({
            "Glossary": "glossary",
            "FeatureAsset": "feature_asset",
            "EditGame": "edit_game"
        });

        let result = get_topics(&json_data);
        let expected = vec![
            "EditGame".to_string(),
            "FeatureAsset".to_string(),
            "Glossary".to_string(),
        ];

        assert_eq!(result, expected);

        // Test case with empty object
        let json_data_empty: Value = json!({});
        let result_empty = get_topics(&json_data_empty);
        assert_eq!(result_empty, Vec::<String>::new());

        // Test case with non-object
        let json_data_non_object: Value = json!("not an object");
        let result_non_object = get_topics(&json_data_non_object);
        assert_eq!(result_non_object, Vec::<String>::new());
    }

    #[test]
    fn test_get_topics_str() {
        // Test case with valid object where all values are strings
        let json_data: Value = json!({
            "Glossary": "glossary",
            "FeatureAsset": "feature_asset",
            "EditGame": "edit_game"
        });

        let result = get_topics_str(&json_data);
        let expected = vec![
            "edit_game".to_string(),
            "feature_asset".to_string(),
            "glossary".to_string(),
        ];

        assert_eq!(result, expected);

        // Test case with mixed values (not all are strings)
        let json_data_mixed: Value = json!({
            "Glossary": "glossary",
            "FeatureAsset": 123,  // Not a string
            "EditGame": "edit_game"
        });

        let result_mixed = get_topics_str(&json_data_mixed);
        let expected_mixed = vec![
            "edit_game".to_string(),  // Only string values are included
            "glossary".to_string(),
        ];

        assert_eq!(result_mixed, expected_mixed);

        // Test case with empty object
        let json_data_empty: Value = json!({});
        let result_empty = get_topics_str(&json_data_empty);
        assert_eq!(result_empty, Vec::<String>::new());

        // Test case with non-object
        let json_data_non_object: Value = json!("not an object");
        let result_non_object = get_topics_str(&json_data_non_object);
        assert_eq!(result_non_object, Vec::<String>::new());
    }
}
