extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug)]
pub struct AntifraudDetails {
}

impl AntifraudDetails {

    //! # Examples
    //!
    //! ```
    //! let ref antifraud_details = culqi::AntifraudDetails::new("av. lima", "lima", "PE", "Will", "Aguirre", "993978969");
    //! ```
    //!

    pub fn new<S: Into<String>>(
        address: S,
        address_city: S,
        country_code: S,
        first_name: S,
        last_name: S,
        phone_number: S
    ) -> HashMap<String, serde_json::Value> {
        let mut map: HashMap<String, serde_json::Value>;
        map = HashMap::new();
        map.insert("address".to_string(), json!(address.into()));
        map.insert("address_city".to_string(), json!(address_city.into()));
        map.insert("country_code".to_string(), json!(country_code.into()));
        map.insert("first_name".to_string(), json!(first_name.into()));
        map.insert("last_name".to_string(), json!(last_name.into()));
        map.insert("phone_number".to_string(), json!(phone_number.into()));
        return map;
    }

}
