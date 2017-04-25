extern crate culqi;

fn main() {

    let new_token = culqi::Token::new("4111111111111111","123","03","20","test@test.com");

    let client = culqi::Client::new("sk_test_UTCQSGcXW8bCyU59");

    let get_token = culqi::Token::create(&client, &new_token);

    let charges = culqi::Charge::all(&client, "chr_test_oOLn2IdX2fQ1jyG2");

    let plans = culqi::Plan::all(&client, "pln_test_UqFVmhqDKQo9ygbJ");

    println!("New Token {:?}", get_token);

    println!("Charges {:?}", charges);

    println!("Plans {:?}", plans);

}
