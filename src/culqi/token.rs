use client::Client;

#[derive(Debug, RustcEncodable)]
pub struct Token {
    pub card_number: String,
    pub cvv: String,
    pub expiration_month: String,
    pub expiration_year: String,
    pub email: String
}

impl Token {

    pub fn new<S: Into<String>>(
        card_number: S,
        cvv: S,
        expiration_month: S,
        expiration_year: S,
        email: S
    ) -> Token {
        Token {
            card_number: card_number.into(),
            cvv: cvv.into(),
            expiration_month: expiration_month.into(),
            expiration_year: expiration_year.into(),
            email: email.into()
        }
    }

    pub fn create(client: &Client, token: &Token) {
         client.post();
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/tokens/{}", id))
    }

}
