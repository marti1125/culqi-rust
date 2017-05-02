extern crate serde_json;

use client::Client;

#[derive(Debug)]
pub struct Token {
}

impl Token {

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/tokens/{}", id))
    }

}
