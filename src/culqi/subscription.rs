extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Subscription {
}

impl Subscription {

    pub fn new<S: Into<String>>(
        card_id: S,
        plan_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("card_id".to_string(), json!(card_id.into()));
        map.insert("plan_id".to_string(), json!(plan_id.into()));
        return map;
    }

    pub fn create(client: &Client, subscription: &HashMap<String, serde_json::Value>) -> String {
        client.post("/subscriptions", subscription)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/subscriptions/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/subscriptions/{}", id))
    }

}
