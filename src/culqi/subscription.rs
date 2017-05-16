extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Subscription {
}

impl Subscription {

    pub fn new<S: Into<String>>(
        card_id: S,
        plan_id: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("card_id".to_string(), json!(card_id.into()));
        map.insert("plan_id".to_string(), json!(plan_id.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
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

    pub fn all(
        client: &Client,
        amount: Option<String>,
        min_amount: Option<String>,
        max_amount: Option<String>,
        creation_date: Option<String>,
        creation_date_from: Option<String>,
        creation_date_to: Option<String>,
        interval: Option<String>,
        status: Option<String>,
        limit: Option<String>
    ) -> String {
        let mut query_url = String::from("/tokens");
        query_url.push_str("?limit");
        if limit.is_none() {
            query_url.push_str("=50");
        } else {
            query_url.push_str(&format!("={:?}", limit));
        }
        if !amount.is_none() {
            query_url.push_str("&amount");
            query_url.push_str(&format!("={:?}", amount));
        }
        if !min_amount.is_none() {
            query_url.push_str("&min_amount");
            query_url.push_str(&format!("={:?}", min_amount));
        }
        if !max_amount.is_none() {
            query_url.push_str("&max_amount");
            query_url.push_str(&format!("={:?}", max_amount));
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
        if !interval.is_none() {
            query_url.push_str("&interval");
            query_url.push_str(&format!("={:?}", interval));
        }
        if !status.is_none() {
            query_url.push_str("&status");
            query_url.push_str(&format!("={:?}", status));
        }
        client.get(&query_url)
    }

}
