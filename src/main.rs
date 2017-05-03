extern crate serde;
extern crate serde_json;
extern crate culqi;

use std::env;
use serde_json::{Value};

fn main() {

    let public_key = env::var("PUBLIC_KEY").unwrap();
    let secret_key = env::var("SECRET_KEY").unwrap();

    let client_pk = culqi::Client::new(&public_key);
    let client_sk = culqi::Client::new(&secret_key);

    let new_token = culqi::Token::new("4111111111111111","123",9, 2020,"test@test.com");

    //let get_token = culqi::Token::create(&client_pk, &new_token);

    //let token_json: Value = serde_json::from_str(&get_token).unwrap();

    let new_charge = culqi::Charge::new("1000", "PE", "will@me.com", "ffff");

    let get_charge = culqi::Charge::create(&client_sk, &new_charge);

    println!(" Response {:?}", get_charge);

    //let ref token_id = token_json["data"][0]["id"];

    //let id = token_id.to_string().replace("\"","");

    //let new_charge = culqi::Charge::new("1000", "PE", "will@me.com", &id);

    //let get_charge = culqi::Charge::create(&client_sk, &new_charge);

    //let charges = culqi::Charge::all(&client, "chr_test_oOLn2IdX2fQ1jyG2");

    //let plans = culqi::Plan::all(&client, "pln_test_UqFVmhqDKQo9ygbJ");

    //println!("New Token {:?}", get_token);

    //println!("Charges {:?}", charges);

    //println!("Plans {:?}", plans);

    //let token_json: Value = serde_json::from_str(&get_token).unwrap();

    //let ref token_id = token_json["data"][0]["id"];

    //println!(" Token JSON {:?}", id);

    //println!(" Charge JSON {:?}", get_charge);

}
