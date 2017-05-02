extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Card {
}

impl Card {

    pub fn new<S: Into<String>>(
        customer_id: S,
        token_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("customer_id".to_string(), json!(customer_id.into()));
        map.insert("customer_id".to_string(), json!(token_id.into()));
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

}
