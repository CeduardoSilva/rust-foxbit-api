#[cfg(test)]
mod tests {
    use rust_foxbit_api::{
        new_hello_crate,
        types::{Category, Currency, DepositInfo, WithdrawInfo},
    };

    #[tokio::test]
    async fn test_list_currencies() {
        let foxbit = new_hello_crate();
        let currency = Currency {
            symbol: Some("btc".into()),
            name: Some("Bitcoin".into()),
            r#type: Some("CRYPTO".into()),
            precision: 8,
            deposit_info: Some(DepositInfo {
                min_to_confirm: Some("1".into()),
                min_amount: Some("0.0001".into()),
            }),
            withdraw_info: Some(WithdrawInfo {
                enabled: Some(true),
                min_amount: Some("0.0001".into()),
                fee: Some("0.0001".into()),
            }),
            category: Some(Category {
                code: Some("cripto".into()),
                name: Some("Cripto".into()),
            }),
        };
        let expected_result = vec![currency];
        let actual_result = foxbit.list_currencies().await.unwrap();
        assert_eq!(actual_result, expected_result);
    }
}
