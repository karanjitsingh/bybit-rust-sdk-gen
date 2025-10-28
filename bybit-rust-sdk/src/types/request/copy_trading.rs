// Auto-generated from TypeScript definitions
// Source: types/request/copy-trading.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::usdc_shared::USDCOrderType;
use crate::types::shared::OrderSide;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingOrderRequest {
    pub side: OrderSide,
    pub symbol: String,
    pub orderType: USDCOrderType,
    pub price: String,
    pub qty: String,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingTradingStopRequest {
    pub symbol: String,
    pub parentOrderId: String,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub parentOrderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingOrderListRequest {
    pub symbol: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub copyTradeOrderType: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingCancelOrderRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingCloseOrderRequest {
    pub symbol: String,
    pub orderLinkId: Option<String>,
    pub parentOrderId: Option<String>,
    pub parentOrderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CopyTradingTransferRequest {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromAccountType: String,
    pub toAccountType: String,
}

