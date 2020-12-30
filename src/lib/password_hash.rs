use sha2::{Digest};

pub fn hash(password: String) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(password);
    let bytes: &[u8] = &hasher.finalize();
    String::from_utf8_lossy(bytes).to_string()
}