extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Card {
}

impl Card {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_card = culqi::Card::new("{CUSTOMER_ID}","{TOKEN_ID}",None,None);
    //! ```
    //!
    //! ```
    //! let create_card = culqi::Card::create(&client, &new_card);
    //! ```
    //!
    //! ```
    //! let get_card = culqi::Card::retrieve(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let delete_card = culqi::Card::delete(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Cards
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut card_filter: HashMap<String, String>;
    //! card_filter = HashMap::new();
    //! card_filter.insert("card_brand".to_string(), "visa".to_string());
    //! card_filter.insert("card_type".to_string(), "debito".to_string());
    //! card_filter.insert("device_type".to_string(), "escritorio".to_string());
    //! card_filter.insert("bin".to_string(), "411111".to_string());
    //! card_filter.insert("country_code".to_string(), "PE".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! card_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! card_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! card_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_cards = culqi::Card::all(&client, Some(card_filter), None, None, None);
    //! ```
    //!

    pub fn new<S: Into<String>>(
        customer_id: S,
        token_id: S,
        validate: Option<bool>,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("customer_id".to_string(), json!(customer_id.into()));
        map.insert("customer_id".to_string(), json!(token_id.into()));
        if !validate.is_none() {
            map.insert("validate".to_string(), json!(validate));
        }
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, card: &HashMap<String, serde_json::Value>) -> String {
        client.post("/cards", card)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/cards/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/cards/{}", id))
    }

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/cards", query_params, limit, before, after)
    }

}
