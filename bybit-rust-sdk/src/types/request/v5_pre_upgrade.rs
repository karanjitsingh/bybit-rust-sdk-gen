// Auto-generated from TypeScript definitions
// Source: types/request/v5-pre-upgrade.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetMarkPriceKlineParamsV5_Category;
use crate::types::shared_v5::ExecTypeV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeOrderHistoryParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<GetPreUpgradeOrderHistoryParamsV5_OrderFilter>,
    pub orderStatus: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeTradeHistoryParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub baseCoin: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub execType: Option<ExecTypeV5>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeClosedPnlParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeTransactionLogParamsV5 {
    pub category: GetPreUpgradeTransactionLogParamsV5_Category,
    pub baseCoin: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeOptionDeliveryRecordParamsV5 {
    pub category: String,
    pub symbol: Option<String>,
    pub expDate: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetPreUpgradeUSDCSessionParamsV5 {
    pub category: String,
    pub symbol: Option<String>,
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

    /// `GetPreUpgradeOrderHistoryParamsV5:orderFilter`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetPreUpgradeOrderHistoryParamsV5_OrderFilter {
        #[default]
        Order,
        StopOrder,
    }

    /// `GetPreUpgradeTransactionLogParamsV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetPreUpgradeTransactionLogParamsV5_Category {
        #[default]
        linear,
        option,
    }

}
