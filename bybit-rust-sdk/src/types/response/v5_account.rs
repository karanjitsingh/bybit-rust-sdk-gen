// Auto-generated from TypeScript definitions
// Source: types/response/v5-account.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared_v5::AccountMarginModeV5;
use crate::types::shared_v5::AccountTypeV5;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::TransactionTypeV5;
use crate::types::shared_v5::UnifiedUpdateStatusV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WalletBalanceV5Coin {
    pub coin: String,
    pub equity: String,
    pub usdValue: String,
    pub walletBalance: String,
    /// spot only
    pub free: String,
    /// spot only
    pub locked: String,
    pub borrowAmount: String,
    /// deprecated field
    pub availableToBorrow: String,
    pub availableToWithdraw: String,
    pub accruedInterest: String,
    pub totalOrderIM: String,
    pub totalPositionIM: String,
    pub totalPositionMM: String,
    pub unrealisedPnl: String,
    pub cumRealisedPnl: String,
    pub bonus: String,
    pub marginCollateral: bool,
    pub collateralSwitch: bool,
    pub spotBorrow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WalletBalanceV5 {
    pub accountType: AccountTypeV5,
    pub accountLTV: String,
    pub accountIMRate: String,
    pub accountMMRate: String,
    pub accountIMRateByMp: String,
    pub accountMMRateByMp: String,
    pub totalInitialMarginByMp: String,
    pub totalMaintenanceMarginByMp: String,
    pub totalEquity: String,
    pub totalWalletBalance: String,
    pub totalMarginBalance: String,
    pub totalAvailableBalance: String,
    pub totalPerpUPL: String,
    pub totalInitialMargin: String,
    pub totalMaintenanceMargin: String,
    pub coin: Vec<WalletBalanceV5Coin>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UnifiedAccountUpgradeResultV5 {
    pub unifiedUpdateStatus: UnifiedUpdateStatusV5,
    pub unifiedUpdateMsg: UnifiedAccountUpgradeResultV5_UnifiedUpdateMsg,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BorrowHistoryRecordV5 {
    pub currency: String,
    pub createdTime: f64,
    pub borrowCost: String,
    pub hourlyBorrowRate: String,
    pub InterestBearingBorrowSize: String,
    pub costExemption: String,
    pub borrowAmount: String,
    pub unrealisedLoss: String,
    pub freeBorrowedAmount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CollateralInfoV5 {
    pub currency: String,
    pub hourlyBorrowRate: String,
    pub maxBorrowingAmount: String,
    pub freeBorrowAmount: String,
    pub freeBorrowingLimit: String,
    pub borrowAmount: String,
    pub availableToBorrow: String,
    pub borrowable: bool,
    pub borrowUsageRate: String,
    pub marginCollateral: bool,
    pub collateralSwitch: bool,
    pub collateralRatio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinGreeksV5 {
    pub baseCoin: String,
    pub totalDelta: String,
    pub totalGamma: String,
    pub totalVega: String,
    pub totalTheta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FeeRateV5 {
    pub symbol: String,
    pub baseCoin: String,
    pub takerFeeRate: String,
    pub makerFeeRate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountInfoV5 {
    pub unifiedMarginStatus: f64,
    pub marginMode: AccountMarginModeV5,
    pub isMasterTrader: bool,
    pub spotHedgingStatus: String,
    pub updatedTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct TransactionLogV5 {
    pub symbol: String,
    pub category: CategoryV5,
    pub side: String,
    pub transactionTime: String,
    #[serde(rename = "type")]
    pub r#type: TransactionTypeV5,
    pub qty: String,
    pub size: String,
    pub currency: String,
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
    pub transSubType: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MMPStateV5 {
    pub baseCoin: String,
    pub mmpEnabled: bool,
    pub window: String,
    pub frozenPeriod: String,
    pub qtyLimit: String,
    pub deltaLimit: String,
    pub mmpFrozenUntil: String,
    pub mmpFrozen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RepayLiabilityResultV5 {
    pub coin: String,
    pub repaymentQty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct DCPInfoV5 {
    pub product: DCPInfoV5_Product,
    pub dcpStatus: String,
    pub timeWindow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UnifiedAccountUpgradeResultV5_UnifiedUpdateMsg {
    pub msg: Option<Vec<String>>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `DCPInfoV5:product`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum DCPInfoV5_Product {
        #[default]
        DERIVATIVES,
        OPTIONS,
        SPOT,
    }

}
