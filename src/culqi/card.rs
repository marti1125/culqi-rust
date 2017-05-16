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
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/cards", query_params, limit, before, after)
    }

}
