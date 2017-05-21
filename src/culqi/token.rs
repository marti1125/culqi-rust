extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Token {
}

impl Token {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_token = culqi::Token::new("4111111111111111","123",9,2020,"test@test.com", None);
    //! ```
    //!
    //! ```
    //! let create_token = culqi::Token::create(&client, &new_token);
    //! ```
    //!
    //! ```
    //! let get_token = culqi::Token::retrieve(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Tokens
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut token_filter: HashMap<String, String>;
    //! token_filter = HashMap::new();
    //! token_filter.insert("card_brand".to_string(), "VISA".to_string());
    //! token_filter.insert("card_type".to_string(), "DEBITO".to_string());
    //! token_filter.insert("device_type".to_string(), "DESKTOP".to_string());
    //! token_filter.insert("bin".to_string(), "411111".to_string());
    //! token_filter.insert("country_code".to_string(), "PE".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! token_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! token_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! token_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_tokens = culqi::Token::all(&client, Some(token_filter), None, None, None);
    //! ```
    //!

    pub fn new<S: Into<String>>(
        card_number: S,
        cvv: S,
        expiration_month: i32,
        expiration_year: i32,
        email: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("card_number".to_string(), json!(card_number.into()));
        map.insert("cvv".to_string(), json!(cvv.into()));
        map.insert("expiration_month".to_string(), json!(expiration_month));
        map.insert("expiration_year".to_string(), json!(expiration_year));
        map.insert("email".to_string(), json!(email.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, token: &HashMap<String, serde_json::Value>) -> String {
        client.post("/tokens", token)
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/tokens/{}", id))
    }

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/tokens", query_params, limit, before, after)
    }

}
