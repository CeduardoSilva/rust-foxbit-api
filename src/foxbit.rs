use crate::{
    api::Api,
    types::{Currency, Market},
};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

pub struct Foxbit {
    http_client: Client,
    api_url: String,
}

impl Foxbit {
    pub fn new(http_client: Client, api_url: String) -> Self {
        Foxbit {
            http_client,
            api_url,
        }
    }

    pub async fn list_currencies(&self) -> Result<Vec<Currency>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);

        let currencies = api.list_currencies().await;
        currencies
    }

    pub async fn list_markets(&self) -> Result<Vec<Market>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);

        let markets = api.list_markets().await;
        markets
    }
}
