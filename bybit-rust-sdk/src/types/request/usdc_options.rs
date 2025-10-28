// Auto-generated from TypeScript definitions
// Source: types/request/usdc-options.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::usdc_shared::USDCAPICategory;
use crate::types::request::usdc_shared::USDCOrderType;
use crate::types::request::usdc_shared::USDCTimeInForce;
use crate::types::shared::OrderSide;
use crate::types::shared::inline::UMPublicTradesRequest_OptionType;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsContractInfoRequest {
    pub symbol: Option<String>,
    pub status: Option<USDCOptionsContractInfoRequest_Status>,
    pub baseCoin: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsDeliveryPriceRequest {
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsRecentTradesRequest {
    pub category: USDCAPICategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub optionType: Option<UMPublicTradesRequest_OptionType>,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsHistoricalVolatilityRequest {
    pub baseCoin: Option<String>,
    pub period: Option<String>,
    pub startTime: Option<String>,
    pub endTime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsOrderRequest {
    pub symbol: String,
    pub orderType: USDCOrderType,
    pub side: OrderSide,
    pub orderPrice: Option<String>,
    pub orderQty: String,
    pub iv: Option<String>,
    pub timeInForce: Option<USDCTimeInForce>,
    pub orderLinkId: Option<String>,
    pub reduceOnly: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsModifyOrderRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderPrice: Option<String>,
    pub orderQty: Option<String>,
    pub iv: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsCancelOrderRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsCancelAllOrdersRequest {
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsActiveOrdersRealtimeRequest {
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsActiveOrdersRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsHistoricOrdersRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderStatus: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsOrderExecutionRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub startTime: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsDeliveryHistoryRequest {
    pub symbol: String,
    pub expDate: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsPositionsInfoExpiryRequest {
    pub expDate: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCOptionsModifyMMPRequest {
    pub currency: String,
    pub windowMs: f64,
    pub frozenPeriodMs: f64,
    pub qtyLimit: String,
    pub deltaLimit: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `USDCOptionsContractInfoRequest:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum USDCOptionsContractInfoRequest_Status {
        #[default]
        DELIVERING,
        OFFLINE,
        ONLINE,
        WAITING_ONLINE,
    }

}
