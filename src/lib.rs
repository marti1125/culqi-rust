//! Culqi-Rust is a library for used Culqi API
//!
//! # Examples
//!
//! Create Token is only use in integration enviroment (keys must be activated)
//!
//! ```
//! let new_token = culqi::Token::new("4111111111111111","123",9, 2020,"test@test.com");
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
