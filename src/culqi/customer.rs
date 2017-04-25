extern crate serde_json;

use client::Client;

#[derive(Debug, Serialize)]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
    pub address_city: String,
    pub country_code: String,
    pub phone_number: String
}

impl Customer {

    pub fn new<S: Into<String>>(
        first_name: S,
        last_name: S,
        email: S,
        address: S,
        address_city: S,
        country_code: S,
        phone_number: S
    ) -> Customer {
        Customer {
            first_name: first_name.into(),
            last_name: last_name.into(),
            email: email.into(),
            address: address.into(),
            address_city: address_city.into(),
            country_code: country_code.into(),
            phone_number: phone_number.into()
        }
    }

    pub fn create(client: &Client, customer: &Customer) -> String {
        let mut customer_json = serde_json::to_string(customer);
        client.post("/customers", customer_json.unwrap().as_str())
    }

    pub fn all(client: &Client, id: &str) -> String {
        client.get(&format!("/customers/{}", id))
    }

}
