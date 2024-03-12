pub mod api;
pub mod foxbit;
pub mod helpers;
pub mod types;

use dotenv::dotenv;
use std::env;

pub use foxbit::Foxbit;

/// Creates a new instance of Foxbit.
pub fn new() -> Foxbit {
    dotenv().ok();
    let api_url = env::var("FOXBIT_V3_API").expect("API URL not found");
    let client: reqwest::Client = reqwest::Client::new();
    Foxbit::new(client, api_url)
}
