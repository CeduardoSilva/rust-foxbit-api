use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{de::Error, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    helpers::{create_signature, get_prehash, get_timestamp},
    types::{
        Bank, Candlestick, CreateOrderResponse, Currency, CurrentTime, FoxBitResponse, Market,
        MemberDetails, OrderBook, Quote,
    },
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
        let headers = self.get_headers(endpoint, None, None);
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
        let headers = self.get_headers(endpoint, None, None);
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
        let headers = self.get_headers(endpoint, Some(query_string), None);
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
        let headers = self.get_headers(&endpoint, Some(query_string), None);
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

    pub async fn get_candles(
        &self,
        market_symbol: &str,
        interval: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<Vec<String>>, serde_json::Error> {
        let mut query_params: HashMap<&str, &str> = HashMap::new();
        query_params.insert("interval", interval);
        query_params.insert("start_time", start_time);
        query_params.insert("end_time", end_time);
        let query_string = self.build_query_string(&query_params);

        let endpoint = format!("/markets/{}/candles", market_symbol);
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, Some(query_string), None);
        let response = self
            .send_get_request(&url, headers, Some(&query_params))
            .await;

        let json_response = serde_json::from_str::<Vec<Vec<String>>>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_candlesticks(
        &self,
        market_symbol: &str,
        interval: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<Candlestick>, serde_json::Error> {
        let mut query_params: HashMap<&str, &str> = HashMap::new();
        query_params.insert("interval", interval);
        query_params.insert("start_time", start_time);
        query_params.insert("end_time", end_time);
        let query_string = self.build_query_string(&query_params);

        let endpoint = format!("/markets/{}/candlesticks", market_symbol);
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, Some(query_string), None);
        let response = self
            .send_get_request(&url, headers, Some(&query_params))
            .await;

        let json_response = serde_json::from_str::<Vec<Candlestick>>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn list_banks(&self) -> Result<Vec<Bank>, serde_json::Error> {
        let endpoint = format!("/banks");
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, None, None);
        let response = self.send_get_request(&url, headers, None).await;

        let json_response = serde_json::from_str::<FoxBitResponse<Vec<Bank>>>(&response);
        match json_response {
            Ok(json) => Ok(json.data),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_current_time(&self) -> Result<CurrentTime, serde_json::Error> {
        let endpoint = format!("/system/time");
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, None, None);
        let response = self.send_get_request(&url, headers, None).await;

        let json_response = serde_json::from_str::<CurrentTime>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_current_member_details(&self) -> Result<MemberDetails, serde_json::Error> {
        let endpoint = format!("/me");
        let url = format!("{}{}", &self.base_url, endpoint);
        let headers = self.get_headers(&endpoint, None, None);
        let response = self.send_get_request(&url, headers, None).await;

        let json_response = serde_json::from_str::<MemberDetails>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
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
        let endpoint = format!("/orders");
        let url = format!("{}{}", &self.base_url, endpoint);
        let body = serde_json::json!({
            "side": side,
            "type": r#type,
            "market_symbol": market_symbol,
            "quantity": quantity,
            "client_order_id": client_order_id.unwrap(),
            "remark": remark.unwrap(),
        });
        let headers = self.get_headers(&endpoint, None, Some(&body));

        let response = match self.send_post_request(&url, headers, &body).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Request to Foxbit failed: {}", e);
                e.to_string()
            }
        };

        println!("Response: {}", response);

        let json_response = serde_json::from_str::<CreateOrderResponse>(&response);
        match json_response {
            Ok(json) => Ok(json),
            Err(e) => {
                eprintln!("Conversion to json failed: {}", e);
                Err(e)
            }
        }
    }

    fn get_headers(
        &self,
        endpoint: &str,
        query_string: Option<String>,
        body: Option<&Value>,
    ) -> HeaderMap {
        let timestamp = get_timestamp();
        let prehash = get_prehash(endpoint, &timestamp, query_string.as_deref(), body);
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

    async fn send_post_request<T: Serialize + ?Sized>(
        &self,
        url: &str,
        headers: HeaderMap,
        body: &T,
    ) -> Result<String, reqwest::Error> {
        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    fn build_query_string(&self, query_params: &HashMap<&str, &str>) -> String {
        query_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, utf8_percent_encode(value, NON_ALPHANUMERIC)))
            .collect::<Vec<String>>()
            .join("&")
    }
}
