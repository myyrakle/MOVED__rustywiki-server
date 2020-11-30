use epoch_timestamp::Epoch;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    user_type: String,
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

fn read_key() -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let file = File::open("/env/key.txt");

    if file.is_ok() {
        let mut file = file.unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    } else {
        "foobar".to_string()
    }
}

pub fn verify(token: String) -> Option<i64> {
    let key = read_key();
    let key = key.as_bytes();

    let decoding_key = DecodingKey::from_secret(key);

    let validation = Validation::new(Algorithm::HS256);

    let claims = jsonwebtoken::decode::<Claims>(token.as_str(), &decoding_key, &validation);

    if claims.is_ok() {
        Some(claims.unwrap().claims.user_id)
    } else {
        None
    }
}

pub fn sign(user_id: i64, user_type: String) -> String {
    let key = read_key();
    let key = key.as_bytes();

    let epoch = Epoch::now() + Epoch::day(1);

    let data = Claims {
        user_id: user_id,
        user_type: user_type,
        exp: epoch as usize,
    };

    let header = Header::new(Algorithm::HS256);

    jsonwebtoken::encode::<Claims>(&header, &data, &EncodingKey::from_secret(key)).unwrap()
}
