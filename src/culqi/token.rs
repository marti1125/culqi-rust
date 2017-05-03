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
        email: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("card_number".to_string(), json!(card_number.into()));
        map.insert("cvv".to_string(), json!(cvv.into()));
        map.insert("expiration_month".to_string(), json!(expiration_month));
        map.insert("expiration_year".to_string(), json!(expiration_year));
        map.insert("email".to_string(), json!(email.into()));
        return map;
    }

    pub fn create(client: &Client, token: &HashMap<String, serde_json::Value>) -> String {
        client.post("/tokens", token)
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/tokens/{}", id))
    }

}
