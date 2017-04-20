use hyper::Client;

#[derive(Debug, RustcEncodable)]
pub struct Charge {
    pub amount: String,
    pub currency_code: String,
    pub email: String,
    pub source_id: String
}

impl Charge {

    // pub fn create(client: &Client, charge: &Charge){
    //     client.post();
    // }

    pub fn all(client: &Client, id: &str) {
        client.get(&format!("/charges/{}", id));
    }

}
