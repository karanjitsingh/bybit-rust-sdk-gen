// Auto-generated from TypeScript definitions
// Source: types/response/account-asset.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::account_asset::inline::CreateSubMemberRequestV3_MemberType;
use crate::types::shared::inline::AssetInfoResponseV3_Spot_Status;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferCreateResponse {
    pub transferId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferListResponseV3 {
    pub list: Vec<UniversalTransferListResponseV3_List>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QueryInternalTransferSResponseV3 {
    pub list: Vec<QueryInternalTransferSResponseV3_List>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountTransferResponseV3 {
    pub list: Vec<SubAccountTransferResponseV3_List>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalanceResponseV3 {
    pub accountType: String,
    pub bizType: f64,
    pub accountId: String,
    pub memberId: String,
    pub balance: AccountCoinBalanceResponseV3_Balance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalancesResponseV3 {
    pub accountType: String,
    pub memberId: String,
    pub balance: Vec<AccountCoinBalancesResponseV3_Balance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoResponseV3 {
    pub spot: AssetInfoResponseV3_Spot,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupportedDepositV3 {
    pub coin: String,
    pub chain: String,
    pub coinShowName: String,
    pub chainType: String,
    pub blockConfirmNumber: f64,
    pub minDepositAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupportedDepositListResponseV3 {
    pub configList: Vec<SupportedDepositV3>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositRecordV3 {
    pub coin: String,
    pub chain: String,
    pub amount: String,
    pub txID: String,
    pub status: f64,
    pub toAddress: String,
    pub tag: String,
    pub depositFee: String,
    pub successAt: String,
    pub confirmations: String,
    pub txIndex: String,
    pub blockHash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositRecordQueryResponseV3 {
    pub rows: Vec<DepositRecordV3>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawRecordsQueryResponseV3 {
    pub rows: Vec<WithdrawRecordsQueryResponseV3_Rows>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinInfoV3 {
    pub name: String,
    pub coin: String,
    pub remainAmount: String,
    pub chains: Vec<CoinInfoV3_Chains>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinInfoQueryResponseV3 {
    pub rows: Vec<CoinInfoV3>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositAddressChainV3 {
    pub chainType: String,
    pub addressDeposit: String,
    pub tagDeposit: String,
    pub chain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositAddressResponseV3 {
    pub coin: String,
    pub chains: Vec<DepositAddressChainV3>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubMemberResponseV3 {
    pub uid: f64,
    pub username: String,
    pub memberType: CreateSubMemberRequestV3_MemberType,
    pub switch: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubAPIKeyResponseV3 {
    pub id: String,
    pub note: String,
    pub apiKey: String,
    pub readOnly: String,
    pub secret: String,
    pub permissions: CreateSubAPIKeyResponseV3_Permissions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubMemberV3 {
    pub uid: String,
    pub username: String,
    pub memberType: CreateSubMemberRequestV3_MemberType,
    pub status: SubMemberV3_Status,
    pub remark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubMemberResponseV3 {
    pub subMembers: Vec<SubMemberV3>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct APIKeyInfoV3 {
    pub id: String,
    pub note: String,
    pub apiKey: String,
    pub readOnly: String,
    pub secret: String,
    pub permissions: APIKeyInfoV3_Permissions,
    pub ips: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: f64,
    pub deadlineDay: f64,
    pub expiredAt: String,
    pub createdAt: String,
    pub unified: f64,
    pub uta: f64,
    pub userID: f64,
    pub inviterID: f64,
    pub vipLevel: String,
    pub mktMakerLevel: String,
    pub affiliateID: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferListResponseV3_List {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub timestamp: String,
    pub status: String,
    pub fromAccountType: String,
    pub toAccountType: String,
    pub fromMemberId: String,
    pub toMemberId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QueryInternalTransferSResponseV3_List {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromAccountType: String,
    pub toAccountType: String,
    pub timestamp: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountTransferResponseV3_List {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub memberId: f64,
    pub subMemberId: f64,
    pub timestamp: String,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: SubAccountTransferResponseV3_List_Type,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalanceResponseV3_Balance {
    pub coin: String,
    pub walletBalance: String,
    pub transferBalance: String,
    pub bonus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalancesResponseV3_Balance {
    pub coin: String,
    pub walletBalance: String,
    pub transferBalance: String,
    pub bonus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoResponseV3_Spot_Assets {
    pub coin: String,
    pub frozen: String,
    pub free: String,
    pub withdraw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoResponseV3_Spot {
    pub status: AssetInfoResponseV3_Spot_Status,
    pub assets: Vec<AssetInfoResponseV3_Spot_Assets>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawRecordsQueryResponseV3_Rows {
    pub coin: String,
    pub chain: String,
    pub amount: String,
    pub txID: String,
    pub status: f64,
    pub toAddress: String,
    pub tag: String,
    pub withdrawFee: String,
    pub createTime: String,
    pub updateTime: String,
    pub withdrawId: String,
    pub withdrawType: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinInfoV3_Chains {
    pub chainType: String,
    pub confirmation: String,
    pub withdrawFee: String,
    pub depositMin: String,
    pub withdrawMin: String,
    pub chain: String,
    pub chainDeposit: String,
    pub chainWithdraw: String,
    pub minAccuracy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubAPIKeyResponseV3_Permissions {
    pub ContractTrade: Vec<String>,
    pub Spot: Vec<String>,
    pub Wallet: Vec<String>,
    pub Options: Vec<String>,
    pub Derivatives: Vec<String>,
    pub CopyTrading: Vec<String>,
    pub BlockTrade: Vec<String>,
    pub Exchange: Vec<String>,
    pub NFT: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct APIKeyInfoV3_Permissions {
    pub ContractTrade: Vec<String>,
    pub Spot: Vec<String>,
    pub Wallet: Vec<String>,
    pub Options: Vec<String>,
    pub Derivatives: Vec<String>,
    pub CopyTrading: Vec<String>,
    pub BlockTrade: Vec<String>,
    pub Exchange: Vec<String>,
    pub NFT: Vec<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `SubAccountTransferResponseV3_List:type`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubAccountTransferResponseV3_List_Type {
        #[default]
        IN,
        OUT,
    }

    /// `SubMemberV3:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubMemberV3_Status {
        #[default]
        #[serde(rename = "1")]
        #[strum(serialize = "1")]
        T1,
        #[serde(rename = "2")]
        #[strum(serialize = "2")]
        T2,
        #[serde(rename = "4")]
        #[strum(serialize = "4")]
        T4,
    }

}
