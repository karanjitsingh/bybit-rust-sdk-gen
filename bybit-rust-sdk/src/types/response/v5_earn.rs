// Auto-generated from TypeScript definitions
// Source: types/response/v5-earn.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SubmitStakeRedeemParamsV5_OrderType;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EarnProductV5 {
    pub category: String,
    pub estimateApr: String,
    pub coin: String,
    pub minStakeAmount: String,
    pub maxStakeAmount: String,
    pub precision: String,
    pub productId: String,
    pub status: EarnProductV5_Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EarnOrderHistoryV5 {
    pub coin: String,
    pub orderValue: String,
    pub orderType: SubmitStakeRedeemParamsV5_OrderType,
    pub orderId: String,
    pub orderLinkId: String,
    pub status: EarnOrderHistoryV5_Status,
    pub createdAt: String,
    pub productId: String,
    pub updatedAt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EarnPositionV5 {
    pub coin: String,
    pub productId: String,
    pub amount: String,
    pub totalPnl: String,
    pub claimableYield: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `EarnProductV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum EarnProductV5_Status {
        #[default]
        Available,
        NotAvailable,
    }

    /// `EarnOrderHistoryV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum EarnOrderHistoryV5_Status {
        #[default]
        Fail,
        Pending,
        Success,
    }

}
