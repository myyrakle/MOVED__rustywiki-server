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

    let file = File::open("env/key.txt");

    match file {
        Ok(mut file) => {
            let mut buffer = String::new();

            match file.read_to_string(&mut buffer) {
                Ok(buffer) => buffer.to_string(),
                Err(error) => {
                    log::error!("file read error: {}", error);
                    "foobar".to_string()
                }
            }
        }
        Err(error) => {
            log::error!("file read error: {}", error);
            "foobar".to_string()
        }
    }
}

pub fn verify(token: String) -> Option<i64> {
    let key = read_key();
    let key = key.as_bytes();

    let decoding_key = DecodingKey::from_secret(key);

    let validation = Validation::new(Algorithm::HS256);

    let claims = jsonwebtoken::decode::<Claims>(token.as_str(), &decoding_key, &validation);

    match claims {
        Ok(claims) => Some(claims.claims.user_id),
        Err(_) => None,
    }
}

pub fn sign(exp: usize, user_id: i64, user_type: String) -> String {
    let key = read_key();
    let key = key.as_bytes();

    //let epoch = Epoch::now() + Epoch::year(100);

    let data = Claims {
        user_id: user_id,
        user_type: user_type,
        exp: exp,
    };

    let header = Header::new(Algorithm::HS256);

    jsonwebtoken::encode::<Claims>(&header, &data, &EncodingKey::from_secret(key))
        .unwrap_or("".into())
}

use epoch_timestamp::Epoch;

pub fn create_access_token(user_id: i64, user_type: String) -> String {
    let epoch = (Epoch::now() + Epoch::hour(1)) as usize;
    sign(epoch, user_id, user_type)
}

pub fn create_refresh_token(user_id: i64, user_type: String) -> String {
    let epoch = (Epoch::now() + Epoch::year(1)) as usize;
    sign(epoch, user_id, user_type)
}
