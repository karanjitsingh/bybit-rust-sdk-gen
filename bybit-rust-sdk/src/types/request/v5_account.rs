// Auto-generated from TypeScript definitions
// Source: types/request/v5-account.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared_v5::AccountTypeV5;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::TransactionTypeV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetWalletBalanceParamsV5 {
    pub accountType: AccountTypeV5,
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowHistoryParamsV5 {
    pub currency: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetFeeRateParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetTransactionLogParamsV5 {
    pub accountType: Option<AccountTypeV5>,
    pub category: Option<CategoryV5>,
    pub currency: Option<String>,
    pub baseCoin: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<TransactionTypeV5>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MMPModifyParamsV5 {
    pub baseCoin: String,
    pub window: String,
    pub frozenPeriod: String,
    pub qtyLimit: String,
    pub deltaLimit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayLiabilityParamsV5 {
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetCollateralCoinParamsV5 {
    pub coin: String,
    pub collateralSwitch: SetCollateralCoinParamsV5_CollateralSwitch,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetClassicTransactionLogsParamsV5 {
    pub currency: Option<String>,
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
pub struct SetLimitPriceActionParamsV5 {
    pub category: CategoryV5,
    pub modifyEnable: bool,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `SetCollateralCoinParamsV5:collateralSwitch`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SetCollateralCoinParamsV5_CollateralSwitch {
        #[default]
        OFF,
        ON,
    }

}
