extern crate culqi;

fn main() {

    let client = culqi::Client::new("sk_test_UTCQSGcXW8bCyU59");

    /*let token = culqi::Token::new("41111111", "123", "09", "20", "ww@me.com");
    println!("Token {:?}", token);
    println!("Token CVV {:?}", token.cvv);

    let encoded_token = json::encode(&token).unwrap();

    println!("TOKEN JSON {:?}", encoded_token);

    let mut api_base_url : String = "https://api.culqi.com/v2".into();

    let mut rng = rand::thread_rng();

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

    //let email = "wwww_www".to_string()+rng+"@gmail.com".to_string();
    let token_url : String = "/tokens".into();
    api_base_url.push_str(&token_url);

    let create_token = client.post(&api_base_url)
        .body(&encoded_token)
        .headers(headers.clone())
        .send()
        .unwrap();

    println!("RESULT {:?}", create_token);

    // let create_customer = client.post("https://api.culqi.com/v2/customers")
    //     .body(
    //         r#"
    //         {
    //           "address": "Av Lima 123",
    //           "address_city": "Lima",
    //           "country_code": "PE",
    //           "email": "email",
    //           "first_name": "Marti",
    //           "last_name": "Rodriguez",
    //           "phone_number": 23432423123
    //         }
    //         "#
    //     )
    //     .headers(headers.clone())
    //     .send()
    //     .unwrap();
    //
    // println!(" STATUS  {:?}", create_customer);

    let get_plan = client.get("https://api.culqi.com/v2/plans/pln_test_UqFVmhqDKQo9ygbJ")
        .headers(headers)
        .send()
        .unwrap()
        .read_to_string(&mut plan)
        .unwrap();

    println!("RESPONSE {:?}", get_plan);
    println!("BODY {:?}", plan);
    */
}
