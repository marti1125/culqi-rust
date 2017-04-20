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
}
