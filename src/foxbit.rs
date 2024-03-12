use crate::types::Currency;
use dotenv::dotenv;
use std::env;

pub struct Foxbit {}

impl Foxbit {
    pub fn new() -> Self {
        Foxbit {}
    }

    pub async fn list_currencies(&self) -> Result<Vec<Currency>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");
        let api_url = env::var("FOXBIT_V3_API").expect("API URL not found");

        let currencies = crate::api::list_currencies(&api_url, &api_secret, &access_key).await;
        currencies
    }
}
