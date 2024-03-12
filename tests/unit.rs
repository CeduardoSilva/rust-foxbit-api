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
        let client: Client = Client::new();
        let foxbit = Foxbit::new(client, api_url);

        let result = foxbit.list_currencies().await;
        assert!(result.is_ok());

        let currencies = result.unwrap();
        assert_eq!(currencies.len(), 1); // or however many you mocked
        assert_eq!(currencies[0].symbol, Some("BTC".to_string()));
    }
}
