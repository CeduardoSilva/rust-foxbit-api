use reqwest::{header::HeaderValue, Client};

use crate::{
    helpers::{create_signature, get_timestamp},
    types::{Currency, FoxBitResponse},
};

pub struct Api<'a> {
    client: &'a Client,
    base_url: &'a String,
    api_secret: String,
    access_key: String,
}

impl Api<'_> {
    pub fn new<'a>(
        client: &'a Client,
        base_url: &'a String,
        api_secret: String,
        access_key: String,
    ) -> Api<'a> {
        Api {
            client,
            base_url,
            api_secret,
            access_key,
        }
    }

    pub async fn list_currencies(&self) -> Result<Vec<Currency>, serde_json::Error> {
        let timestamp = get_timestamp();
        let prehash = format!("{}{}{}{}", &timestamp, "GET", "/rest/v3/currencies", "");
        let signature = create_signature(&prehash, &self.api_secret);
        let url = format!("{}/currencies", &self.base_url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-FB-ACCESS-KEY",
            reqwest::header::HeaderValue::from_str(&self.access_key).unwrap(),
        );
        headers.insert(
            "X-FB-ACCESS-TIMESTAMP",
            HeaderValue::from_str(&timestamp).unwrap(),
        );
        headers.insert(
            "X-FB-ACCESS-SIGNATURE",
            HeaderValue::from_str(&signature).unwrap(),
        );

        let response = match self.client.get(&url).headers(headers).send().await {
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
}
