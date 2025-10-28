// Auto-generated from TypeScript definitions
// Source: types/response/v5-preupgrade.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PreUpgradeTransaction {
    pub symbol: String,
    pub category: String,
    pub side: PreUpgradeTransaction_Side,
    pub transactionTime: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub qty: String,
    pub size: String,
    pub currency: PreUpgradeTransaction_Currency,
    pub tradePrice: String,
    pub funding: String,
    pub fee: String,
    pub cashFlow: String,
    pub change: String,
    pub cashBalance: String,
    pub feeRate: String,
    pub bonusChange: String,
    pub tradeId: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub extraFees: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PreUpgradeOptionsDelivery {
    pub deliveryTime: f64,
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub position: String,
    pub deliveryPrice: String,
    pub strike: String,
    pub fee: String,
    pub deliveryRpl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PreUpgradeUSDCSessionSettlement {
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub size: String,
    pub sessionAvgPrice: String,
    pub markPrice: String,
    pub realisedPnl: String,
    pub createdTime: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `PreUpgradeTransaction:side`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum PreUpgradeTransaction_Side {
        #[default]
        Buy,
        None,
        Sell,
    }

    /// `PreUpgradeTransaction:currency`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum PreUpgradeTransaction_Currency {
        #[default]
        BTC,
        ETH,
        USDC,
        USDT,
    }

}
