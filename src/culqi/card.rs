use client::Client;

#[derive(Debug, RustcEncodable)]
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

    pub fn all(client: &Client, id: &str) {
        client.get(&format!("/cards/{}", id));
    }

}
