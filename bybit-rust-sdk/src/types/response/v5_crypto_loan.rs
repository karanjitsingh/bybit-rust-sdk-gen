// Auto-generated from TypeScript definitions
// Source: types/response/v5-crypto-loan.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralCoinV5 {
    pub collateralAccuracy: f64,
    pub initialLTV: String,
    pub liquidationLTV: String,
    pub marginCallLTV: String,
    pub maxLimit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VipCollateralCoinsV5 {
    pub list: Vec<CollateralCoinV5>,
    pub vipLevel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowableCoinV5 {
    pub borrowingAccuracy: f64,
    pub currency: String,
    pub flexibleHourlyInterestRate: String,
    pub hourlyInterestRate7D: String,
    pub hourlyInterestRate14D: String,
    pub hourlyInterestRate30D: String,
    pub hourlyInterestRate90D: String,
    pub hourlyInterestRate180D: String,
    pub maxBorrowingAmount: String,
    pub minBorrowingAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VipBorrowableCoinsV5 {
    pub list: Vec<BorrowableCoinV5>,
    pub vipLevel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountBorrowCollateralLimitV5 {
    pub collateralCurrency: String,
    pub loanCurrency: String,
    pub maxCollateralAmount: String,
    pub maxLoanAmount: String,
    pub minCollateralAmount: String,
    pub minLoanAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UnpaidLoanOrderV5 {
    pub collateralAmount: String,
    pub collateralCurrency: String,
    pub currentLTV: String,
    pub expirationTime: String,
    pub hourlyInterestRate: String,
    pub loanCurrency: String,
    pub loanTerm: String,
    pub orderId: String,
    pub residualInterest: String,
    pub residualPenaltyInterest: String,
    pub totalDebt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepaymentHistoryV5 {
    pub collateralCurrency: String,
    pub collateralReturn: String,
    pub loanCurrency: String,
    pub loanTerm: String,
    pub orderId: String,
    pub repayAmount: String,
    pub repayId: String,
    pub repayStatus: f64,
    pub repayTime: String,
    pub repayType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CompletedLoanOrderV5 {
    pub borrowTime: String,
    pub collateralCurrency: String,
    pub expirationTime: String,
    pub hourlyInterestRate: String,
    pub initialCollateralAmount: String,
    pub initialLoanAmount: String,
    pub loanCurrency: String,
    pub loanTerm: String,
    pub orderId: String,
    pub repaidInterest: String,
    pub repaidPenaltyInterest: String,
    pub status: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LoanLTVAdjustmentHistoryV5 {
    pub collateralCurrency: String,
    pub orderId: String,
    pub adjustId: String,
    pub adjustTime: String,
    pub preLTV: String,
    pub afterLTV: String,
    pub direction: f64,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowCoinV5 {
    pub currency: String,
    pub fixedBorrowable: bool,
    pub fixedBorrowingAccuracy: f64,
    pub flexibleBorrowable: bool,
    pub flexibleBorrowingAccuracy: f64,
    pub maxBorrowingAmount: String,
    pub minFixedBorrowingAmount: String,
    pub minFlexibleBorrowingAmount: String,
    pub vipLevel: String,
    pub flexibleAnnualizedInterestRate: String,
    pub annualizedInterestRate7D: String,
    pub annualizedInterestRate14D: String,
    pub annualizedInterestRate30D: String,
    pub annualizedInterestRate60D: String,
    pub annualizedInterestRate90D: String,
    pub annualizedInterestRate180D: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralRatioV5 {
    pub collateralRatio: String,
    pub maxValue: String,
    pub minValue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralRatioConfigV5 {
    pub collateralRatioList: Vec<CollateralRatioV5>,
    pub currencies: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CurrencyLiquidationV5 {
    pub currency: String,
    pub liquidationOrder: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralDataV5 {
    pub collateralRatioConfigList: Vec<CollateralRatioConfigV5>,
    pub currencyLiquidationList: Vec<CurrencyLiquidationV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AdjustCollateralAmountV5 {
    pub adjustId: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralAdjustmentHistoryV5 {
    pub adjustId: f64,
    pub adjustTime: f64,
    pub afterLTV: String,
    pub amount: String,
    pub collateralCurrency: String,
    pub direction: f64,
    pub preLTV: String,
    pub status: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowListV5 {
    pub fixedTotalDebt: String,
    pub fixedTotalDebtUSD: String,
    pub flexibleHourlyInterestRate: String,
    pub flexibleTotalDebt: String,
    pub flexibleTotalDebtUSD: String,
    pub loanCurrency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralListV5 {
    pub amount: String,
    pub amountUSD: String,
    pub currency: String,
    pub ltv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupplyListV5 {
    pub amount: String,
    pub amountUSD: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CryptoLoanPositionV5 {
    pub borrowList: Vec<BorrowListV5>,
    pub collateralList: Vec<CollateralListV5>,
    pub supplyList: Vec<SupplyListV5>,
    pub totalCollateral: String,
    pub totalDebt: String,
    pub totalSupply: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowFlexibleV5 {
    pub orderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayFlexibleV5 {
    pub repayId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OngoingFlexibleLoanV5 {
    pub hourlyInterestRate: String,
    pub loanCurrency: String,
    pub totalDebt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowHistoryFlexibleV5 {
    pub borrowTime: f64,
    pub initialLoanAmount: String,
    pub loanCurrency: String,
    pub orderId: String,
    pub status: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepaymentHistoryFlexibleV5 {
    pub loanCurrency: String,
    pub repayAmount: String,
    pub repayId: String,
    pub repayStatus: f64,
    pub repayTime: f64,
    pub repayType: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupplyOrderQuoteFixedV5 {
    pub orderCurrency: String,
    pub term: f64,
    pub annualRate: String,
    pub qty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowOrderQuoteFixedV5 {
    pub orderCurrency: String,
    pub term: f64,
    pub annualRate: String,
    pub qty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateBorrowOrderFixedV5 {
    pub orderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateSupplyOrderFixedV5 {
    pub orderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowContractInfoFixedV5 {
    pub annualRate: String,
    /// Deprecated
    pub autoRepay: String,
    pub borrowCurrency: String,
    pub borrowTime: String,
    pub interestPaid: String,
    pub loanId: String,
    pub orderId: String,
    /// 1: Auto Repayment; 2: Transfer to flexible loan; 0: No Automatic Repayment
    pub repayType: String,
    pub repaymentTime: String,
    pub residualPenaltyInterest: String,
    pub residualPrincipal: String,
    pub status: f64,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupplyContractInfoFixedV5 {
    pub annualRate: String,
    pub supplyCurrency: String,
    pub supplyTime: String,
    pub supplyAmount: String,
    pub interestPaid: String,
    pub supplyId: String,
    pub orderId: String,
    pub redemptionTime: String,
    pub penaltyInterest: String,
    pub actualRedemptionTime: String,
    pub status: f64,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowOrderInfoFixedV5 {
    pub annualRate: String,
    pub orderId: f64,
    pub orderTime: String,
    pub filledQty: String,
    pub orderQty: String,
    pub orderCurrency: String,
    pub state: f64,
    pub term: f64,
    /// 1: Auto Repayment; 2: Transfer to flexible loan; 0: No Automatic Repayment
    pub repayType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SupplyOrderInfoFixedV5 {
    pub annualRate: String,
    pub orderId: f64,
    pub orderTime: String,
    pub filledQty: String,
    pub orderQty: String,
    pub orderCurrency: String,
    pub state: f64,
    pub term: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayFixedV5 {
    pub repayId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepaymentHistoryFixedV5 {
    pub details: Vec<RepaymentHistoryFixedV5_Details>,
    pub loanCurrency: String,
    pub repayAmount: String,
    pub repayId: String,
    pub repayStatus: f64,
    pub repayTime: f64,
    pub repayType: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepaymentHistoryFixedV5_Details {
    pub loanCurrency: String,
    pub loanId: String,
    pub repayAmount: String,
}

