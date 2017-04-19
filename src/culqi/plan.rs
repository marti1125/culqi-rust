#[derive(Debug)]
pub struct Plan {
    pub name: String,
    pub amount: i32,
    pub currency_code: String,
    pub interval: String,
    pub interval_count: i32,
    pub trial_days: i32
}
