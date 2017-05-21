extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Customer {
}

impl Customer {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let new_customer = culqi::Customer::new("Will","Muro","wmuro@me.com","Av. Lima 123","LIMA","PE","999999999",None);
    //! ```
    //!
    //! ```
    //! let create_customer = culqi::Customer::create(&client, &new_customer);
    //! ```
    //!
    //! ```
    //! let get_customer = culqi::Customer::retrieve(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let delete_customer = culqi::Customer::delete(&client, "{ID}");
    //! ```
    //!
    //!
    //! Here you are the list of filters you can use for get Customers
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut customer_filter: HashMap<String, String>;
    //! customer_filter = HashMap::new();
    //! customer_filter.insert("first_name".to_string(), "Will".to_string());
    //! customer_filter.insert("last_name".to_string(), "Muro".to_string());
    //! customer_filter.insert("email".to_string(), "will@me.com".to_string());
    //! customer_filter.insert("address".to_string(), "Av. Lima 123".to_string());
    //! customer_filter.insert("address_city".to_string(), "LIMA".to_string());
    //! customer_filter.insert("phone_number".to_string(), "99999999".to_string());
    //! customer_filter.insert("country_code".to_string(), "PE".to_string());
    //! ```
    //!
    //! ```
    //! let all_customers = culqi::Customer::all(&client, Some(customer_filter), None, None, None);
    //! ```
    //!

    pub fn new<S: Into<String>>(
        first_name: S,
        last_name: S,
        email: S,
        address: S,
        address_city: S,
        country_code: S,
        phone_number: S,
        metadata: Option<HashMap<String, serde_json::Value>>
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("first_name".to_string(), json!(first_name.into()));
        map.insert("last_name".to_string(), json!(last_name.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("address".to_string(), json!(address.into()));
        map.insert("address_city".to_string(), json!(address_city.into()));
        map.insert("country_code".to_string(), json!(country_code.into()));
        map.insert("phone_number".to_string(), json!(phone_number.into()));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        return map;
    }

    pub fn create(client: &Client, customer: &HashMap<String, serde_json::Value>) -> String {
        client.post("/customers", customer)
    }

    pub fn delete(client: &Client, id: &str) -> String {
        client.delete(&format!("/customers/{}", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/customers/{}", id))
    }

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/customers", query_params, limit, before, after)
    }

}
