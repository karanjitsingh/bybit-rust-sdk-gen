// Auto-generated from TypeScript definitions
// Source: types/response/v5-user.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractOrderRequest_PositionIdx;
use crate::types::shared::inline::ContractOrderRequest_TriggerDirection;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::PermissionsV5;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum ApiKeyType {
    #[default]
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
    #[serde(rename = "2")]
    #[strum(serialize = "2")]
    T2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubMemberResultV5 {
    pub uid: String,
    pub username: String,
    pub memberType: f64,
    pub status: f64,
    pub remark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubApiKeyResultV5 {
    pub id: String,
    pub note: String,
    pub apiKey: String,
    pub readOnly: f64,
    pub secret: String,
    pub permissions: PermissionsV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubMemberV5 {
    pub uid: String,
    pub username: String,
    pub memberType: f64,
    pub status: f64,
    pub accountMode: f64,
    pub remark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ApiKeyPermissionsV5 {
    pub ContractTrade: Vec<String>,
    pub Spot: Vec<String>,
    pub Wallet: Vec<String>,
    pub Options: Vec<String>,
    pub Derivatives: Vec<String>,
    pub CopyTrading: Vec<String>,
    pub BlockTrade: Vec<String>,
    pub Exchange: Vec<String>,
    pub NFT: Vec<String>,
    pub Affiliate: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ApiKeyInfoV5 {
    pub id: String,
    pub note: String,
    pub apiKey: String,
    pub readOnly: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub secret: String,
    pub permissions: ApiKeyPermissionsV5,
    pub ips: Vec<String>,
    /// 1: personal, 2: connected to third-party app
    #[serde(rename = "type")]
    pub r#type: ContractOrderRequest_TriggerDirection,
    pub deadlineDay: f64,
    pub expiredAt: String,
    pub createdAt: String,
    #[deprecated]
    pub unified: f64,
    /// 0: regular account, 1: unified trade account
    pub uta: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub userID: f64,
    pub inviterID: f64,
    pub vipLevel: String,
    pub mktMakerLevel: String,
    pub affiliateID: f64,
    pub rsaPublicKey: String,
    pub isMaster: bool,
    pub parentUid: String,
    pub kycLevel: ApiKeyInfoV5_KycLevel,
    pub kycRegion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateApiKeyResultV5 {
    pub id: String,
    pub note: String,
    pub apiKey: String,
    pub readOnly: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub secret: String,
    pub permissions: PermissionsV5,
    pub ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountAllApiKeysResultV5 {
    pub result: Vec<SubAccountAllApiKeysResultV5_Result>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AffiliateUserListItemV5 {
    pub userId: String,
    pub registerTime: String,
    pub source: String,
    pub remarks: String,
    pub isKyc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AffiliateUserInfoV5 {
    pub uid: String,
    pub vipLevel: String,
    pub takerVol30Day: String,
    pub makerVol30Day: String,
    pub tradeVol30Day: String,
    pub depositAmount30Day: String,
    pub takerVol365Day: String,
    pub makerVol365Day: String,
    pub tradeVol365Day: String,
    pub depositAmount365Day: String,
    pub totalWalletBalance: AffiliateUserInfoV5_TotalWalletBalance,
    pub depositUpdateTime: String,
    pub volUpdateTime: String,
    pub KycLevel: ContractOrderRequest_PositionIdx,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountAllApiKeysResultV5_Result {
    pub id: String,
    pub ips: Option<Vec<String>>,
    pub apiKey: String,
    pub note: String,
    pub status: f64,
    pub expiredAt: Option<String>,
    pub createdAt: String,
    #[serde(rename = "type")]
    pub r#type: ContractOrderRequest_TriggerDirection,
    pub permissions: PermissionsV5,
    pub secret: String,
    pub readOnly: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub deadlineDay: Option<f64>,
    pub flag: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `ApiKeyInfoV5:kycLevel`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ApiKeyInfoV5_KycLevel {
        #[default]
        LEVEL_1,
        LEVEL_2,
        LEVEL_DEFAULT,
    }

    /// `AffiliateUserInfoV5:totalWalletBalance`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum AffiliateUserInfoV5_TotalWalletBalance {
        #[default]
        #[serde(rename = "1")]
        #[strum(serialize = "1")]
        T1,
        #[serde(rename = "2")]
        #[strum(serialize = "2")]
        T2,
        #[serde(rename = "3")]
        #[strum(serialize = "3")]
        T3,
        #[serde(rename = "4")]
        #[strum(serialize = "4")]
        T4,
    }

}
