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
pub struct OrderBook {
    pub sequence_id: u32,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FoxBitResponse<T> {
    pub data: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Candlestick {
    pub open_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub close_time: String,
    pub volume: String,
    pub quote_asset_volume: String,
    pub number_of_trades: i32,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    pub abbreviation: String,
    pub name: String,
    pub code: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentTime {
    pub iso: String,
    pub timestamp: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberDetails {
    pub sn: String,
    pub email: String,
    pub level: usize,
    pub created_at: String,
    pub disabled: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub id: u64,
    pub sn: String,
    pub client_order_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    pub id: u64,
    pub sn: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub sn: String,
    pub market_symbol: String,
    pub client_order_id: String,
    pub side: String,
    pub r#type: String,
    pub state: String,
    pub price: String,
    pub price_avg: String,
    pub quantity: String,
    pub quantity_executed: String,
    pub instant_amount: String,
    pub instant_amount_executed: String,
    pub created_at: String,
    pub trades_count: String,
    pub remark: String,
    pub funds_received: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    pub id: u64,
    pub sn: String,
    pub order_id: String,
    pub market_symbol: String,
    pub side: String,
    pub price: String,
    pub quantity: String,
    pub fee: String,
    pub fee_currency_symbol: String,
    pub created_at: String,
    pub role: String,
}
