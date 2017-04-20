use hyper::Client;

#[derive(Debug, RustcEncodable)]
pub struct Card {
    pub customer_id: String,
    pub token_id: String
}

impl Card {

    pub fn all(client: &Client, id: &str) {
        client.get(&format!("/cards/{}", id));
    }

}
