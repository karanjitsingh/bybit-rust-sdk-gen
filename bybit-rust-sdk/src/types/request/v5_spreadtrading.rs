// Auto-generated from TypeScript definitions
// Source: types/request/v5-spreadtrading.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;
use crate::types::shared::inline::SubmitSpreadOrderParamsV5_OrderType;
use crate::types::shared::inline::SubmitSpreadOrderParamsV5_TimeInForce;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSpreadInstrumentsInfoParamsV5 {
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitSpreadOrderParamsV5 {
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub orderType: SubmitSpreadOrderParamsV5_OrderType,
    pub qty: String,
    pub price: String,
    pub orderLinkId: String,
    pub timeInForce: SubmitSpreadOrderParamsV5_TimeInForce,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AmendSpreadOrderParamsV5 {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSpreadOpenOrdersParamsV5 {
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSpreadOrderHistoryParamsV5 {
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSpreadTradeHistoryParamsV5 {
    pub symbol: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

