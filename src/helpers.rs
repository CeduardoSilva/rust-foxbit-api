use hex;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub fn create_signature(prehash: &str, api_secret: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(prehash.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    hex::encode(code_bytes)
}

pub fn get_timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis().to_string(),
        Err(_) => panic!("SystemTime before UNIX EPOCH"),
    }
}

pub fn get_prehash<B: Serialize>(
    endpoint: &str,
    timestamp: &str,
    query_string: Option<&str>,
    body: Option<&B>,
) -> String {
    let method = if body.is_some() { "POST" } else { "GET" };

    let qs = query_string.unwrap_or_else(|| "");

    let b = match body {
        Some(b) => serde_json::to_string(b).unwrap(),
        None => "".into(),
    };

    format!(
        "{}{}{}{}{}{}",
        timestamp, method, "/rest/v3", endpoint, qs, b
    )
}
