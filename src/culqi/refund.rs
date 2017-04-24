use client::Client;

#[derive(Debug, RustcEncodable)]
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

    pub fn create(client: &Client, refund: &Refund) {
         client.post();
    }

    pub fn all(client: &Client, id: &str) {
        client.get(&format!("/refunds/{}", id));
    }

}
