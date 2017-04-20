#[derive(Debug, RustcEncodable)]
pub struct Subscription {
    pub card_id: String,
    pub plan_id: String
}
