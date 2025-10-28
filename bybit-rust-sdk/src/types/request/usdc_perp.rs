// Auto-generated from TypeScript definitions
// Source: types/request/usdc-perp.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::usdc_shared::USDCAPICategory;
use crate::types::request::usdc_shared::USDCOrderFilter;
use crate::types::request::usdc_shared::USDCOrderType;
use crate::types::request::usdc_shared::USDCTimeInForce;
use crate::types::shared::OrderSide;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOpenInterestRequest {
    pub symbol: String,
    pub period: String,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCLast500TradesRequest {
    pub category: USDCAPICategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCSymbolDirectionLimit {
    pub symbol: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCSymbolDirectionLimitCursor {
    pub symbol: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPerpOrderRequest {
    pub symbol: String,
    pub orderType: USDCOrderType,
    pub orderFilter: USDCOrderFilter,
    pub side: OrderSide,
    pub orderPrice: Option<String>,
    pub orderQty: String,
    pub timeInForce: Option<USDCTimeInForce>,
    pub orderLinkId: Option<String>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tptriggerby: Option<String>,
    pub slTriggerBy: Option<String>,
    pub basePrice: Option<String>,
    pub triggerPrice: Option<String>,
    pub triggerBy: Option<String>,
    pub mmp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPerpModifyOrderRequest {
    pub symbol: String,
    pub orderFilter: USDCOrderFilter,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderPrice: Option<String>,
    pub orderQty: Option<String>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tptriggerby: Option<String>,
    pub slTriggerBy: Option<String>,
    pub triggerPrice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPerpCancelOrderRequest {
    pub symbol: String,
    pub orderFilter: USDCOrderFilter,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPerpActiveOrdersRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPerpHistoricOrdersRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderStatus: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

