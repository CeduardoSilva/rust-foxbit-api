use crate::{
    api::Api,
    types::{Candlestick, Currency, Market, OrderBook, Quote},
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

    pub async fn get_market_quotation(
        &self,
        side: &str,
        base_currency: &str,
        quote_currency: &str,
        quantity: Option<&str>,
        amount: Option<&str>,
    ) -> Result<Quote, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);

        let quote = api
            .get_market_quotation(side, base_currency, quote_currency, quantity, amount)
            .await;
        quote
    }

    pub async fn get_order_book(
        &self,
        market_symbol: &str,
        depth: u8,
    ) -> Result<OrderBook, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let order_book = api.get_order_book(market_symbol, depth).await;
        order_book
    }

    pub async fn get_candles(
        &self,
        market_symbol: &str,
        interval: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<Vec<String>>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let candles = api
            .get_candles(market_symbol, interval, start_time, end_time)
            .await;
        candles
    }

    pub async fn get_candlesticks(
        &self,
        market_symbol: &str,
        interval: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<Candlestick>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let candlesticks = api
            .get_candlesticks(market_symbol, interval, start_time, end_time)
            .await;
        candlesticks
    }
}
