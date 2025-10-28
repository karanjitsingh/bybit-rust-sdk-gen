// Auto-generated from TypeScript definitions
// Source: types/request/v5-p2p-trading.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractOrderRequest_TriggerDirection;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2PAccountCoinsBalanceParamsV5 {
    pub memberId: Option<String>,
    pub accountType: String,
    pub coin: Option<String>,
    pub withBonus: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2POnlineAdsParamsV5 {
    pub tokenId: String,
    pub currencyId: String,
    /// 0: buy; 1: sell
    pub side: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub page: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct P2PTradingPreferenceSetV5 {
    pub hasUnPostAd: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub isKyc: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub isEmail: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub isMobile: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub hasRegisterTime: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub registerTimeThreshold: Option<f64>,
    pub orderFinishNumberDay30: Option<f64>,
    pub completeRateDay30: Option<String>,
    pub nationalLimit: Option<String>,
    pub hasOrderFinishNumberDay30: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub hasCompleteRateDay30: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub hasNationalLimit: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateP2PAdParamsV5 {
    pub tokenId: String,
    pub currencyId: String,
    /// 0: buy; 1: sell
    pub side: SingleAccountCoinBalanceRequestV3_WithBonus,
    /// 0: fixed rate; 1: floating rate
    pub priceType: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub premium: String,
    pub price: String,
    pub minAmount: String,
    pub maxAmount: String,
    pub remark: String,
    pub tradingPreferenceSet: P2PTradingPreferenceSetV5,
    pub paymentIds: Vec<String>,
    pub quantity: String,
    pub paymentPeriod: String,
    pub itemType: CreateP2PAdParamsV5_ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateP2PAdParamsV5 {
    pub id: String,
    /// 0: fixed rate; 1: floating rate
    pub priceType: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub premium: String,
    pub price: String,
    pub minAmount: String,
    pub maxAmount: String,
    pub remark: String,
    pub tradingPreferenceSet: P2PTradingPreferenceSetV5,
    pub paymentIds: Vec<String>,
    /// MODIFY: modify adv; ACTIVE: reonline adv
    pub actionType: UpdateP2PAdParamsV5_ActionType,
    pub quantity: String,
    pub paymentPeriod: String,
    pub itemType: Option<CreateP2PAdParamsV5_ItemType>,
    pub subsidyAd: Option<bool>,
    pub securityRiskToken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2PPersonalAdsParamsV5 {
    pub itemId: Option<String>,
    /// 1: Sold Out; 2: Available
    pub status: Option<ContractOrderRequest_TriggerDirection>,
    /// 0: buy; 1: sell
    pub side: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub tokenId: Option<String>,
    pub page: Option<String>,
    pub size: Option<String>,
    pub currencyId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2POrdersParamsV5 {
    pub status: Option<f64>,
    pub beginTime: Option<String>,
    pub endTime: Option<String>,
    pub tokenId: Option<String>,
    pub side: Option<Vec<f64>>,
    pub page: f64,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2PPendingOrdersParamsV5 {
    pub status: Option<f64>,
    pub beginTime: Option<String>,
    pub endTime: Option<String>,
    pub tokenId: Option<String>,
    pub side: Option<Vec<f64>>,
    pub page: f64,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MarkP2POrderAsPaidParamsV5 {
    pub orderId: String,
    pub paymentType: String,
    pub paymentId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SendP2POrderMessageParamsV5 {
    pub message: String,
    pub contentType: String,
    pub orderId: String,
    pub msgUuid: Option<String>,
    pub fileName: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2POrderMessagesParamsV5 {
    pub orderId: String,
    pub currentPage: Option<String>,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetP2PCounterpartyUserInfoParamsV5 {
    pub originalUid: String,
    pub orderId: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `CreateP2PAdParamsV5:itemType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum CreateP2PAdParamsV5_ItemType {
        #[default]
        BULK,
        ORIGIN,
    }

    /// `UpdateP2PAdParamsV5:actionType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum UpdateP2PAdParamsV5_ActionType {
        #[default]
        ACTIVE,
        MODIFY,
    }

}
