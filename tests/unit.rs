#[cfg(test)]
mod tests {
    use mockito::{mock, server_address};
    use reqwest::Client;
    use rust_foxbit_api::Foxbit;
    use serde_json::json;

    #[tokio::test]
    async fn test_list_currencies() {
        let _mock = mock("GET", "/currencies")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "data": [
                        {
                            "symbol": "BTC",
                            "name": "Bitcoin",
                            "type": "crypto",
                            "precision": 8,
                        },
                    ]
                })
                .to_string(),
            )
            .create();

        let api_url = format!("http://{}", server_address());
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment this line to test the real Foxbit endpoint and comment the mocking.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_currencies().await;
        assert!(result.is_ok());

        let currencies = result.unwrap();
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0].symbol, Some("BTC".to_string()));
    }

    #[tokio::test]
    async fn test_list_markets() {
        let _mock = mock("GET", "/markets")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
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
                })
                .to_string(),
            )
            .create();

        let api_url = format!("http://{}", server_address());
        //let api_url = "https://api.foxbit.com.br/rest/v3".into(); // Uncomment this line to test the real Foxbit endpoint and comment the mocking.
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_markets().await;
        assert!(result.is_ok());

        let markets = result.unwrap();
        assert_eq!(markets.len(), 1);
        assert_eq!(markets[0].symbol, Some("usdtbrl".to_string()));
    }
}
