# culqi-rust
Culqi API - Rust

### Instructions for Build

```bash
cargo build
```

#### Ejemplo

> Para la creación de tokens de forma directa(utilizando el API) solo se activaran sus llaves en integración
para casos de prueba.

```rust
extern crate culqi;

use std::env;

fn main() {

    let public_key = env::var("PUBLIC_KEY").unwrap();

    let client_pk = culqi::Client::new(&public_key);

    let new_token = culqi::Token::new("4111111111111111","123",9, 2020,"test@test.com");

    let get_token = culqi::Token::create(&client_sk, &new_token);

    println!(" Response {:?}", get_token);

}
```
