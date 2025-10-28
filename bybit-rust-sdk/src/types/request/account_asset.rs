// Auto-generated from TypeScript definitions
// Source: types/request/account-asset.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TransferAccountType {
    #[default]
    CONTRACT,
    SPOT,
    INVESTMENT,
    OPTION,
    UNIFIED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TransferType {
    #[default]
    IN,
    OUT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TransferStatus {
    #[default]
    SUCCESS,
    PENDING,
    FAILED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PageDirection {
    #[default]
    Prev,
    Next,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InternalTransferRequest {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_account_type: TransferAccountType,
    pub to_account_type: TransferAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InternalTransferRequestV3 {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromAccountType: String,
    pub toAccountType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QueryInternalTransfersRequestV3 {
    pub transferId: Option<String>,
    pub coin: String,
    pub status: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountTransferRequest {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub sub_user_id: String,
    #[serde(rename = "type")]
    pub r#type: TransferType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubAccountTransferRequestV3 {
    pub transferId: Option<String>,
    pub coin: Option<String>,
    pub status: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TransferQueryRequest {
    pub transfer_id: Option<String>,
    pub coin: Option<String>,
    pub status: Option<TransferStatus>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub direction: Option<PageDirection>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EnableUniversalTransferRequest {
    /// A comma-separated list of subaccount UIDs, for example "123,45,14,26,46"
    pub transferable_sub_ids: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferRequest {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_member_id: String,
    pub to_member_id: String,
    pub from_account_type: TransferAccountType,
    pub to_account_type: TransferAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupportedDepositListRequest {
    pub coin: Option<String>,
    pub chain: Option<String>,
    pub page_index: Option<f64>,
    pub page_size: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositRecordsRequest {
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub coin: Option<String>,
    pub cursor: Option<String>,
    pub direction: Option<PageDirection>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawalRecordsRequest {
    pub withdraw_id: Option<f64>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub coin: Option<String>,
    pub cursor: Option<String>,
    pub direction: Option<PageDirection>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountAssetInformationRequest {
    /// Account type. Default value: ACCOUNT_TYPE_SPOT
    pub account_type: Option<String>,
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawalRequest {
    pub address: String,
    pub amount: String,
    pub coin: String,
    pub chain: String,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferRequestV3 {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromMemberId: String,
    pub toMemberId: String,
    pub fromAccountType: TransferAccountType,
    pub toAccountType: TransferAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferListRequestV3 {
    pub transferId: Option<String>,
    pub coin: String,
    pub status: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TransferCoinListRequestV3 {
    pub fromAccountType: TransferAccountType,
    pub toAccountType: TransferAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SingleAccountCoinBalanceRequestV3 {
    pub memberId: Option<String>,
    pub accountType: TransferAccountType,
    pub coin: String,
    pub withBonus: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalancesRequestV3 {
    pub memberId: Option<String>,
    pub accountType: TransferAccountType,
    pub coin: Option<String>,
    pub withBonus: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoRequestV3 {
    pub accountType: Option<TransferAccountType>,
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupportedDepositListRequestV3 {
    pub coin: Option<String>,
    pub chain: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositRecordQueryRequestV3 {
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub coin: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SubDepositRecordQueryRequestV3 {
    pub subMemberId: f64,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub coin: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawRecordQueryRequestV3 {
    pub withdrawID: Option<f64>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub coin: Option<String>,
    pub withdrawType: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawCreateRequestV3 {
    pub coin: String,
    pub chain: String,
    pub address: String,
    pub tag: Option<String>,
    pub amount: String,
    pub timestamp: f64,
    pub forceChain: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QueryDepositAddressRequestV3 {
    pub coin: Option<String>,
    pub chainType: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QuerySubAccountDepositAddressRequestV3 {
    pub coin: Option<String>,
    pub chainType: Option<String>,
    pub subMemberId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubMemberRequestV3 {
    pub username: String,
    pub memberType: CreateSubMemberRequestV3_MemberType,
    pub switch: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubAPIKeyRequestV3 {
    pub subuid: String,
    pub note: Option<String>,
    pub readOnly: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub ips: Option<Vec<String>>,
    pub permissions: CreateSubAPIKeyRequestV3_Permissions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ModifyAPIKeyRequestV3 {
    pub readOnly: f64,
    pub ips: Option<Vec<String>>,
    pub permissions: ModifyAPIKeyRequestV3_Permissions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSubAPIKeyRequestV3_Permissions {
    pub ContractTrade: Option<Vec<String>>,
    pub Spot: Option<Vec<String>>,
    pub Wallet: Option<Vec<String>>,
    pub Options: Option<Vec<String>>,
    pub Derivatives: Option<Vec<String>>,
    pub Exchange: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ModifyAPIKeyRequestV3_Permissions {
    pub ContractTrade: Option<Vec<String>>,
    pub Spot: Option<Vec<String>>,
    pub Wallet: Option<Vec<String>>,
    pub Options: Option<Vec<String>>,
    pub Derivatives: Option<Vec<String>>,
    pub CopyTrading: Option<Vec<String>>,
    pub BlockTrade: Option<Vec<String>>,
    pub Exchange: Option<Vec<String>>,
    pub NFT: Option<Vec<String>>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `CreateSubMemberRequestV3:memberType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum CreateSubMemberRequestV3_MemberType {
        #[default]
        #[serde(rename = "1")]
        #[strum(serialize = "1")]
        T1,
        #[serde(rename = "6")]
        #[strum(serialize = "6")]
        T6,
    }

}
