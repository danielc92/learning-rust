use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn try_it_out() {
    /// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        company: String,
        exp: u64,
    }

    let my_claims: Claims = Claims {
        sub: "Daniel Corcoran".to_owned(),
        company: "Realtek".to_owned(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 3500,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );

    println!("{:?}", token);
    let decoded = decode::<Claims>(
        &token.unwrap(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    println!("{:?}", decoded);
}
