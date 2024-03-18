use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::Error;
use std::collections::HashMap;

use crate::{
    helpers::{create_signature, get_prehash, get_timestamp},
    types::{Currency, FoxBitResponse, Market, OrderBook, Quote},
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
        let endpoint = "/currencies";
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(endpoint, None);
        let response = self.send_get_request(&url, headers, None).await;

        let json_response = serde_json::from_str::<FoxBitResponse<Vec<Currency>>>(&response);
        match json_response {
            Ok(json) => Ok(json.data),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn list_markets(&self) -> Result<Vec<Market>, serde_json::Error> {
        let endpoint = "/markets";
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(endpoint, None);
        let response = self.send_get_request(&url, headers, None).await;

        let json_response = serde_json::from_str::<FoxBitResponse<Vec<Market>>>(&response);
        match json_response {
            Ok(json) => Ok(json.data),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_market_quotation(
        &self,
        side: &str,
        base_currency: &str,
        quote_currency: &str,
        quantity: Option<&str>,
        amount: Option<&str>,
    ) -> Result<Quote, serde_json::Error> {
        if quantity.is_none() && amount.is_none() {
            return Err(serde_json::Error::custom("Must receive quantity or amount"));
        }
        let mut query_params: HashMap<&str, &str> = HashMap::new();
        query_params.insert("side", side);
        query_params.insert("base_currency", base_currency);
        query_params.insert("quote_currency", quote_currency);

        if let Some(qty) = quantity {
            query_params.insert("quantity", qty);
        }
        if let Some(amt) = amount {
            query_params.insert("amount", amt);
        }

        let query_string = self.build_query_string(&query_params);
        let endpoint = "/markets/quotes";
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(endpoint, Some(query_string));
        let response = self
            .send_get_request(&url, headers, Some(&query_params))
            .await;

        let json_response = serde_json::from_str::<Quote>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_order_book(
        &self,
        market_symbol: &str,
        depth: u8,
    ) -> Result<OrderBook, serde_json::Error> {
        let depth_str = format!("{}", depth);
        let mut query_params: HashMap<&str, &str> = HashMap::new();
        query_params.insert("market_symbol", market_symbol);
        query_params.insert("depth", depth_str.as_str());
        let query_string = self.build_query_string(&query_params);

        let endpoint = format!("/markets/{}/orderbook", market_symbol);
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, Some(query_string));
        let response = self
            .send_get_request(&url, headers, Some(&query_params))
            .await;

        let json_response = serde_json::from_str::<OrderBook>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    fn get_headers(&self, endpoint: &str, query_string: Option<String>) -> HeaderMap {
        let timestamp = get_timestamp();
        let prehash = match &query_string {
            Some(qs) => get_prehash(endpoint, &timestamp, Some(qs)),
            None => get_prehash(endpoint, &timestamp, None),
        };
        let signature = create_signature(&prehash, &self.api_secret);
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
        headers
    }

    async fn send_get_request(
        &self,
        url: &str,
        headers: HeaderMap,
        query_params: Option<&HashMap<&str, &str>>,
    ) -> String {
        let request_builder = self.client.get(url).headers(headers);

        let request_builder = if let Some(params) = query_params {
            request_builder.query(&params)
        } else {
            request_builder
        };

        match request_builder.send().await {
            Ok(resp) => match resp.text().await {
                Ok(text_response) => text_response,
                Err(e) => {
                    eprintln!("Converting Foxbit response to text failed: {}", e);
                    e.to_string()
                }
            },
            Err(e) => {
                eprintln!("Request to Foxbit failed: {}", e);
                e.to_string()
            }
        }
    }

    fn build_query_string(&self, query_params: &HashMap<&str, &str>) -> String {
        query_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, utf8_percent_encode(value, NON_ALPHANUMERIC)))
            .collect::<Vec<String>>()
            .join("&")
    }
}
