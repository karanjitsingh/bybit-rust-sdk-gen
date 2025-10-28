// Auto-generated from TypeScript definitions
// Source: types/request/v5-crypto-loan.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowCryptoLoanParamsV5 {
    pub loanCurrency: String,
    pub loanAmount: Option<String>,
    pub loanTerm: Option<String>,
    pub collateralCurrency: String,
    pub collateralAmount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetUnpaidLoanOrdersParamsV5 {
    pub orderId: Option<String>,
    pub loanCurrency: Option<String>,
    pub collateralCurrency: Option<String>,
    pub loanTermType: Option<String>,
    pub loanTerm: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRepaymentHistoryParamsV5 {
    pub orderId: Option<String>,
    pub repayId: Option<String>,
    pub loanCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetCompletedLoanOrderHistoryParamsV5 {
    pub orderId: Option<String>,
    pub loanCurrency: Option<String>,
    pub collateralCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetLoanLTVAdjustmentHistoryParamsV5 {
    pub orderId: Option<String>,
    pub adjustId: Option<String>,
    pub collateralCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowableCoinsParamsV5 {
    pub vipLevel: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetCollateralCoinsParamsV5 {
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetMaxCollateralAmountParamsV5 {
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AdjustCollateralAmountParamsV5 {
    pub currency: String,
    pub amount: String,
    pub direction: SingleAccountCoinBalanceRequestV3_WithBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetCollateralAdjustmentHistoryParamsV5 {
    pub adjustId: Option<String>,
    pub collateralCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowFlexibleParamsV5 {
    pub loanCurrency: String,
    pub loanAmount: String,
    pub collateralList: Option<Vec<BorrowFlexibleParamsV5_CollateralList>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayFlexibleParamsV5 {
    pub loanCurrency: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayCollateralFlexibleParamsV5 {
    pub loanCurrency: String,
    pub collateralCoin: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetOngoingFlexibleLoansParamsV5 {
    pub loanCurrency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowHistoryFlexibleParamsV5 {
    pub orderId: Option<String>,
    pub loanCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRepaymentHistoryFlexibleParamsV5 {
    pub repayId: Option<String>,
    pub loanCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSupplyOrderQuoteFixedParamsV5 {
    pub orderCurrency: String,
    pub term: Option<String>,
    pub orderBy: GetSupplyOrderQuoteFixedParamsV5_OrderBy,
    pub sort: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowOrderQuoteFixedParamsV5 {
    pub orderCurrency: String,
    pub term: Option<String>,
    pub orderBy: GetSupplyOrderQuoteFixedParamsV5_OrderBy,
    pub sort: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateBorrowOrderFixedParamsV5 {
    pub orderCurrency: String,
    pub orderAmount: String,
    pub annualRate: String,
    pub term: String,
    /// Deprecated
    pub autoRepay: Option<String>,
    /// 1: Auto Repayment (default); 2: Transfer to flexible loan
    pub repayType: Option<String>,
    pub collateralList: Option<Vec<CreateBorrowOrderFixedParamsV5_CollateralList>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSupplyOrderFixedParamsV5 {
    pub orderCurrency: String,
    pub orderAmount: String,
    pub annualRate: String,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelBorrowOrderFixedParamsV5 {
    pub orderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelSupplyOrderFixedParamsV5 {
    pub orderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowContractInfoFixedParamsV5 {
    pub orderId: Option<String>,
    pub loanId: Option<String>,
    pub orderCurrency: Option<String>,
    pub term: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSupplyContractInfoFixedParamsV5 {
    pub orderId: Option<String>,
    pub supplyId: Option<String>,
    pub supplyCurrency: Option<String>,
    pub term: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBorrowOrderInfoFixedParamsV5 {
    pub orderId: Option<String>,
    pub orderCurrency: Option<String>,
    pub state: Option<String>,
    pub term: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSupplyOrderInfoFixedParamsV5 {
    pub orderId: Option<String>,
    pub orderCurrency: Option<String>,
    pub state: Option<String>,
    pub term: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayFixedParamsV5 {
    pub loanId: Option<String>,
    pub loanCurrency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayCollateralFixedParamsV5 {
    pub loanCurrency: String,
    pub collateralCoin: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRepaymentHistoryFixedParamsV5 {
    pub repayId: Option<String>,
    pub loanCurrency: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowFlexibleParamsV5_CollateralList {
    pub currency: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateBorrowOrderFixedParamsV5_CollateralList {
    pub currency: String,
    pub amount: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `GetSupplyOrderQuoteFixedParamsV5:orderBy`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetSupplyOrderQuoteFixedParamsV5_OrderBy {
        #[default]
        apy,
        quantity,
        term,
    }

}
