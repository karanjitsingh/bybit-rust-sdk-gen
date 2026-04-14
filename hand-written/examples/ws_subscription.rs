//! Subscribe to real-time WebSocket streams from Bybit
//!
//! Demonstrates public orderbook subscription with auto-reconnect.
//! For private streams (positions, orders), set BYBIT_API_KEY and BYBIT_API_SECRET.

use bybit_rust_sdk::client::config::ClientConfig;
use bybit_rust_sdk::client::BaseWebsocketClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder().testnet(true).build();
    let ws = BaseWebsocketClient::new(config)?;

    // Connect to spot public stream
    ws.connect("v5SpotPublic").await?;
    println!("Connected to spot public WS");

    // Subscribe to BTC/USDT orderbook (depth 1 = best bid/ask)
    ws.subscribe(vec![
        "orderbook.1.BTCUSDT".to_string(),
        "tickers.BTCUSDT".to_string(),
    ]).await?;
    println!("Subscribed to orderbook and tickers");

    // Read events — auto-reconnect happens in background
    let mut count = 0;
    while let Some((ws_key, msg)) = ws.recv().await {
        // Internal events for connection lifecycle
        if msg.contains("\"_internal\"") {
            println!("[{}] {}", ws_key, msg);
            continue;
        }

        let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();

        if let Some(topic) = json.get("topic").and_then(|v| v.as_str()) {
            match topic {
                t if t.starts_with("orderbook") => {
                    if let Some(data) = json.get("data") {
                        let bids = data.get("b").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                        let asks = data.get("a").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                        println!("[orderbook] {} bids, {} asks", bids, asks);
                    }
                }
                t if t.starts_with("tickers") => {
                    if let Some(data) = json.get("data") {
                        let last = data.get("lastPrice").and_then(|v| v.as_str()).unwrap_or("?");
                        let vol = data.get("volume24h").and_then(|v| v.as_str()).unwrap_or("?");
                        println!("[ticker] BTC/USDT last={} vol24h={}", last, vol);
                    }
                }
                _ => println!("[{}] {}", topic, msg),
            }
        }

        count += 1;
        if count >= 20 {
            println!("\nReceived 20 messages, shutting down...");
            break;
        }
    }

    ws.shutdown().await;
    Ok(())
}
