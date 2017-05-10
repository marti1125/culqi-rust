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
        metadata: Option<&HashMap<String, serde_json::Value>>,
        antifraud_details: Option<&HashMap<String, serde_json::Value>>,
        source_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("amount".to_string(), json!(amount.into()));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("installments".to_string(), json!(installments));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        if !antifraud_details.is_none() {
            map.insert("antifraud_details".to_string(), json!(antifraud_details));
        }
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
