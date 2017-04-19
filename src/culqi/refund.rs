#[derive(Debug)]
pub struct Refund {
    pub amount: i32,
    pub charge_id: String,
    pub reason: String
}
