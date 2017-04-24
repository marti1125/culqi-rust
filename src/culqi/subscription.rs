use client::Client;

#[derive(Debug, RustcEncodable)]
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

    pub fn create(client: &Client, subscription: &Subscription) {
         client.post();
    }

    pub fn all(client: &Client, id: &str) {
        client.get(&format!("/subscriptions/{}", id));
    }

}
