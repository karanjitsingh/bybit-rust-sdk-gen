// Auto-generated from TypeScript definitions
// Source: types/response/v5-broker.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetExchangeBrokerEarningsParamsV5_BizType;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EarningDetailV5 {
    pub userId: String,
    pub bizType: GetExchangeBrokerEarningsParamsV5_BizType,
    pub symbol: String,
    pub coin: String,
    pub earning: String,
    pub markupEarning: String,
    pub baseFeeEarning: String,
    pub orderId: String,
    pub execTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TotalEarningCategoryV5 {
    pub coin: String,
    pub earning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerEarningResultV5 {
    pub totalEarningCat: ExchangeBrokerEarningResultV5_TotalEarningCat,
    pub details: Vec<EarningDetailV5>,
    pub nextPageCursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerAccountInfoV5 {
    pub subAcctQty: String,
    pub maxSubAcctQty: String,
    pub baseFeeRebateRate: ExchangeBrokerAccountInfoV5_BaseFeeRebateRate,
    pub markupFeeRebateRate: ExchangeBrokerAccountInfoV5_MarkupFeeRebateRate,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerSubAccountDepositRecordV5 {
    pub id: String,
    pub subMemberId: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BrokerVoucherSpecV5 {
    pub id: String,
    pub coin: String,
    pub amountUnit: BrokerVoucherSpecV5_AmountUnit,
    pub productLine: String,
    pub subProductLine: String,
    pub totalAmount: indexmap::IndexMap<String, String>,
    pub usedAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BrokerIssuedVoucherV5 {
    pub accountId: String,
    pub awardId: String,
    pub specCode: String,
    pub amount: String,
    pub isClaimed: bool,
    pub startAt: String,
    pub endAt: String,
    pub effectiveAt: String,
    pub ineffectiveAt: String,
    pub usedAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerEarningResultV5_TotalEarningCat {
    pub spot: Vec<TotalEarningCategoryV5>,
    pub derivatives: Vec<TotalEarningCategoryV5>,
    pub options: Vec<TotalEarningCategoryV5>,
    pub convert: Vec<TotalEarningCategoryV5>,
    pub total: Vec<TotalEarningCategoryV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerAccountInfoV5_BaseFeeRebateRate {
    pub spot: String,
    pub derivatives: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExchangeBrokerAccountInfoV5_MarkupFeeRebateRate {
    pub spot: String,
    pub derivatives: String,
    pub convert: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `BrokerVoucherSpecV5:amountUnit`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum BrokerVoucherSpecV5_AmountUnit {
        #[default]
        AWARD_AMOUNT_UNIT_COIN,
        AWARD_AMOUNT_UNIT_USD,
    }

}
