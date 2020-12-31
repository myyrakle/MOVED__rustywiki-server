use sha2::{Digest};

pub fn hash(password: String) -> String {
    let bytes = sha2::Sha512::digest(password.as_bytes());

    let formatted = format!("{:x}", bytes);

    formatted
}