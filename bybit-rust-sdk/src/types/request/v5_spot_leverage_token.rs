// Auto-generated from TypeScript definitions
// Source: types/request/v5-spot-leverage-token.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared_v5::LTOrderTypeV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PurchaseSpotLeveragedTokenParamsV5 {
    pub ltCoin: String,
    pub amount: String,
    pub serialNo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RedeemSpotLeveragedTokenParamsV5 {
    pub ltCoin: String,
    pub quantity: String,
    pub serialNo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSpotLeveragedTokenOrderHistoryParamsV5 {
    pub ltCoin: Option<String>,
    pub orderId: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub ltOrderType: Option<LTOrderTypeV5>,
    pub serialNo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetVIPMarginDataParamsV5 {
    pub vipLevel: Option<String>,
    pub currency: Option<String>,
}

