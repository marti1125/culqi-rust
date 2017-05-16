extern crate serde_json;

use client::Client;
use std::collections::HashMap;
use std::iter::IntoIterator;

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

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        let mut query_url = String::from("/charges");
        query_url.push_str("?limit");
        if limit.is_none() {
            query_url.push_str("=50");
        } else {
            query_url.push_str(&format!("={:?}", limit));
        }
        if !query_params.is_none() {
            for (k, v) in query_params.into_iter().flat_map(IntoIterator::into_iter) {
                query_url.push_str(&format!("&{:?}", k));
                query_url.push_str(&format!("={:?}", v));
            }
        }        
        if !before.is_none() {
            query_url.push_str("&before");
            query_url.push_str(&format!("={:?}", before));
        }
        if !after.is_none() {
            query_url.push_str("&after");
            query_url.push_str(&format!("={:?}", after));
        }
        client.get(&query_url)
    }

}
