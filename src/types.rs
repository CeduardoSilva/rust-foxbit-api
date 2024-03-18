use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WithdrawInfo {
    pub enabled: Option<bool>,
    pub min_amount: Option<String>,
    pub fee: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositInfo {
    pub min_to_confirm: Option<String>,
    pub min_amount: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub precision: usize,
    pub deposit_info: Option<DepositInfo>,
    pub withdraw_info: Option<WithdrawInfo>,
    pub category: Option<Category>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub symbol: Option<String>,
    pub quantity_min: Option<String>,
    pub quantity_increment: Option<String>,
    pub price_min: Option<String>,
    pub price_increment: Option<String>,
    pub base: Currency,
    pub quote: Currency,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub side: Option<String>,
    pub market_symbol: Option<String>,
    pub base_amount: Option<String>,
    pub quote_amount: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FoxBitResponse<T> {
    pub data: T,
}
