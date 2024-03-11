use reqwest::header::HeaderValue;

use crate::{
    helpers::{create_signature, get_timestamp},
    types::{Currency, FoxBitResponse},
};

pub async fn list_currencies(
    base_url: &str,
    api_secret: &str,
    access_key: &str,
) -> Result<Vec<Currency>, serde_json::Error> {
    let timestamp = get_timestamp();
    let prehash = format!("{}{}{}{}", &timestamp, "GET", "/rest/v3/currencies", "");
    let signature = create_signature(&prehash, api_secret);
    let url = format!("{}/currencies", base_url);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-FB-ACCESS-KEY",
        reqwest::header::HeaderValue::from_str(&access_key).unwrap(),
    );
    headers.insert(
        "X-FB-ACCESS-TIMESTAMP",
        HeaderValue::from_str(&timestamp).unwrap(),
    );
    headers.insert(
        "X-FB-ACCESS-SIGNATURE",
        HeaderValue::from_str(&signature).unwrap(),
    );

    let response = match client.get(&url).headers(headers).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text_response) => text_response,
            Err(e) => {
                eprintln!("Converting Foxbit to text failed: {}", e);
                e.to_string()
            }
        },
        Err(e) => {
            eprintln!("Request to Foxbit failed: {}", e);
            e.to_string()
        }
    };

    let json_response = serde_json::from_str::<FoxBitResponse>(&response);
    match json_response {
        Ok(json) => Ok(json.data),
        Err(e) => {
            eprintln!("Conversion to json failed: {}", e);
            Err(e)
        }
    }
}
