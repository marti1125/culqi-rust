use hyper::Client as HttpClient;
use hyper::client::Response;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::json::encode;

pub struct Client {
    client: HttpClient,
    secret_key: String
}

impl Client {

    pub fn new(secret_key: &str) -> Client {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HttpClient::with_connector(connector);
        return Client{client: client, secret_key: secret_key.to_owned()};
    }

    pub fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(
           Authorization(
               Bearer {
                   token: self.secret_key.to_owned()
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
        return headers;
    }

    pub fn get(&self, path: &str) -> String {
        let mut body_response = String::new();
        let url = get_url(path);
        let request = self.client.get(&url)
                .headers(self.get_headers())
                .send()
                .unwrap()
                .read_to_string(&mut body_response)
                .unwrap();
        return body_response;
    }

    pub fn post(&self) {
    }

    pub fn delete(&self) {
    }

    pub fn patch(&self) {
    }

}

fn get_url(path: &str) -> String {
    String::from("https://api.culqi.com/v2") + path
}

// fn send() {
//
// }
