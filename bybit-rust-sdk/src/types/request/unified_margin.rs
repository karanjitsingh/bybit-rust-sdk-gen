// Auto-generated from TypeScript definitions
// Source: types/request/unified-margin.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::usdc_shared::USDCOrderFilter;
use crate::types::request::usdc_shared::USDCTimeInForce;
use crate::types::shared::KlineIntervalV3;
use crate::types::shared::OrderSide;
use crate::types::shared::inline::ContractOrderRequest_PositionIdx;
use crate::types::shared::inline::UMPublicTradesRequest_OptionType;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum UMCategory {
    #[default]
    linear,
    inverse,
    option,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum UMOrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum UMDirection {
    #[default]
    prev,
    next,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMCandlesRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub interval: KlineIntervalV3,
    pub start: f64,
    pub end: f64,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMInstrumentInfoRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMFundingRateHistoryRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMOptionDeliveryPriceRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub direction: Option<UMDirection>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMPublicTradesRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub baseCoin: Option<String>,
    pub optionType: Option<UMPublicTradesRequest_OptionType>,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMOpenInterestRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub interval: UMOpenInterestRequest_Interval,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMOrderRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub side: OrderSide,
    pub positionIdx: Option<ContractOrderRequest_PositionIdx>,
    pub orderType: UMOrderType,
    pub qty: String,
    pub price: Option<String>,
    pub basePrice: Option<String>,
    pub triggerPrice: Option<String>,
    pub triggerBy: Option<String>,
    pub iv: Option<String>,
    pub timeInForce: USDCTimeInForce,
    pub orderLinkId: Option<String>,
    pub takeProfit: Option<f64>,
    pub stopLoss: Option<f64>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub mmp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMModifyOrderRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub iv: Option<String>,
    pub triggerPrice: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub takeProfit: Option<f64>,
    pub stopLoss: Option<f64>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub triggerBy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMCancelOrderRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMActiveOrdersRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMHistoricOrdersRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderStatus: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMBatchOrder {
    pub symbol: String,
    pub side: OrderSide,
    pub positionIdx: Option<String>,
    pub orderType: UMOrderType,
    pub qty: String,
    pub price: Option<String>,
    pub iv: Option<String>,
    pub timeInForce: USDCTimeInForce,
    pub orderLinkId: Option<String>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub mmp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMBatchOrderReplace {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub iv: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMBatchOrderCancel {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMCancelAllOrdersRequest {
    pub category: UMCategory,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub symbol: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMPositionsRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMSetTPSLRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub trailingStop: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub activePrice: Option<String>,
    pub slSize: Option<String>,
    pub tpSize: Option<String>,
    pub positionIdx: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UM7DayTradingHistoryRequest {
    pub category: UMCategory,
    pub symbol: String,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
    pub execType: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMOptionsSettlementHistoryRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub expDate: Option<String>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMPerpSettlementHistoryRequest {
    pub category: UMCategory,
    pub symbol: Option<String>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMTransactionLogRequest {
    pub category: UMCategory,
    pub currency: String,
    pub baseCoin: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMExchangeCoinsRequest {
    pub fromCoin: Option<String>,
    pub toCoin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMBorrowHistoryRequest {
    pub currency: String,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub direction: Option<UMDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `UMOpenInterestRequest:interval`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum UMOpenInterestRequest_Interval {
        #[default]
        #[serde(rename = "15min")]
        #[strum(serialize = "15min")]
        T15min,
        #[serde(rename = "1d")]
        #[strum(serialize = "1d")]
        T1d,
        #[serde(rename = "1h")]
        #[strum(serialize = "1h")]
        T1h,
        #[serde(rename = "30min")]
        #[strum(serialize = "30min")]
        T30min,
        #[serde(rename = "4h")]
        #[strum(serialize = "4h")]
        T4h,
        #[serde(rename = "5min")]
        #[strum(serialize = "5min")]
        T5min,
    }

}
