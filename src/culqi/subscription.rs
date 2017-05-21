extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Subscription {
}

impl Subscription {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_subscription = culqi::Subscription::new("{CARD_ID}", "{PLAN_ID}", None);
    //! ```
    //!
    //! ```
    //! let create_subscription = culqi::Subscription::create(&client, &new_subscription);
    //! ```
    //!
    //! ```
    //! let del_subscription = culqi::Subscription::delete(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let get_subscription = culqi::Subscription::retrieve(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Subscriptions
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut subscription_filter: HashMap<String, String>;
    //! subscription_filter = HashMap::new();
    //! subscription_filter.insert("amount".to_string(), "100".to_string());
    //! subscription_filter.insert("min_amount".to_string(), "100".to_string());
    //! subscription_filter.insert("max_amount".to_string(), "2000".to_string());
    //! subscription_filter.insert("interval".to_string(), "dias".to_string());
    //! subscription_filter.insert("status".to_string(), "activa".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! subscription_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! subscription_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! subscription_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_subscription = culqi::Subscription::all(&client, Some(subscription_filter), None, None, None);
    //! ```
    //!

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
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/subscriptions", query_params, limit, before, after)
    }

}
