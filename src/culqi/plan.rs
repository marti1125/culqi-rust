extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Plan {
}

impl Plan {

    pub fn new<S: Into<String>>(
        name: S,
        amount: i32,
        currency_code: S,
        interval: S,
        interval_count: i32,
        trial_days: i32
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("name".to_string(), json!(name.into()));
        map.insert("amount".to_string(), json!(amount));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("interval".to_string(), json!(interval.into()));
        map.insert("interval_count".to_string(), json!(interval_count));
        map.insert("trial_days".to_string(), json!(trial_days));
        return map;
    }

    pub fn create(client: &Client, plan: &HashMap<String, serde_json::Value>) -> String {
        client.post("/plans", plan)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/plans/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/plans/{}", id))
    }

}
