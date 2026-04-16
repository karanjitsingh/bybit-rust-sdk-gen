//! Place and query orders using Bybit V5 API (requires API key)
//!
//! Set environment variables:
//!   BYBIT_API_KEY=your_key
//!   BYBIT_API_SECRET=your_secret

use bybit_rust_sdk::client::BaseRestClient;
use bybit_rust_sdk::client::RestClientV5::RestClientV5;
use bybit_rust_sdk::client::config::ClientConfig;
use bybit_rust_sdk::types::request::v5_account::GetWalletBalanceParamsV5;
use bybit_rust_sdk::types::request::v5_trade::{GetAccountOrdersParamsV5, OrderParamsV5};
use bybit_rust_sdk::types::shared_v5::{AccountTypeV5, CategoryV5, OrderSideV5, OrderTypeV5};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("BYBIT_API_KEY").expect("Set BYBIT_API_KEY");
    let api_secret = std::env::var("BYBIT_API_SECRET").expect("Set BYBIT_API_SECRET");

    let config = ClientConfig::builder()
        .testnet(true)
        .api_key(api_key)
        .api_secret(api_secret)
        .enable_time_sync(true)
        .build();

    let base = BaseRestClient::new(config)?;
    base.sync_time().await?;
    println!("Time synced");

    let client = RestClientV5::new(base);

    // Get wallet balance
    let balance = client
        .get_wallet_balance(GetWalletBalanceParamsV5 {
            account_type: AccountTypeV5::UNIFIED,
            ..Default::default()
        })
        .await?;
    println!("Balance: {}", serde_json::to_string_pretty(&balance)?);

    // Place a limit buy order (small amount on testnet)
    let order = client
        .submit_order(OrderParamsV5 {
            category: CategoryV5::spot,
            symbol: "BTCUSDT".into(),
            side: OrderSideV5::Buy,
            order_type: OrderTypeV5::Limit,
            qty: "0.001".into(),
            price: Some("20000".into()),
            ..Default::default()
        })
        .await;

    match order {
        Ok(result) => println!("\nOrder placed: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => {
            if let Some(code) = e.ret_code() {
                println!("Bybit error code {}: {}", code, e);
            } else {
                println!("Error: {}", e);
            }
        }
    }

    // Get open orders
    let orders = client
        .get_active_orders(GetAccountOrdersParamsV5 {
            category: CategoryV5::spot,
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await?;
    println!("\nOpen orders: {}", serde_json::to_string_pretty(&orders)?);

    Ok(())
}
