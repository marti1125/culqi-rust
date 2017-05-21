extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Refund {
}

impl Refund {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_refund = culqi::Refund::new(100,"{CHARGE_ID}","fradudulento", None);
    //! ```
    //!
    //! ```
    //! let create_refund = culqi::Refund::create(&client, &new_refund);
    //! ```
    //!
    //! ```
    //! let get_refund = culqi::Refund::retrieve(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Refunds
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut reason_filter: HashMap<String, String>;
    //! reason_filter = HashMap::new();
    //! reason_filter.insert("reason".to_string(), "solicitud_comprador".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! reason_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! reason_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! reason_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_refunds = culqi::Refund::all(&client, Some(reason_filter), None, None, None);
    //! ```
    //!

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
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/refunds", query_params, limit, before, after)
    }

}
