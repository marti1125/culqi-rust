extern crate serde_json;

use client::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Charge {
}

impl Charge {

    //! # Examples
    //!
    //! ```
    //! let client = culqi::Client::new("{KEY}");
    //! ```
    //!
    //! ```
    //! let ref antifraud_details = culqi::AntifraudDetails::new("av. lima", "lima", "PE", "Will", "Aguirre", "993978969");
    //! ```
    //!
    //! ```
    //! let new_charge = culqi::Charge::new("3500", "PEN", "will@me.com", charge.installments, None, Some(antifraud_details), "{TOKEN}");
    //! ```
    //!
    //! ```
    //! let create_charge = culqi::Charge::create(&client, &new_charge);
    //! ```
    //!
    //! ```
    //! let get_charge = culqi::Charge::retrieve(&client, "{ID}");
    //! ```
    //!
    //! ```
    //! let capture_charge = culqi::Charge::capture(&client, "{ID}");
    //! ```
    //!
    //! Here you are the list of filters you can use for get Charges
    //!
    //! ```
    //! use std::collections::HashMap;
    //! ```
    //!
    //! ```
    //! let mut charge_filter: HashMap<String, String>;
    //! charge_filter = HashMap::new();
    //! charge_filter.insert("amount".to_string(), "100".to_string());
    //! charge_filter.insert("min_amount".to_string(), "100".to_string());
    //! charge_filter.insert("max_amount".to_string(), "1000".to_string());
    //! charge_filter.insert("installments".to_string(), "0".to_string());
    //! charge_filter.insert("min_installments".to_string(), "2".to_string());
    //! charge_filter.insert("max_installments".to_string(), "6".to_string());
    //! charge_filter.insert("max_installments".to_string(), "6".to_string());
    //! charge_filter.insert("currency_code".to_string(), "PEN".to_string());
    //! charge_filter.insert("fraud_score".to_string(), "50".to_string());
    //! charge_filter.insert("min_fraud_score".to_string(), "10".to_string());
    //! charge_filter.insert("max_fraud_score".to_string(), "50".to_string());
    //! charge_filter.insert("first_name".to_string(), "Will".to_string());
    //! charge_filter.insert("last_name".to_string(), "Muro".to_string());
    //! charge_filter.insert("email".to_string(), "wmuro@me.com".to_string());
    //! charge_filter.insert("address".to_string(), "Av. Lima 123".to_string());
    //! charge_filter.insert("address_city".to_string(), "LIMA".to_string());
    //! charge_filter.insert("phone_number".to_string(), "999999999".to_string());
    //! charge_filter.insert("country_code".to_string(), "PE".to_string());
    //! charge_filter.insert("capture".to_string(), "true".to_string());
    //! charge_filter.insert("dispute".to_string(), "true".to_string());
    //! charge_filter.insert("paid".to_string(), "true".to_string());
    //! charge_filter.insert("customer_id".to_string(), "{CUSTOMER_ID}".to_string());
    //! charge_filter.insert("fee".to_string(), "20".to_string());
    //! charge_filter.insert("min_fee".to_string(), "20".to_string());
    //! charge_filter.insert("max_fee".to_string(), "60".to_string());
    //! charge_filter.insert("card_brand".to_string(), "visa".to_string());
    //! charge_filter.insert("card_type".to_string(), "debito".to_string());
    //! charge_filter.insert("device_type".to_string(), "tablet".to_string());
    //! charge_filter.insert("bin".to_string(), "411111".to_string());
    //! ```
    //!
    //! It must uses [Unix Timestamp](http://www.unixtimestamp.com/index.php)
    //!
    //! ```
    //! charge_filter.insert("creation_date".to_string(), "1484006400".to_string());
    //! charge_filter.insert("creation_date_from".to_string(), "1479600000".to_string());
    //! charge_filter.insert("creation_date_to".to_string(), "1484006400".to_string());
    //! ```
    //!
    //! ```
    //! let all_charges = culqi::Charge::all(&client_sk, Some(charge_filter), None, None, None);
    //! ```
    //!

    pub fn new<S: Into<String>>(
        amount: S,
        currency_code: S,
        email: S,
        installments: i32,
        metadata: Option<&HashMap<String, serde_json::Value>>,
        antifraud_details: Option<&HashMap<String, serde_json::Value>>,
        source_id: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("amount".to_string(), json!(amount.into()));
        map.insert("currency_code".to_string(), json!(currency_code.into()));
        map.insert("email".to_string(), json!(email.into()));
        map.insert("installments".to_string(), json!(installments));
        if !metadata.is_none() {
            map.insert("metadata".to_string(), json!(metadata));
        }
        if !antifraud_details.is_none() {
            map.insert("antifraud_details".to_string(), json!(antifraud_details));
        }
        map.insert("source_id".to_string(), json!(source_id.into()));
        return map;
    }

    pub fn create(client: &Client, charge: &HashMap<String, serde_json::Value>) -> String {
        client.post("/charges", charge)
    }

    pub fn capture(client: &Client, id: &str) -> String {
        client.capture(&format!("/charges/{}/capture", id))
    }

    pub fn retrieve(client: &Client, id: &str) -> String {
        client.get(&format!("/charges/{}", id))
    }

    pub fn all(
        client: &Client,
        query_params: Option<HashMap<String, String>>,
        limit: Option<String>,
        before: Option<String>,
        after: Option<String>
    ) -> String {
        client.get_filter("/charges", query_params, limit, before, after)
    }

}
