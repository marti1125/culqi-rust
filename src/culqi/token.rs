extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
}

impl Token {

    pub fn new<S: Into<String>>(
        card_number: S,
        cvv: S,
        expiration_month: i32,
        expiration_year: i32,
        email: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("card_number".to_string(), json!(card_number.into()));
        map.insert("cvv".to_string(), json!(cvv.into()));
        map.insert("expiration_month".to_string(), json!(expiration_month));
        map.insert("expiration_year".to_string(), json!(expiration_year));
        map.insert("email".to_string(), json!(email.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, token: &HashMap<String, serde_json::Value>) -> String {
        client.post("/tokens", token)
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/tokens/{}", id))
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
        let mut query_url = String::from("/tokens");
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
