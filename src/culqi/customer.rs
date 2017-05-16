extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Customer {
}

impl Customer {

    pub fn new<S: Into<String>>(
        first_name: S,
        last_name: S,
        email: S,
        address: S,
        address_city: S,
        country_code: S,
        phone_number: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("first_name".to_string(), json!(first_name.into()));
        map.insert("last_name".to_string(), json!(last_name.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("address".to_string(), json!(address.into()));
        map.insert("address_city".to_string(), json!(address_city.into()));
        map.insert("country_code".to_string(), json!(country_code.into()));
        map.insert("phone_number".to_string(), json!(phone_number.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, customer: &HashMap<String, serde_json::Value>) -> String {
        client.post("/customers", customer)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/customers/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/customers/{}", id))
    }

    pub fn all(
        client: &Client,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        address: Option<String>,
        address_city: Option<String>,
        phone_number: Option<String>,
        country_code: Option<String>,
        limit: Option<String>
    ) -> String {
        let mut query_url = String::from("/customers");
        query_url.push_str("?limit");
        if limit.is_none() {
            query_url.push_str("=50");
        } else {
            query_url.push_str(&format!("={:?}", limit));
        }
        if !first_name.is_none() {
            query_url.push_str("&first_name");
            query_url.push_str(&format!("={:?}", first_name));
        }
        if !last_name.is_none() {
            query_url.push_str("&last_name");
            query_url.push_str(&format!("={:?}", last_name));
        }
        if !email.is_none() {
            query_url.push_str("&email");
            query_url.push_str(&format!("={:?}", email));
        }
        if !address.is_none() {
            query_url.push_str("&address");
            query_url.push_str(&format!("={:?}", address));
        }
        if !address_city.is_none() {
            query_url.push_str("&address_city");
            query_url.push_str(&format!("={:?}", address_city));
        }
        if !phone_number.is_none() {
            query_url.push_str("&phone_number");
            query_url.push_str(&format!("={:?}", phone_number));
        }
        if !country_code.is_none() {
            query_url.push_str("&country_code");
            query_url.push_str(&format!("={:?}", country_code));
        }
        client.get(&query_url)
    }

}
