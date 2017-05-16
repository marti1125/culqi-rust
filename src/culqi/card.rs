extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Card {
}

impl Card {

    pub fn new<S: Into<String>>(
        customer_id: S,
        token_id: S,
        validate: Option<bool>,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("customer_id".to_string(), json!(customer_id.into()));
        map.insert("customer_id".to_string(), json!(token_id.into()));
        if !validate.is_none() {
            map.insert("validate".to_string(), json!(validate));
        }
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, card: &HashMap<String, serde_json::Value>) -> String {
        client.post("/cards", card)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/cards/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/cards/{}", id))
    }

    pub fn all(
        client: &Client,
        creation_date: Option<String>,
        creation_date_from: Option<String>,
        creation_date_to: Option<String>,
        card_brand: Option<String>,
        card_type: Option<String>,
        device_type: Option<String>,
        bin: Option<String>,
        country_code: Option<String>,
        limit: Option<String>
    ) -> String {
        let mut query_url = String::from("/cards");
        query_url.push_str("?limit");
        if limit.is_none() {
            query_url.push_str("=50");
        } else {
            query_url.push_str(&format!("={:?}", limit));
        }
        if !creation_date.is_none() {
            query_url.push_str("&creation_date");
            query_url.push_str(&format!("={:?}", creation_date));
        }
        if !creation_date_from.is_none() {
            query_url.push_str("&creation_date_from");
            query_url.push_str(&format!("={:?}", creation_date_from));
        }
        if !creation_date_to.is_none() {
            query_url.push_str("&creation_date_to");
            query_url.push_str(&format!("={:?}", creation_date_to));
        }
        if !card_brand.is_none() {
            query_url.push_str("&card_brand");
            query_url.push_str(&format!("={:?}", card_brand));
        }
        if !card_type.is_none() {
            query_url.push_str("&card_type");
            query_url.push_str(&format!("={:?}", card_type));
        }
        if !device_type.is_none() {
            query_url.push_str("&device_type");
            query_url.push_str(&format!("={:?}", device_type));
        }
        if !bin.is_none() {
            query_url.push_str("&bin");
            query_url.push_str(&format!("={:?}", bin));
        }
        if !country_code.is_none() {
            query_url.push_str("&country_code");
            query_url.push_str(&format!("={:?}", country_code));
        }
        client.get(&query_url)
    }

}
