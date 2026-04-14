# bybit-rust-sdk

⚠️ **AI Slop Disclaimer** ⚠️ Most of this code was generated using AI. Use at your own risk.

Unofficial Rust SDK for the [Bybit](https://bybit.com) V5 API. Auto-generated from the official [TypeScript SDK](https://github.com/tiagosiebler/bybit-api) with hand-written HTTP/WebSocket transport.

## Features

- **891 typed structs** — request params, response types, enums, all with serde
- **REST client** — full V5 API coverage (market data, trading, account, asset, etc.)
- **WebSocket client** — public and private streams with auto-reconnect
- **HMAC-SHA256 signing** — automatic for private endpoints
- **Time sync** — clock drift compensation with Bybit servers
- **Typed errors** — `BybitApiError` with `ret_code` / `ret_msg` from the API
- **Builder pattern** — all param structs derive `Builder` for ergonomic construction

## Quick Start

Add to `Cargo.toml`:
```toml
[dependencies]
bybit-rust-sdk = { path = "." }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

### Public Market Data (no API key)

```rust
use bybit_rust_sdk::client::{BaseRestClient, RestClientV5::RestClientV5, config::ClientConfig};
use bybit_rust_sdk::types::request::v5_market::GetTickersParamsV5;
use bybit_rust_sdk::types::shared_v5::CategoryV5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder().testnet(true).build();
    let base = BaseRestClient::new(config)?;
    let client = RestClientV5::new(&base);

    let ticker = client.get_tickers(GetTickersParamsV5 {
        category: CategoryV5::spot,
        symbol: Some("BTCUSDT".into()),
        ..Default::default()
    }).await?;

    println!("{}", serde_json::to_string_pretty(&ticker)?);
    Ok(())
}
```

### Authenticated Trading

```rust
use bybit_rust_sdk::client::{BaseRestClient, RestClientV5::RestClientV5, config::ClientConfig};
use bybit_rust_sdk::types::request::v5_trade::OrderParamsV5;
use bybit_rust_sdk::types::shared_v5::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder()
        .testnet(true)
        .api_key("YOUR_KEY")
        .api_secret("YOUR_SECRET")
        .enable_time_sync(true)
        .build();

    let base = BaseRestClient::new(config)?;
    base.sync_time().await?;
    let client = RestClientV5::new(&base);

    let result = client.submit_order(OrderParamsV5 {
        category: CategoryV5::spot,
        symbol: "BTCUSDT".into(),
        side: OrderSideV5::Buy,
        order_type: OrderTypeV5::Limit,
        qty: "0.001".into(),
        price: Some("20000".into()),
        ..Default::default()
    }).await;

    match result {
        Ok(order) => println!("Order: {:?}", order),
        Err(e) => {
            // Typed error with Bybit retCode
            if let Some(code) = e.ret_code() {
                eprintln!("Bybit error {}: {}", code, e);
            }
        }
    }
    Ok(())
}
```

### WebSocket Streams

```rust
use bybit_rust_sdk::client::{BaseWebsocketClient, config::ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder().testnet(true).build();
    let ws = BaseWebsocketClient::new(config)?;

    ws.connect("v5SpotPublic").await?;
    ws.subscribe(vec!["orderbook.1.BTCUSDT".into()]).await?;

    while let Some((ws_key, msg)) = ws.recv().await {
        println!("[{}] {}", ws_key, &msg[..msg.len().min(120)]);
    }
    Ok(())
}
```

## Architecture

```
bybit-rust-sdk/
├── src/
│   ├── client/
│   │   ├── mod.rs              # ClientError, BybitApiError, ClientResult
│   │   ├── config.rs           # ClientConfig + builder
│   │   ├── signing.rs          # HMAC-SHA256 signing
│   │   ├── BaseRestClient.rs   # HTTP transport, time sync
│   │   ├── BaseWebsocketClient.rs  # WS transport, reconnect
│   │   ├── RestClientV5.rs     # Generated — all V5 REST methods
│   │   ├── SpotClientV3.rs     # Generated — V3 spot methods
│   │   ├── WebsocketClient.rs  # Generated — WS subscribe/auth
│   │   └── WebsocketAPIClient.rs   # Generated — WS order placement
│   ├── types/                  # Generated — 891 typed structs/enums
│   │   ├── request/            # Request parameter types
│   │   ├── response/           # Response types
│   │   ├── websockets/         # WS event types
│   │   ├── shared.rs           # Shared types across versions
│   │   └── shared_v5.rs        # V5-specific shared types
│   ├── constants/              # API constants, WS keys
│   └── util/                   # Utility types
└── examples/
    ├── rest_market_data.rs     # Public REST API
    ├── rest_private_order.rs   # Authenticated trading
    └── ws_subscription.rs      # WebSocket streams
```

**Hand-written** (transport layer): `BaseRestClient.rs`, `BaseWebsocketClient.rs`, `config.rs`, `signing.rs`, `mod.rs`

**Auto-generated** (from TS SDK): Everything else — types, client methods, constants

## WebSocket Keys

| Key | Stream |
|-----|--------|
| `v5SpotPublic` | Spot market data |
| `v5LinearPublic` | Linear/USDT perpetuals |
| `v5InversePublic` | Inverse perpetuals |
| `v5OptionPublic` | Options |
| `v5Private` | Private (positions, orders, wallet) |
| `v5PrivateTrade` | Private trade execution (WS API orders) |

## Error Handling

```rust
use bybit_rust_sdk::client::ClientError;

match client.submit_order(params).await {
    Ok(result) => { /* success */ }
    Err(ClientError::Api(e)) => {
        // Bybit API error — e.ret_code, e.ret_msg, e.raw_response
        eprintln!("Bybit {}: {}", e.ret_code, e.ret_msg);
    }
    Err(ClientError::AuthError(msg)) => { /* missing/invalid credentials */ }
    Err(ClientError::HttpError(msg)) => { /* network/timeout */ }
    Err(ClientError::WebSocketDisconnected { ws_key }) => { /* WS dropped */ }
    Err(e) if e.is_retryable() => { /* safe to retry */ }
    Err(e) => { /* other error */ }
}
```

## Configuration

```rust
let config = ClientConfig::builder()
    .api_key("key")
    .api_secret("secret")
    .testnet(true)              // Use testnet (default: false/mainnet)
    .recv_window(5000)          // Request validity window in ms
    .timeout(Duration::from_secs(30))
    .enable_time_sync(true)     // Auto-sync clock with server
    .sync_interval(Duration::from_secs(3600))
    .build();
```

## License

MIT
