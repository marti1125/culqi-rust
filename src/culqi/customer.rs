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

}
