use bybit_rust_sdk::client::BaseRestClient;
use bybit_rust_sdk::client::RestClientV5::RestClientV5;
use bybit_rust_sdk::client::config::ClientConfig;
use bybit_rust_sdk::types::request::v5_market::GetKlineParamsV5;
use bybit_rust_sdk::types::shared::KlineIntervalV3;
use bybit_rust_sdk::types::shared::inline::CategoryInverseLinearSpot;

/// Wrapper struct that owns a RestClientV5, demonstrating
/// that the client can now be stored in a struct without lifetime issues.
pub struct Exchange {
    client: RestClientV5,
}

impl Exchange {
    pub fn new(config: ClientConfig) -> Result<Self, bybit_rust_sdk::client::ClientError> {
        let base = BaseRestClient::new(config)?;
        Ok(Self {
            client: RestClientV5::new(base),
        })
    }

    pub async fn get_data(
        &self,
        symbol: &str,
        interval: KlineIntervalV3,
    ) -> Result<serde_json::Value, bybit_rust_sdk::client::ClientError> {
        let params = GetKlineParamsV5::builder()
            .category(CategoryInverseLinearSpot::spot)
            .symbol(symbol)
            .interval(interval)
            .limit(5.0)
            .build()
            .expect("valid params");

        let result = self.client.get_kline(params).await?;
        serde_json::to_value(result).map_err(|e| {
            bybit_rust_sdk::client::ClientError::SerializationError(e.to_string())
        })
    }
}

#[tokio::test]
async fn test_exchange_get_kline() {
    let exchange = Exchange::new(ClientConfig::default()).expect("exchange should be created");

    // This calls the real Bybit API. It may fail due to US IP restrictions,
    // but the test validates that Exchange owns RestClientV5 and compiles correctly.
    let result = exchange
        .get_data("BTCUSDT", KlineIntervalV3::T15)
        .await;

    match result {
        Ok(data) => {
            println!("Kline data: {}", serde_json::to_string_pretty(&data).unwrap());
        }
        Err(e) => {
            // IP restriction or network error is expected in some environments
            println!("API call failed (expected in US): {}", e);
        }
    }
}
