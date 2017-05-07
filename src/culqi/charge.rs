extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Charge {
}

impl Charge {

    pub fn new<S: Into<String>>(
        amount: S,
        currency_code: S,
        email: S,
        installments: i32,
        source_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("amount".to_string(), json!(amount.into()));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("installments".to_string(), json!(installments));
        map.insert("source_id".to_string(), json!(source_id.into()));
        return map;
    }

    pub fn create(client: &Client, charge: &HashMap<String, serde_json::Value>) -> String {
        client.post("/charges", charge)
    }

    pub fn capture(client: &Client, id: &str) -> String {
        client.capture(&format!("/charges/{}/capture", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/charges/{}", id))
    }

}

#[derive(Debug)]
pub struct AntifraudDetails {
}

impl AntifraudDetails {

    pub fn new<S: Into<String>>(
        address: S,
        address_city: S,
        country_code: S,
        first_name: S,
        last_name: S,
        phone_number: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("address".to_string(), json!(address.into()));
        map.insert("address_city".to_string(), json!(address_city.into()));
        map.insert("country_code".to_string(), json!(country_code.into()));
        map.insert("first_name".to_string(), json!(first_name.into()));
        map.insert("last_name".to_string(), json!(last_name.into()));
        map.insert("phone_number".to_string(), json!(phone_number.into()));
        return map;
    }

}
