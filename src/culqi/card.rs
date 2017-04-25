extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Card {
    pub customer_id: String,
    pub token_id: String
}

impl Card {

    pub fn new<S: Into<String>>(
        customer_id: S,
        token_id: S
    ) -> Card {
        Card {
            customer_id: customer_id.into(),
            token_id: token_id.into()
        }
    }

    pub fn create(client: &Client, card: &Card) -> String {
        let mut card_json = serde_json::to_string(card);
        client.post("/cards", card_json.unwrap().as_str())
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/cards/{}", id))
    }

}
