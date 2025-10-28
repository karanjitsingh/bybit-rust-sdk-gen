// Auto-generated from TypeScript definitions
// Source: types/response/v5-asset.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::AssetInfoResponseV3_Spot_Status;
use crate::types::shared_v5::AccountTypeV5;
use crate::types::shared_v5::OrderSideV5;
use crate::types::shared_v5::WithdrawalTypeV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinExchangeRecordV5 {
    pub fromCoin: String,
    pub fromAmount: String,
    pub toCoin: String,
    pub toAmount: String,
    pub exchangeRate: String,
    pub createdTime: String,
    pub exchangeTxId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DeliveryRecordV5 {
    pub deliveryTime: f64,
    pub symbol: String,
    pub side: OrderSideV5,
    pub position: String,
    pub deliveryPrice: String,
    pub strike: String,
    pub fee: String,
    pub deliveryRpl: String,
    pub entryPrice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SettlementRecordV5 {
    pub symbol: String,
    pub side: String,
    pub size: f64,
    pub sessionAvgPrice: String,
    pub markPrice: String,
    pub realisedPnl: String,
    pub createdTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoAssetV5 {
    pub coin: String,
    pub frozen: String,
    pub free: String,
    pub withdraw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetInfoV5 {
    pub status: AssetInfoResponseV3_Spot_Status,
    pub assets: Vec<AssetInfoAssetV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinBalanceV5 {
    pub coin: String,
    pub walletBalance: String,
    pub transferBalance: String,
    pub bonus: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AllCoinsBalanceV5 {
    pub accountType: AccountTypeV5,
    pub memberId: Option<String>,
    pub balance: Vec<CoinBalanceV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalanceV5 {
    pub accountType: AccountTypeV5,
    pub bizType: f64,
    pub accountId: String,
    pub memberId: String,
    pub balance: AccountCoinBalanceV5_Balance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InternalTransferRecordV5 {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromAccountType: AccountTypeV5,
    pub toAccountType: AccountTypeV5,
    pub timestamp: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalTransferRecordV5 {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromMemberId: String,
    pub toMemberId: String,
    pub fromAccountType: AccountTypeV5,
    pub toAccountType: AccountTypeV5,
    pub timestamp: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AllowedDepositCoinInfoV5 {
    pub coin: String,
    pub chain: String,
    pub coinShowName: String,
    pub chainType: String,
    pub blockConfirmNumber: f64,
    pub minDepositAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositRecordV5 {
    pub id: String,
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
    pub batchReleaseLimit: String,
    pub depositType: String,
    pub fromAddress: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InternalDepositRecordV5 {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: f64,
    pub coin: String,
    pub amount: String,
    pub status: InternalDepositRecordV5_Status,
    pub address: String,
    pub createdTime: String,
    pub txID: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositAddressChainV5 {
    pub chainType: String,
    pub addressDeposit: String,
    pub tagDeposit: String,
    pub chain: String,
    pub batchReleaseLimit: String,
    pub contractAddress: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DepositAddressResultV5 {
    pub coin: String,
    pub chains: Vec<DepositAddressChainV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinInfoV5 {
    pub name: String,
    pub coin: String,
    pub remainAmount: String,
    pub chains: Vec<CoinInfoV5_Chains>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawalRecordV5 {
    pub withdrawId: String,
    pub txID: String,
    pub withdrawType: WithdrawalTypeV5,
    pub coin: String,
    pub chain: String,
    pub amount: String,
    pub withdrawFee: String,
    pub status: String,
    pub toAddress: String,
    pub tag: String,
    pub createTime: String,
    pub updateTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawableAmountV5 {
    pub limitAmountUsd: String,
    pub withdrawableAmount: WithdrawableAmountV5_WithdrawableAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VaspEntityV5 {
    pub vaspEntityId: String,
    pub vaspName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertCoinSpecV5 {
    pub coin: String,
    pub fullName: String,
    pub icon: String,
    pub iconNight: String,
    pub accuracyLength: f64,
    pub coinType: String,
    pub balance: String,
    pub uBalance: String,
    pub singleFromMinLimit: String,
    pub singleFromMaxLimit: String,
    pub disableFrom: bool,
    pub disableTo: bool,
    pub timePeriod: f64,
    pub singleToMinLimit: String,
    pub singleToMaxLimit: String,
    pub dailyFromMinLimit: String,
    pub dailyFromMaxLimit: String,
    pub dailyToMinLimit: String,
    pub dailyToMaxLimit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertQuoteV5 {
    pub quoteTxId: String,
    pub exchangeRate: String,
    pub fromCoin: String,
    pub fromCoinType: String,
    pub toCoin: String,
    pub toCoinType: String,
    pub fromAmount: String,
    pub toAmount: String,
    pub expiredTime: String,
    pub requestId: String,
    pub extTaxAndFee: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertStatusV5 {
    pub accountType: String,
    pub exchangeTxId: String,
    pub userId: String,
    pub fromCoin: String,
    pub fromCoinType: String,
    pub toCoin: String,
    pub toCoinType: String,
    pub fromAmount: String,
    pub toAmount: String,
    pub exchangeStatus: ConvertStatusV5_ExchangeStatus,
    pub extInfo: ConvertStatusV5_ExtInfo,
    pub convertRate: String,
    pub createdAt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertHistoryRecordV5 {
    pub accountType: String,
    pub exchangeTxId: String,
    pub userId: String,
    pub fromCoin: String,
    pub fromCoinType: String,
    pub toCoin: String,
    pub toCoinType: String,
    pub fromAmount: String,
    pub toAmount: String,
    pub exchangeStatus: ConvertStatusV5_ExchangeStatus,
    pub extInfo: ConvertHistoryRecordV5_ExtInfo,
    pub convertRate: String,
    pub createdAt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountCoinBalanceV5_Balance {
    pub coin: String,
    pub walletBalance: String,
    pub transferBalance: String,
    pub bonus: String,
    pub transferSafeAmount: String,
    pub ltvTransferSafeAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinInfoV5_Chains {
    pub chain: String,
    pub chainType: String,
    pub confirmation: String,
    pub withdrawFee: String,
    pub depositMin: String,
    pub withdrawMin: String,
    pub minAccuracy: String,
    pub chainDeposit: String,
    pub chainWithdraw: String,
    pub withdrawPercentageFee: String,
    pub contractAddress: String,
    pub safeConfirmNumber: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawableAmountV5_WithdrawableAmount_SPOT {
    pub coin: String,
    pub withdrawableAmount: String,
    pub availableBalance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawableAmountV5_WithdrawableAmount_FUND {
    pub coin: String,
    pub withdrawableAmount: String,
    pub availableBalance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawableAmountV5_WithdrawableAmount {
    pub SPOT: WithdrawableAmountV5_WithdrawableAmount_SPOT,
    pub FUND: WithdrawableAmountV5_WithdrawableAmount_FUND,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertStatusV5_ExtInfo {
    pub paramType: String,
    pub paramValue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertHistoryRecordV5_ExtInfo {
    pub paramType: String,
    pub paramValue: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `InternalDepositRecordV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum InternalDepositRecordV5_Status {
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
    }

    /// `ConvertStatusV5:exchangeStatus`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ConvertStatusV5_ExchangeStatus {
        #[default]
        failure,
        init,
        processing,
        success,
    }

}
