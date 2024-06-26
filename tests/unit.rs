#[cfg(test)]
mod tests {
    use reqwest::Client;
    use rust_foxbit_api::Foxbit;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: reqwest::Client = reqwest::Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_currencies().await;
        assert!(result.is_ok());

        let currencies = result.unwrap();
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0].symbol, Some("BTC".to_string()));
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

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: reqwest::Client = reqwest::Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_markets().await;
        assert!(result.is_ok());

        let markets = result.unwrap();
        assert_eq!(markets.len(), 1);
        assert_eq!(markets[0].symbol, Some("usdtbrl".to_string()));
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

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .get_market_quotation("buy", "usdt", "brl", Some("40"), None)
            .await;

        // Assert the request was successful
        assert!(result.is_ok());

        let quote = result.unwrap();
        assert_eq!(quote.market_symbol, Some("usdtbrl".into()));
    }

    #[tokio::test]
    async fn test_get_order_book() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/BTCBRL/orderbook"))
            .and(query_param("market_symbol", "BTCBRL"))
            .and(query_param("depth", "50"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                      "sequence_id": 1234567890,
                      "bids": [
                        [
                          "3.00000000",
                          "300.00000000"
                        ],
                        [
                          "1.70000000",
                          "310.00000000"
                        ]
                      ],
                      "asks": [
                        [
                          "3.00000000",
                          "300.00000000"
                        ],
                        [
                          "2.00000000",
                          "321.00000000"
                        ]
                      ]
                    }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_order_book("BTCBRL", 50).await;

        assert!(result.is_ok());

        let orderbook = result.unwrap();
        assert_eq!(orderbook.sequence_id, 1234567890_u32);
    }

    #[tokio::test]
    async fn test_get_candles() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/BTCBRL/candles"))
            .and(query_param("interval", "1d"))
            .and(query_param("start_time", "2022-07-18T00:00"))
            .and(query_param("end_time", "2022-08-19T12:00"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([
                        [
                            "1692918060000",
                            "127772.05150000",
                            "128467.99980000",
                            "127750.01000000",
                            "128353.99990000",
                            "0.17080431"
                        ],
                        [
                            "1692921660000",
                            "128353.99990000",
                            "128353.99990000",
                            "127922.00030000",
                            "128339.99990000",
                            "0.12355465"
                        ]
                    ]))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .get_candles("BTCBRL", "1d", "2022-07-18T00:00", "2022-08-19T12:00")
            .await;

        assert!(result.is_ok());

        let candles = result.unwrap();
        assert_eq!(candles.len(), 2);
    }

    #[tokio::test]
    async fn test_get_candlesticks() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/markets/BTCBRL/candlesticks"))
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

        let api_url = mock_server.uri();
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit
            .get_candlesticks("BTCBRL", "1d", "2022-07-18T00:00", "2022-08-19T12:00")
            .await;

        assert!(result.is_ok());

        let candlesticks = result.unwrap();
        assert_eq!(candlesticks.len(), 2);
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

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_banks().await;

        assert!(result.is_ok());

        let banks = result.unwrap();
        assert_eq!(banks.len(), 1);
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

        let api_url = mock_server.uri();
        // let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_current_time().await;

        assert!(result.is_ok());

        let current_time = result.unwrap();
        assert_eq!(current_time.iso, "2021-06-15T18:00:00.123Z");
        assert_eq!(current_time.timestamp, 1637342699407_u64);
    }

    #[tokio::test]
    async fn test_get_current_member_details() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({
                      "sn": "FRR3DTLHGJ7DPB",
                      "email": "nakamoto.satoshi@example.com",
                      "level": 10,
                      "created_at": "2021-11-25T13:23:27.961Z",
                      "disabled": false
                    }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_current_member_details().await;

        assert!(result.is_ok());

        let current_member_details = result.unwrap();
        assert_eq!(current_member_details.sn, "FRR3DTLHGJ7DPB");
        assert_eq!(current_member_details.email, "nakamoto.satoshi@example.com");
        assert_eq!(current_member_details.level, 10);
        assert_eq!(
            current_member_details.created_at,
            "2021-11-25T13:23:27.961Z"
        );
        assert_eq!(current_member_details.disabled, false);
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
                          "trades_count": "2",
                          "remark": "A remarkable note for the order.",
                          "funds_received": "290.0"
                        }
                      ]
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
            .list_orders(
                "2022-07-18T00:00",
                "2022-08-19T12:00",
                10,
                1,
                "btcbrl",
                "ACTIVE",
                "BUY",
            )
            .await;

        assert!(result.is_ok());

        let orders = result.unwrap();
        assert_eq!(orders.len(), 1)
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
                      "trades_count": "2",
                      "remark": "A remarkable note for the order.",
                      "funds_received": "290.0"
                    }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_order_by_id("1234567890").await;

        assert!(result.is_ok());

        let order = result.unwrap();
        assert_eq!(order.id, "1234567890");
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
                      "trades_count": "2",
                      "remark": "A remarkable note for the order.",
                      "funds_received": "290.0"
                    }))
                    .insert_header("content-type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let api_url = mock_server.uri();
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.get_order_by_client_id("451637946501").await;

        assert!(result.is_ok());

        let order = result.unwrap();
        assert_eq!(order.client_order_id, "451637946501");
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
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment to test Foxbit Production.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.cancel_orders("ALL").await;

        assert!(result.is_ok());

        let cancel_order_response = result.unwrap();
        assert_eq!(cancel_order_response[0].sn, "OKMAKSDHRVVREK");
    }
}
