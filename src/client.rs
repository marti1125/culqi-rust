extern crate hyper;

use hyper::Client as HttpClient;

pub struct Client {
    client: HttpClient,
    secret_key: String
}
