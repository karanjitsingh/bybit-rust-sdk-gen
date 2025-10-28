// Auto-generated from TypeScript definitions
// Source: types/request/v5-earn.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SubmitStakeRedeemParamsV5_OrderType;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitStakeRedeemParamsV5 {
    pub category: String,
    pub orderType: SubmitStakeRedeemParamsV5_OrderType,
    pub accountType: SubmitStakeRedeemParamsV5_AccountType,
    pub amount: String,
    pub coin: String,
    pub productId: String,
    pub orderLinkId: String,
    pub toAccountType: Option<SubmitStakeRedeemParamsV5_AccountType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetEarnOrderHistoryParamsV5 {
    pub category: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetEarnPositionParamsV5 {
    pub category: String,
    pub productId: Option<String>,
    pub coin: Option<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `SubmitStakeRedeemParamsV5:accountType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubmitStakeRedeemParamsV5_AccountType {
        #[default]
        FUND,
        UNIFIED,
    }

}
