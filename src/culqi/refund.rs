extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Refund {
    pub amount: i32,
    pub charge_id: String,
    pub reason: String
}

impl Refund {

    pub fn new<S: Into<String>>(
        amount: i32,
        charge_id: S,
        reason: S
    ) -> Refund {
        Refund {
            amount: amount,
            charge_id: charge_id.into(),
            reason: reason.into()
        }
    }

    pub fn create(client: &Client, refund: &Refund) -> String {
        let mut refund_json = serde_json::to_string(refund);
        client.post("/refunds", refund_json.unwrap().as_str())
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/refunds/{}", id))
    }

}
