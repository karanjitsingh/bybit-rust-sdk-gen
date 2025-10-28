// Auto-generated from TypeScript definitions
// Source: /Users/karan/github/bybit-rust-sdk-gen/bybit-api/src/websocket-api-client.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::client::ClientResult;
use crate::client::WebsocketClient::WebsocketClient;
use crate::types::request::v5_trade::*;
use crate::types::response::v5_trade::{BatchOrdersRetExtInfoV5, OrderResultV5};
use crate::types::websockets::ws_api::{WSAPIResponse};


// Generated client: WebsocketAPIClient
pub struct WebsocketAPIClient<'a> {
    ws_client: &'a WebsocketClient<'a>,
}

impl<'a> WebsocketAPIClient<'a> {
    /// Create a new instance of WebsocketAPIClient
    pub fn new(ws_client: &'a WebsocketClient<'a>) -> Self {
        Self { ws_client }
    }

    pub fn get_ws_client(&self) -> ClientResult<WebsocketClient> {
        todo!("Method implementation: getWSClient")
    }

    pub fn set_time_offset_ms(&self, newOffset: f64) -> ClientResult<()> {
        todo!("Method implementation: setTimeOffsetMs")
    }

    /// Submit a new order
    pub async fn submit_new_order(&self, params: OrderParamsV5) -> ClientResult<WSAPIResponse<OrderResultV5, String, serde_json::Value>> {
        todo!("Method implementation: submitNewOrder")
    }

    /// Amend an order
    pub async fn amend_order(&self, params: AmendOrderParamsV5) -> ClientResult<WSAPIResponse<OrderResultV5, String, serde_json::Value>> {
        todo!("Method implementation: amendOrder")
    }

    /// Cancel an order
    pub async fn cancel_order(&self, params: CancelOrderParamsV5) -> ClientResult<WSAPIResponse<OrderResultV5, String, serde_json::Value>> {
        todo!("Method implementation: cancelOrder")
    }

    /// Batch submit orders
    pub async fn batch_submit_orders(&self, category: String, orders: Vec<BatchOrderParamsV5>) -> ClientResult<WSAPIResponse<serde_json::Value, String, BatchOrdersRetExtInfoV5>> {
        todo!("Method implementation: batchSubmitOrders")
    }

    /// Batch amend orders
    pub async fn batch_amend_order(&self, category: String, orders: Vec<BatchAmendOrderParamsV5>) -> ClientResult<WSAPIResponse<serde_json::Value, String, BatchOrdersRetExtInfoV5>> {
        todo!("Method implementation: batchAmendOrder")
    }

    /// Batch cancel orders
    pub async fn batch_cancel_order(&self, category: String, orders: Vec<BatchCancelOrderParamsV5>) -> ClientResult<WSAPIResponse<serde_json::Value, String, BatchOrdersRetExtInfoV5>> {
        todo!("Method implementation: batchCancelOrder")
    }

}

