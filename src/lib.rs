//! Culqi-Rust is a library for used Culqi API
//!
//! # Usage
//!
//! Put this in your `Cargo.toml`
//!
//! ```
//! [dependencies]
//! culqi = "0.2.0"
//! ```
//!
//! And this in your crate root:
//!
//! ```
//! extern crate culqi;
//! ```
//!
//! # Examples
//!
//!
//! ## Initialization
//!
//! At Culqi you have two keys (public_key, secret_key) public_key is only for create Token
//!
//! ```
//! let public_key = "{PUBLIC_KEY}";
//! let secret_key = "{SECRET_KEY}";
//! ```
//!
//! ```
//! let client = culqi::Client::new(&secret_key);
//! ```
//!
//! Create Token is only use in integration enviroment (keys must be activated)
//!
//! ```
//! let new_token = culqi::Token::new("4111111111111111","123",9,2020,"test@test.com", None);
//! ```
//!
//! ## Create a Charge with Antifraud Details
//!
//! ```
//! let ref antifraud_details = culqi::AntifraudDetails::new("av. lima", "lima", "PE", "Will", "Aguirre", "993978969");
//! let new_charge = culqi::Charge::new("3500", "PEN", "will@me.com", 4, None, Some(antifraud_details), "{token}");
//! let get_charge = culqi::Charge::create(&client, &new_charge);
//! ```

#![feature(box_syntax)]
#![feature(custom_attribute)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_qs;

mod client;
mod culqi;

pub use client::Client;
pub use culqi::*;
