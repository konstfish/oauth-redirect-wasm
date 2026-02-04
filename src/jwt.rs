use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Claims {
    exp: i64,
}

pub fn is_valid_within(token: &str, minutes: i64) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return true;
    }

    let payload = parts[1];
    let decoded_payload = match URL_SAFE_NO_PAD.decode(payload) {
        Ok(bytes) => bytes,
        Err(_) => return true,
    };

    let claims: Claims = match serde_json::from_slice(&decoded_payload) {
        Ok(c) => c,
        Err(_) => return true,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    claims.exp - now > minutes * 60
}