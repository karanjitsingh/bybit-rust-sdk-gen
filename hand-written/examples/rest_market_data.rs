//! Fetch public market data from Bybit V5 API (no API key needed)

use bybit_rust_sdk::client::BaseRestClient;
use bybit_rust_sdk::client::RestClientV5::RestClientV5;
use bybit_rust_sdk::client::config::ClientConfig;
use bybit_rust_sdk::types::request::v5_market::{
    GetOrderbookParamsV5, GetTickersParamsV5,
};
use bybit_rust_sdk::types::shared_v5::CategoryV5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder().testnet(true).build();
    let base = BaseRestClient::new(config)?;
    let client = RestClientV5::new(base);

    // Get BTC/USDT ticker
    let ticker = client
        .get_tickers(GetTickersParamsV5 {
            category: CategoryV5::spot,
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await?;
    println!("Ticker: {}", serde_json::to_string_pretty(&ticker)?);

    // Get orderbook
    let orderbook = client
        .get_orderbook(GetOrderbookParamsV5 {
            category: CategoryV5::spot,
            symbol: "BTCUSDT".into(),
            limit: Some(5.0),
        })
        .await?;
    println!("\nOrderbook: {}", serde_json::to_string_pretty(&orderbook)?);

    // Get server time
    let time = client.get_server_time().await?;
    println!("\nServer time: {}", time);

    Ok(())
}
