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
        trial_days: i32,
        limit: Option<i32>,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("name".to_string(), json!(name.into()));
        map.insert("amount".to_string(), json!(amount));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("interval".to_string(), json!(interval.into()));
        map.insert("interval_count".to_string(), json!(interval_count));
        map.insert("trial_days".to_string(), json!(trial_days));
        if !limit.is_none() {
            map.insert("limit".to_string(), json!(limit));
        }
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
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

    pub fn all(
        client: &Client,
        amount: Option<String>,
        min_amount: Option<String>,
        max_amount: Option<String>,
        creation_date: Option<String>,
        creation_date_from: Option<String>,
        creation_date_to: Option<String>,
        limit: Option<String>
    ) -> String {
        let mut query_url = String::from("/plans");
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
        client.get(&query_url)
    }

}
