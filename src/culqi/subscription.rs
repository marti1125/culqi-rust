extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Subscription {
    pub card_id: String,
    pub plan_id: String
}

impl Subscription {

    pub fn new<S: Into<String>>(
        card_id: S,
        plan_id: S
    ) -> Subscription {
        Subscription {
            card_id: card_id.into(),
            plan_id: plan_id.into()
        }
    }

    pub fn create(client: &Client, subscription: &Subscription) -> String {
        client.post("/subscriptions", subscription)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/subscriptions/{}", id))
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/subscriptions/{}", id))
    }

}
