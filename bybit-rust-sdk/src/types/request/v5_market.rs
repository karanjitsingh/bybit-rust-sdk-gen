// Auto-generated from TypeScript definitions
// Source: types/request/v5-market.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::KlineIntervalV3;
use crate::types::shared::inline::GetMarkPriceKlineParamsV5_Category;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::InstrumentStatusV5;
use crate::types::shared_v5::OptionTypeV5;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OpenInterestIntervalV5 {
    #[default]
    #[serde(rename = "5min")]
    #[strum(serialize = "5min")]
    T5min,
    #[serde(rename = "15min")]
    #[strum(serialize = "15min")]
    T15min,
    #[serde(rename = "30min")]
    #[strum(serialize = "30min")]
    T30min,
    #[serde(rename = "1h")]
    #[strum(serialize = "1h")]
    T1h,
    #[serde(rename = "4h")]
    #[strum(serialize = "4h")]
    T4h,
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    T1d,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetKlineParamsV5 {
    pub category: GetKlineParamsV5_Category,
    pub symbol: String,
    pub interval: KlineIntervalV3,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetMarkPriceKlineParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub interval: KlineIntervalV3,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetIndexPriceKlineParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub interval: KlineIntervalV3,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPremiumIndexPriceKlineParamsV5 {
    pub category: String,
    pub symbol: String,
    pub interval: KlineIntervalV3,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetInstrumentsInfoParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub status: Option<InstrumentStatusV5>,
    pub baseCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetOrderbookParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRPIOrderbookParamsV5 {
    pub category: Option<GetKlineParamsV5_Category>,
    pub symbol: String,
    /// Required for RPI orderbook, [1, 50]
    pub limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetIndexPriceComponentsParamsV5 {
    /// Index name, like BTCUSDT
    pub indexName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetADLAlertParamsV5 {
    /// Contract name, e.g. BTCUSDT. Uppercase only
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetFeeGroupStructureParamsV5 {
    /// Product type. contract only for now
    pub productType: String,
    /// Group ID. 1, 2, 3, 4, 5, 6, 7
    pub groupId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetTickersParamsV5<TCategory: Default = CategoryV5> {
    pub category: TCategory,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub expDate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetFundingRateHistoryParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPublicTradingHistoryParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub baseCoin: Option<String>,
    pub optionType: Option<OptionTypeV5>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetOpenInterestParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub intervalTime: OpenInterestIntervalV5,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetHistoricalVolatilityParamsV5 {
    pub category: String,
    pub baseCoin: Option<String>,
    pub period: Option<GetHistoricalVolatilityParamsV5_Period>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetInsuranceParamsV5 {
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRiskLimitParamsV5 {
    pub category: Option<GetMarkPriceKlineParamsV5_Category>,
    pub symbol: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetOptionDeliveryPriceParamsV5 {
    pub category: String,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetDeliveryPriceParamsV5 {
    pub category: GetDeliveryPriceParamsV5_Category,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetLongShortRatioParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub period: OpenInterestIntervalV5,
    pub startTime: Option<String>,
    pub endTime: Option<String>,
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

    /// `GetKlineParamsV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetKlineParamsV5_Category {
        #[default]
        inverse,
        linear,
        spot,
    }

    /// `GetHistoricalVolatilityParamsV5:period`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetHistoricalVolatilityParamsV5_Period {
        #[default]
        #[serde(rename = "14")]
        #[strum(serialize = "14")]
        T14,
        #[serde(rename = "180")]
        #[strum(serialize = "180")]
        T180,
        #[serde(rename = "21")]
        #[strum(serialize = "21")]
        T21,
        #[serde(rename = "270")]
        #[strum(serialize = "270")]
        T270,
        #[serde(rename = "30")]
        #[strum(serialize = "30")]
        T30,
        #[serde(rename = "60")]
        #[strum(serialize = "60")]
        T60,
        #[serde(rename = "7")]
        #[strum(serialize = "7")]
        T7,
        #[serde(rename = "90")]
        #[strum(serialize = "90")]
        T90,
    }

    /// `GetDeliveryPriceParamsV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetDeliveryPriceParamsV5_Category {
        #[default]
        inverse,
        linear,
        option,
    }

}
