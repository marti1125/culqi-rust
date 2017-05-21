extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Plan {
}

impl Plan {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_plan = culqi::Plan::new("Hooli Subscription",1000,"PEN","meses",3,15,2,None);
    //! ```
    //!
    //! ```
    //! let create_plan = culqi::Plan::create(&client, &new_plan);
    //! ```
    //!
    //! ```
    //! let get_plan = culqi::Plan::retrieve(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let delete_plan = culqi::Plan::delete(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Plans
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut plan_filter: HashMap<String, String>;
    //! plan_filter = HashMap::new();
    //! plan_filter.insert("amount".to_string(), "100".to_string());
    //! plan_filter.insert("min_amount".to_string(), "100".to_string());
    //! plan_filter.insert("max_amount".to_string(), "2000".to_string());
    //! plan_filter.insert("interval".to_string(), "meses".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! plan_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! plan_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! plan_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_plans = culqi::Plan::all(&client, Some(plan_filter), None, None, None);
    //! ```
    //!

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
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/plans", query_params, limit, before, after)
    }

}
