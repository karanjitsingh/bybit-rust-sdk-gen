// Auto-generated from TypeScript definitions
// Source: types/request/usdc-shared.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum USDCAPICategory {
    #[default]
    PERPETUAL,
    OPTION,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum USDCOrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum USDCTimeInForce {
    #[default]
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum USDCOrderFilter {
    #[default]
    Order,
    StopOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCKlineRequest {
    pub symbol: String,
    pub period: String,
    pub startTime: f64,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCTransactionLogRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub baseCoin: Option<String>,
    pub startTime: Option<String>,
    pub endTime: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
    pub category: Option<USDCAPICategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct USDCPositionsRequest {
    pub category: USDCAPICategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub expDate: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

