use crate::{
    api::Api,
    types::{
        Bank, CancelOrderResponse, Candlestick, CreateOrderResponse, Currency, CurrentTime, Market,
        MemberDetails, Order, OrderBook, Quote, Trade,
    },
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

    pub async fn list_banks(&self) -> Result<Vec<Bank>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let banks = api.list_banks().await;
        banks
    }

    pub async fn get_current_time(&self) -> Result<CurrentTime, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let current_time = api.get_current_time().await;
        current_time
    }

    pub async fn get_current_member_details(&self) -> Result<MemberDetails, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let member_details = api.get_current_member_details().await;
        member_details
    }

    pub async fn create_order(
        &self,
        side: &str,
        r#type: &str,
        market_symbol: &str,
        quantity: &str,
        client_order_id: Option<&str>,
        remark: Option<&str>,
    ) -> Result<CreateOrderResponse, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let response = api
            .create_order(
                side,
                r#type,
                market_symbol,
                quantity,
                client_order_id,
                remark,
            )
            .await;
        response
    }

    pub async fn list_orders(
        &self,
        start_time: &str,
        end_time: &str,
        page_size: usize,
        page: usize,
        market_symbol: &str,
        state: &str,
        side: &str,
    ) -> Result<Vec<Order>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let orders = api
            .list_orders(
                start_time,
                end_time,
                page_size,
                page,
                market_symbol,
                state,
                side,
            )
            .await;
        orders
    }

    pub async fn get_order_by_id(&self, order_id: &str) -> Result<Order, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let order = api.get_order_by_id(order_id).await;
        order
    }

    pub async fn get_order_by_client_id(
        &self,
        client_order_id: &str,
    ) -> Result<Order, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let order = api.get_order_by_client_id(client_order_id).await;
        order
    }

    pub async fn cancel_orders(
        &self,
        r#type: &str,
    ) -> Result<Vec<CancelOrderResponse>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let cancel_order_response = api.cancel_orders(r#type).await;
        cancel_order_response
    }

    pub async fn list_trades(
        &self,
        start_time: &str,
        end_time: &str,
        page_size: usize,
        page: usize,
        market_symbol: &str,
    ) -> Result<Vec<Trade>, serde_json::Error> {
        dotenv().ok();
        let api_secret = env::var("API_SECRET").expect("API secret not found");
        let access_key = env::var("ACCESS_KEY").expect("Access key not found");

        let api = Api::new(&self.http_client, &self.api_url, api_secret, access_key);
        let listed_trades = api
            .list_trades(start_time, end_time, page_size, page, market_symbol)
            .await;
        listed_trades
    }
}
