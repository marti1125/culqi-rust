extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Plan {
    pub name: String,
    pub amount: i32,
    pub currency_code: String,
    pub interval: String,
    pub interval_count: i32,
    pub trial_days: i32
}

impl Plan {

    pub fn new<S: Into<String>>(
        name: S,
        amount: i32,
        currency_code: S,
        interval: S,
        interval_count: i32,
        trial_days: i32
    ) -> Plan {
        Plan {
            name: name.into(),
            amount: amount,
            currency_code: currency_code.into(),
            interval: interval.into(),
            interval_count: interval_count,
            trial_days: trial_days
        }
    }

    pub fn create(client: &Client, plan: &Plan) -> String {
        let mut plan_json = serde_json::to_string(plan);
        client.post("/plans", plan_json.unwrap().as_str())
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/plans/{}", id))
    }

}
