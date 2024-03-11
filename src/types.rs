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
pub struct FoxBitResponse {
    pub data: Vec<Currency>,
}
