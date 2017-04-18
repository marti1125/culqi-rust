extern crate hyper;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

fn main() {

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut plan = String::new();

    let mut headers = Headers::new();
    headers.set(
       Authorization(
           Bearer {
               token: "sk_test_UTCQSGcXW8bCyU59".to_owned()
           }
       )
    );

    headers.set(
        ContentType(
            Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)]
            )
        )
    );

    let create_customer = client.post("https://api.culqi.com/v2/customers")
        .body(
            r#"
            {
              "address": "Av Lima 123",
              "address_city": "Lima",
              "country_code": "PE",
              "email": "wwm@gmail.com",
              "first_name": "Marti",
              "last_name": "Rodriguez",
              "phone_number": 23432423123
            }
            "#
        )
        .headers(headers)
        .send()
        .unwrap();

    println!(" STATUS  {:?}", create_customer);

    let get_plan = client.get("https://api.culqi.com/v2/plans/pln_test_UqFVmhqDKQo9ygbJ")
        .headers(headers)
        .send()
        .unwrap()
        .read_to_string(&mut plan)
        .unwrap();

    println!("RESPONSE {:?}", get_plan);
    println!("BODY {:?}", plan);

}
