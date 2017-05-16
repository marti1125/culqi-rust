extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Refund {
}

impl Refund {

    pub fn new<S: Into<String>>(
        amount: i32,
        charge_id: S,
        reason: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("amount".to_string(), json!(amount));
        map.insert("charge_id".to_string(), json!(charge_id.into()));
        map.insert("reason".to_string(), json!(reason.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, refund: &HashMap<String, serde_json::Value>) -> String {
        client.post("/refunds", refund)
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/refunds/{}", id))
    }

    pub fn all(
        client: &Client,
        creation_date: Option<String>,
        creation_date_from: Option<String>,
        creation_date_to: Option<String>,
        reason: Option<String>,
        limit: Option<String>
    ) -> String {
        let mut query_url = String::from("/refunds");
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
        if !reason.is_none() {
            query_url.push_str("&reason");
            query_url.push_str(&format!("={:?}", reason));
        }
        client.get(&query_url)
    }

}
