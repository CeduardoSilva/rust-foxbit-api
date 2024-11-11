#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use reqwest::Client;
    use rust_foxbit_api::Foxbit;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use std::env;

    #[tokio::test]
    async fn test_list_currencies() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/currencies"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [
                        {
                            "symbol": "BTC",
                            "name": "Bitcoin",
                            "type": "crypto",
                            "precision": 8,
                        },
                    ]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        // Check the environment variable
        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_currencies().await;
        assert!(result.is_ok());

        let currencies = result.unwrap();
        assert!(!currencies.is_empty(), "Currencies list should not be empty");
        
        for currency in currencies {
            assert!(currency.symbol.is_some(), "Currency symbol should be present");
            assert!(currency.name.is_some(), "Currency name should be present");
            assert!(currency.r#type.is_some(), "Currency type should be present");
        }
    }


    #[tokio::test]
    async fn test_list_markets() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [
                        {
                            "symbol": "usdtbrl",
                            "quantity_min": "0.00002",
                            "quantity_increment": "0.00001",
                            "price_min": "1.0",
                            "price_increment": "0.0001",
                            "base": {
                                "symbol": "btc",
                                "name": "Bitcoin",
                                "type": "CRYPTO",
                                "precision": 8,
                                "deposit_info": {},
                                "withdraw_info": {},
                                "category": {}
                            },
                            "quote": {
                                "symbol": "btc",
                                "name": "Bitcoin",
                                "type": "CRYPTO",
                                "precision": 8,
                                "deposit_info": {},
                                "withdraw_info": {},
                                "category": {}
                            }
                        }
                    ]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_markets().await;
        assert!(result.is_ok());

        let markets = result.unwrap();
        assert!(!markets.is_empty(), "Markets list should not be empty");

        for market in markets {
            assert!(market.symbol.is_some(), "Market symbol should be present");
            assert!(market.quantity_min.expect("REASON").parse::<f64>().is_ok(), "Quantity min should be a valid number");
            assert!(market.quantity_increment.expect("REASON").parse::<f64>().is_ok(), "Quantity increment should be a valid number");
            assert!(market.price_min.expect("REASON").parse::<f64>().is_ok(), "Price min should be a valid number");
            assert!(market.price_increment.expect("REASON").parse::<f64>().is_ok(), "Price increment should be a valid number");

            let base = &market.base;
            assert!(base.symbol.is_some(), "Base asset symbol should be present");
            assert!(base.name.is_some(), "Base asset name should be present");
            assert!(base.r#type.is_some(), "Base asset type should be present");
            assert_ne!(base.precision, 0, "Base precision should be a non-negative integer");

            let quote = &market.quote;
            assert!(quote.symbol.is_some(), "Quote asset symbol should be present");
            assert!(quote.name.is_some(), "Quote asset name should be present");
            assert!(quote.r#type.is_some(), "Quote asset type should be present");
            assert_ne!(quote.precision, 0, "Quote precision should be a non-negative integer");
        }
    }

    #[tokio::test]
    async fn test_get_market_quotation() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/quotes"))
            .and(query_param("quote_currency", "brl"))
            .and(query_param("base_currency", "usdt"))
            .and(query_param("side", "buy"))
            .and(query_param("quantity", "40"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "side": "buy",
                    "market_symbol": "usdtbrl",
                    "base_amount": "40",
                    "quote_amount": "201.58",
                    "price": "5.0395"
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .get_market_quotation("buy", "usdt", "brl", Some("40"), None)
            .await;

        assert!(result.is_ok());

        let quote = result.unwrap();

        assert!(quote.side.is_some(), "Side should be present");
        assert!(quote.market_symbol.is_some(), "Market symbol should be present");
        assert!(quote.base_amount.is_some(), "Base amount should be present");
        assert!(quote.quote_amount.is_some(), "Quote amount should be present");
        assert!(quote.price.is_some(), "Price should be present");

        assert!(quote.base_amount.as_ref().unwrap().parse::<f64>().is_ok(), "Base amount should be a valid number");
        assert!(quote.quote_amount.as_ref().unwrap().parse::<f64>().is_ok(), "Quote amount should be a valid number");
        assert!(quote.price.as_ref().unwrap().parse::<f64>().is_ok(), "Price should be a valid number");
    }

    #[tokio::test]
    async fn test_get_order_book() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/btcbrl/orderbook"))
            .and(query_param("depth", "50"))
            .respond_with(
                ResponseTemplate::new(200)
                        .set_body_json(json!({
                            "sequence_id": 1234567890,
                            "timestamp": 1234567890,
                            "bids": [
                                ["3.00000000", "300.00000000"],
                                ["1.70000000", "310.00000000"]
                            ],
                            "asks": [
                                ["3.00000000", "300.00000000"],
                                ["2.00000000", "321.00000000"]
                            ]
                        }))
                            .insert_header("content-type", "application/json"),
                    )
                    .mount(&mock_server)
                    .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let mut query_params = BTreeMap::new();
        query_params.insert("depth", "50");

        let result = foxbit.get_order_book("btcbrl", 50).await;
        assert!(result.is_ok());

        let orderbook = result.unwrap();

        assert!(orderbook.sequence_id > 0, "Sequence ID should be a positive integer");
        assert!(orderbook.timestamp > 0, "Timestamp should be a positive integer");
        assert!(!orderbook.bids.is_empty(), "Bids should not be empty");
        assert!(!orderbook.asks.is_empty(), "Asks should not be empty");

        for bid in &orderbook.bids {
            assert_eq!(bid.len(), 2, "Each bid should contain a price and quantity");
            assert!(bid[0].parse::<f64>().is_ok(), "Bid price should be a valid number");
            assert!(bid[1].parse::<f64>().is_ok(), "Bid quantity should be a valid number");
        }

        for ask in &orderbook.asks {
            assert_eq!(ask.len(), 2, "Each ask should contain a price and quantity");
            assert!(ask[0].parse::<f64>().is_ok(), "Ask price should be a valid number");
            assert!(ask[1].parse::<f64>().is_ok(), "Ask quantity should be a valid number");
        }
    }

    #[tokio::test]
    async fn test_get_candlesticks() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/btcbrl/candlesticks"))
            .and(query_param("interval", "1d"))
            .and(query_param("start_time", "2022-07-18T00:00"))
            .and(query_param("end_time", "2022-08-19T12:00"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([
                    [
                        "1692918000000",
                        "127772.05150000",
                        "128467.99980000",
                        "127750.01000000",
                        "128353.99990000",
                        "1692918060000",
                        "0.17080431",
                        "21866.35948786",
                        66,
                        "0.12073605",
                        "15466.34096391"
                    ],
                    [
                        "1692921600000",
                        "128353.99990000",
                        "128353.99990000",
                        "127922.00030000",
                        "128339.99990000",
                        "1692921660000",
                        "0.12355465",
                        "15851.30631056",
                        45,
                        "0.11030870",
                        "14156.75206627"
                    ]
                ]))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .get_candlesticks("btcbrl", "1d", "2022-07-18T00:00", "2022-08-19T12:00")
            .await;

        assert!(result.is_ok());

        let candlesticks = result.unwrap();
        assert!(!candlesticks.is_empty(), "Candlesticks list should not be empty");

        for candlestick in candlesticks {
            assert!(candlestick.open_time.parse::<i64>().is_ok(), "Open time should be a valid timestamp");
            assert!(candlestick.close_time.parse::<i64>().is_ok(), "Close time should be a valid timestamp");

            assert!(candlestick.open_price.parse::<f64>().is_ok(), "Open price should be a valid number");
            assert!(candlestick.high_price.parse::<f64>().is_ok(), "High price should be a valid number");
            assert!(candlestick.low_price.parse::<f64>().is_ok(), "Low price should be a valid number");
            assert!(candlestick.close_price.parse::<f64>().is_ok(), "Close price should be a valid number");

            assert!(candlestick.volume.parse::<f64>().is_ok(), "Volume should be a valid number");
            assert!(candlestick.quote_asset_volume.parse::<f64>().is_ok(), "Quote asset volume should be a valid number");
            assert!(candlestick.number_of_trades >= 0, "Number of trades should be a non-negative integer");
            assert!(candlestick.taker_buy_base_asset_volume.parse::<f64>().is_ok(), "Taker buy base asset volume should be a valid number");
            assert!(candlestick.taker_buy_quote_asset_volume.parse::<f64>().is_ok(), "Taker buy quote asset volume should be a valid number");
        }
    }

    #[tokio::test]
    async fn test_list_banks() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/banks"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [
                        {
                            "abbreviation": "BB",
                            "name": "Banco do Brasil",
                            "code": 1
                        }
                    ]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_banks().await;
        assert!(result.is_ok());

        let banks = result.unwrap();
        assert!(!banks.is_empty(), "Banks list should not be empty");

        for bank in banks {
            assert!(!bank.abbreviation.is_empty(), "Abbreviation should not be empty");
            assert!(!bank.name.is_empty(), "Bank name should not be empty");
            assert!(bank.code > 0, "Bank code should be a positive integer");
        }
    }

    #[tokio::test]
    async fn test_get_current_time() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/system/time"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "iso": "2021-06-15T18:00:00.123Z",
                    "timestamp": 1637342699407_u64
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_current_time().await;
        assert!(result.is_ok());

        let current_time = result.unwrap();

        assert!(!current_time.iso.is_empty(), "ISO timestamp should not be empty");
        assert!(current_time.iso.ends_with('Z'), "ISO timestamp should end with 'Z'");
        assert!(current_time.timestamp > 0, "Timestamp should be a positive integer");
    }

    #[tokio::test]
    async fn test_get_current_member_details() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "sn": "FTEF4ISD4SV7QB",
                    "email": "cs.eduardo@icloud.com",
                    "level": 30,
                    "created_at": "2018-07-10T17:45:18.000Z",
                    "disabled": false
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_current_member_details().await;
        assert!(result.is_ok());

        let current_member_details = result.unwrap();

        assert!(!current_member_details.sn.is_empty(), "SN should not be empty");
        assert!(current_member_details.email.contains('@'), "Email should be a valid email format");
        assert!(current_member_details.level > 0, "Level should be a positive integer");
        assert!(!current_member_details.created_at.is_empty(), "Created_at should not be empty");
        assert!(!current_member_details.disabled, "Disabled should be false");
    }

    #[tokio::test]
    async fn test_create_order() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/orders"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                      "id": 1234567890,
                      "sn": "OKMAKSDHRVVREK",
                      "client_order_id": "451637946501"
                    }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .create_order(
                "BUY",
                "MARKET",
                "btcbrl",
                "0.42",
                Some("123456789"),
                Some("remark"),
            )
            .await;

        assert!(result.is_ok());

        let create_order_response = result.unwrap();
        assert_eq!(create_order_response.id, 1234567890);
        assert_eq!(create_order_response.sn, "OKMAKSDHRVVREK");
        assert_eq!(create_order_response.client_order_id, "451637946501")
    }

    #[tokio::test]
    async fn test_list_orders() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orders"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [
                        {
                            "id": "1234567890",
                            "sn": "OKMAKSDHRVVREK",
                            "client_order_id": "451637946501",
                            "market_symbol": "btcbrl",
                            "side": "BUY",
                            "type": "LIMIT",
                            "state": "ACTIVE",
                            "price": "290000.0",
                            "price_avg": "295333.3333",
                            "quantity": "0.42",
                            "quantity_executed": "0.41",
                            "instant_amount": "290.0",
                            "instant_amount_executed": "290.0",
                            "created_at": "2021-02-15T22:06:32.999Z",
                            "trades_count": 2,
                            "remark": "A remarkable note for the order.",
                            "funds_received": "290.0"
                        }
                    ]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .list_orders(
                "2024-08-28T00:00:00.000Z",
                "2024-08-29T20:00:22.013Z",
                10,
                1,
                "btcbrl",
                "FILLED",
                "BUY",
            )
            .await;

        assert!(result.is_ok());

        let orders = result.unwrap();
        assert!(!orders.is_empty(), "Orders list should not be empty");

        for order in orders {
            assert!(!order.id.is_empty(), "Order ID should not be empty");
            assert!(!order.sn.is_empty(), "SN should not be empty");
            assert!(!order.market_symbol.is_empty(), "Market symbol should not be empty");
            assert!(["BUY", "SELL"].contains(&order.side.as_str()), "Side should be either BUY or SELL");
            assert!(["LIMIT", "MARKET"].contains(&order.r#type.as_str()), "Type should be either LIMIT or MARKET");
            assert!(!order.state.is_empty(), "State should not be empty");
            assert!(order.price_avg.parse::<f64>().is_ok(), "Price avg should be a valid number");
            assert!(order.quantity.parse::<f64>().is_ok(), "Quantity should be a valid number");
            assert!(order.quantity_executed.parse::<f64>().is_ok(), "Quantity executed should be a valid number");
            assert!(order.created_at.ends_with('Z'), "Created_at should end with 'Z' for UTC format");
        }
    }

    #[tokio::test]
    async fn test_get_order_by_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orders/by-order-id/1234567890"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "id": "1234567890",
                    "sn": "OKMAKSDHRVVREK",
                    "client_order_id": "451637946501",
                    "market_symbol": "btcbrl",
                    "side": "BUY",
                    "type": "LIMIT",
                    "state": "ACTIVE",
                    "price": "290000.0",
                    "price_avg": "295333.3333",
                    "quantity": "0.42",
                    "quantity_executed": "0.41",
                    "instant_amount": "290.0",
                    "instant_amount_executed": "290.0",
                    "created_at": "2021-02-15T22:06:32.999Z",
                    "trades_count": 2,
                    "remark": "A remarkable note for the order.",
                    "funds_received": "290.0"
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_order_by_id("1234567890").await;
        assert!(result.is_ok());

        let order = result.unwrap();

        assert_eq!(order.id, "1234567890", "Order ID should match the requested ID");
        assert!(!order.sn.is_empty(), "SN should not be empty");
        assert!(!order.market_symbol.is_empty(), "Market symbol should not be empty");
        assert!(order.client_order_id.is_some(), "Client order ID should be present");
        assert!(["BUY", "SELL"].contains(&order.side.as_str()), "Side should be either BUY or SELL");
        assert!(["LIMIT", "MARKET"].contains(&order.r#type.as_str()), "Type should be either LIMIT or MARKET");
        assert!(!order.state.is_empty(), "State should not be empty");

        assert!(order.price.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Price should be a valid number if present");
        assert!(order.price_avg.parse::<f64>().is_ok(), "Price avg should be a valid number");
        assert!(order.quantity.parse::<f64>().is_ok(), "Quantity should be a valid number");
        assert!(order.quantity_executed.parse::<f64>().is_ok(), "Quantity executed should be a valid number");

        assert!(order.instant_amount.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Instant amount should be a valid number if present");
        assert!(order.instant_amount_executed.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Instant amount executed should be a valid number if present");

        assert!(order.created_at.ends_with('Z'), "Created_at should end with 'Z' for UTC format");
        assert!(order.trades_count >= 0, "Trades count should be a non-negative integer");
        assert!(order.cancellation_reason.is_none(), "Cancellation reason should be None for active orders");
    }

    #[tokio::test]
    async fn test_get_order_by_client_id() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/orders/by-client-order-id/451637946501"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "id": "1234567890",
                    "sn": "OKMAKSDHRVVREK",
                    "client_order_id": "451637946501",
                    "market_symbol": "btcbrl",
                    "side": "BUY",
                    "type": "LIMIT",
                    "state": "ACTIVE",
                    "price": "290000.0",
                    "price_avg": "295333.3333",
                    "quantity": "0.42",
                    "quantity_executed": "0.41",
                    "instant_amount": "290.0",
                    "instant_amount_executed": "290.0",
                    "created_at": "2021-02-15T22:06:32.999Z",
                    "trades_count": 2,
                    "remark": "A remarkable note for the order.",
                    "funds_received": "290.0"
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_order_by_client_id("451637946501").await;
        assert!(result.is_ok());

        let order = result.unwrap();

        assert_eq!(order.client_order_id.as_ref().unwrap(), "451637946501", "Client order ID should match the requested ID");
        assert!(!order.id.is_empty(), "Order ID should not be empty");
        assert!(!order.sn.is_empty(), "SN should not be empty");
        assert!(!order.market_symbol.is_empty(), "Market symbol should not be empty");

        assert!(["BUY", "SELL"].contains(&order.side.as_str()), "Side should be either BUY or SELL");
        assert!(["LIMIT", "MARKET"].contains(&order.r#type.as_str()), "Type should be either LIMIT or MARKET");
        assert!(!order.state.is_empty(), "State should not be empty");

        assert!(order.price.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Price should be a valid number if present");
        assert!(order.price_avg.parse::<f64>().is_ok(), "Price avg should be a valid number");
        assert!(order.quantity.parse::<f64>().is_ok(), "Quantity should be a valid number");
        assert!(order.quantity_executed.parse::<f64>().is_ok(), "Quantity executed should be a valid number");

        assert!(order.instant_amount.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Instant amount should be a valid number if present");
        assert!(order.instant_amount_executed.as_ref().map(|p| p.parse::<f64>().is_ok()).unwrap_or(true), "Instant amount executed should be a valid number if present");

        assert!(order.created_at.ends_with('Z'), "Created_at should end with 'Z' for UTC format");
        assert!(order.trades_count >= 0, "Trades count should be a non-negative integer");
    }

    #[tokio::test]
    async fn test_cancel_orders() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/orders/cancel"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [
                        {
                            "sn": "OKMAKSDHRVVREK",
                            "id": 123456789
                        }
                    ]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.cancel_orders("ALL").await;
        assert!(result.is_ok());

        let cancel_order_response = result.unwrap();
        assert!(!cancel_order_response.is_empty(), "Cancel order response should not be empty");

        for order in cancel_order_response {
            assert!(!order.sn.is_empty(), "SN should not be empty");
            assert!(order.id > 0, "Order ID should be a positive integer");
        }
    }

    #[tokio::test]
    async fn test_list_trades() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/trades"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                    "data": [{
                        "id": 1234567890,
                        "sn": "TC5JZVW2LLJ3IW",
                        "order_id": "1234567890",
                        "market_symbol": "btcbrl",
                        "side": "BUY",
                        "price": "290000.0",
                        "quantity": "1.0",
                        "fee": "0.01",
                        "fee_currency_symbol": "btc",
                        "created_at": "2021-02-15T22:06:32.999Z",
                        "role": "TAKER",
                    }]
                }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = match env::var("API_ENV") {
            Ok(env) if env == "production" => "https://api.foxbit.com.br/rest/v3".to_string(),
            _ => mock_server.uri(),
        };

        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .list_trades(
                "2024-08-28T00:00:00.000Z", // start_time
                "2024-08-29T20:00:22.013Z", // end_time
                10,                         // page_size
                1,                          // page
                "btcbrl",                   // market_symbol
            )
            .await;

        assert!(result.is_ok());

        let listed_trades = result.unwrap();
        assert!(!listed_trades.is_empty(), "Trades list should not be empty");

        for trade in listed_trades {
            assert!(trade.id > 0, "Trade ID should be a positive integer");
            assert!(!trade.sn.is_empty(), "SN should not be empty");
            assert!(!trade.order_id.is_empty(), "Order ID should not be empty");
            assert!(!trade.market_symbol.is_empty(), "Market symbol should not be empty");
            assert!(["BUY", "SELL"].contains(&trade.side.as_str()), "Side should be either BUY or SELL");

            assert!(trade.price.parse::<f64>().is_ok(), "Price should be a valid number");
            assert!(trade.quantity.parse::<f64>().is_ok(), "Quantity should be a valid number");
            assert!(trade.fee.parse::<f64>().is_ok(), "Fee should be a valid number");

            assert!(!trade.fee_currency_symbol.is_empty(), "Fee currency symbol should not be empty");
            assert!(trade.created_at.ends_with('Z'), "Created_at should end with 'Z' for UTC format");
            assert!(["TAKER", "MAKER"].contains(&trade.role.as_str()), "Role should be either TAKER or MAKER");
        }
    }
}
