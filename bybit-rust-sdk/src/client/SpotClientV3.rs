// Auto-generated from TypeScript definitions
// Source: /Users/karan/github/bybit-rust-sdk-gen/bybit-api/src/spot-client-v3.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::client::BaseRestClient;
use crate::client::ClientResult;


// Generated client: SpotClientV3
pub struct SpotClientV3<'a> {
    base: &'a BaseRestClient,
}

impl<'a> SpotClientV3<'a> {
    /// Create a new instance of SpotClientV3
    pub fn new(base: &'a BaseRestClient) -> Self {
        Self { base }
    }

    pub fn get_client_type(&self) -> ClientResult<String> {
        todo!("Method implementation: getClientType")
    }

    pub async fn fetch_server_time(&self) -> ClientResult<f64> {
        todo!("Method implementation: fetchServerTime")
    }

    /// Market Data Endpoints
    /// Get merged orderbook for symbol
    /// This is the only known pre-V5 endpoint to still be online.
    pub async fn get_merged_order_book(&self, symbol: String, scale: Option<f64>, limit: Option<f64>) -> ClientResult<serde_json::Value> {
        self.base.get("/spot/v3/public/quote/depth/merged", Some(serde_json::to_value(symbol).unwrap_or_default())).await
    }

    /// API Data Endpoints
    pub async fn get_server_time(&self) -> ClientResult<serde_json::Value> {
        self.base.get("/v2/public/time", None).await
    }

}

