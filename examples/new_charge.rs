extern crate culqi;

use std::env;

fn main() {

    let public_key = env::var("PUBLIC_KEY").unwrap();
    let secret_key = env::var("SECRET_KEY").unwrap();

    let client_pk = culqi::Client::new(&public_key);
    let client_sk = culqi::Client::new(&secret_key);

    let new_token = culqi::Token::new("4111111111111111","123",9, 2020,"test@test.com", None);

    let get_token = culqi::Token::create(&client_pk, &new_token);

    let token_json: Value = serde_json::from_str(&get_token).unwrap();

    let new_charge = culqi::Charge::new("1000", "PE", "will@me.com", 1,"{token}");

    let get_charge = culqi::Charge::create(&client_sk, &new_charge);

    println!(" Response {:?}", get_charge);

}
