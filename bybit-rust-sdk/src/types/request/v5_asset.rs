// Auto-generated from TypeScript definitions
// Source: types/request/v5-asset.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::AccountTypeV5;
use crate::types::shared_v5::CategoryV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetCoinExchangeRecordParamsV5 {
    pub fromCoin: Option<String>,
    pub toCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetDeliveryRecordParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub expDate: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSettlementRecordParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetAssetInfoParamsV5 {
    pub accountType: AccountTypeV5,
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetAllCoinsBalanceParamsV5 {
    pub memberId: Option<String>,
    pub accountType: AccountTypeV5,
    pub coin: Option<String>,
    pub withBonus: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetAccountCoinBalanceParamsV5 {
    pub memberId: Option<String>,
    pub toMemberId: Option<String>,
    pub accountType: AccountTypeV5,
    pub coin: String,
    pub toAccountType: Option<AccountTypeV5>,
    pub withBonus: Option<f64>,
    pub withTransferSafeAmount: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub withLtvTransferSafeAmount: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetInternalTransferParamsV5 {
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
pub struct UniversalTransferParamsV5 {
    pub transferId: String,
    pub coin: String,
    pub amount: String,
    pub fromMemberId: f64,
    pub toMemberId: f64,
    pub fromAccountType: AccountTypeV5,
    pub toAccountType: AccountTypeV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetUniversalTransferRecordsParamsV5 {
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
pub struct GetAllowedDepositCoinInfoParamsV5 {
    pub coin: Option<String>,
    pub chain: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetDepositRecordParamsV5 {
    pub id: Option<String>,
    pub txID: Option<String>,
    pub coin: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSubAccountDepositRecordParamsV5 {
    pub id: Option<String>,
    pub txID: Option<String>,
    pub subMemberId: String,
    pub coin: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetInternalDepositRecordParamsV5 {
    pub txID: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub coin: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetWithdrawalRecordsParamsV5 {
    pub withdrawID: Option<String>,
    pub txID: Option<String>,
    pub coin: Option<String>,
    pub withdrawType: Option<f64>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawParamsV5 {
    pub coin: String,
    pub chain: String,
    pub address: String,
    pub tag: Option<String>,
    pub amount: String,
    pub timestamp: f64,
    pub forceChain: Option<f64>,
    pub accountType: WithdrawParamsV5_AccountType,
    pub feeType: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub requestId: Option<String>,
    pub beneficiary: Option<WithdrawParamsV5_Beneficiary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConvertCoinsParamsV5 {
    pub coin: Option<String>,
    pub side: Option<f64>,
    pub accountType: ConvertCoinsParamsV5_AccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RequestConvertQuoteParamsV5 {
    pub fromCoin: String,
    pub toCoin: String,
    pub fromCoinType: Option<String>,
    pub toCoinType: Option<String>,
    pub requestCoin: String,
    pub requestAmount: String,
    pub accountType: ConvertCoinsParamsV5_AccountType,
    pub requestId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetConvertHistoryParamsV5 {
    pub accountType: Option<String>,
    pub index: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawParamsV5_Beneficiary {
    pub vaspEntityId: Option<String>,
    pub beneficiaryName: Option<String>,
    pub beneficiaryLegalType: Option<String>,
    pub beneficiaryWalletType: Option<String>,
    pub beneficiaryUnhostedWalletType: Option<String>,
    pub beneficiaryPoiNumber: Option<String>,
    pub beneficiaryPoiType: Option<String>,
    pub beneficiaryPoiIssuingCountry: Option<String>,
    pub beneficiaryPoiExpiredDate: Option<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `WithdrawParamsV5:accountType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum WithdrawParamsV5_AccountType {
        #[default]
        FUND,
        SPOT,
    }

    /// `ConvertCoinsParamsV5:accountType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ConvertCoinsParamsV5_AccountType {
        #[default]
        eb_convert_contract,
        eb_convert_funding,
        eb_convert_inverse,
        eb_convert_spot,
        eb_convert_uta,
    }

}
