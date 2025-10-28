// Auto-generated from TypeScript definitions
// Source: types/shared-v5.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum CategoryV5 {
    #[default]
    spot,
    linear,
    inverse,
    option,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum ContractTypeV5 {
    #[default]
    InversePerpetual,
    LinearPerpetual,
    InverseFutures,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum CopyTradingV5 {
    #[default]
    none,
    both,
    utaOnly,
    normalOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum InstrumentStatusV5 {
    #[default]
    PreLaunch,
    Trading,
    Settling,
    Delivering,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum MarginTradingV5 {
    #[default]
    none,
    both,
    utaOnly,
    normalSpotOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderFilterV5 {
    #[default]
    Order,
    tpslOrder,
    StopOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderSideV5 {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderTypeV5 {
    #[default]
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderTimeInForceV5 {
    #[default]
    GTC,
    IOC,
    FOK,
    PostOnly,
    RPI,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderTriggerByV5 {
    #[default]
    LastPrice,
    IndexPrice,
    MarkPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OCOTriggerTypeV5 {
    #[default]
    OcoTriggerByUnknown,
    OcoTriggerTp,
    OcoTriggerBySl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderSMPTypeV5 {
    #[default]
    None,
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderStatusV5 {
    #[default]
    Created,
    New,
    Rejected,
    PartiallyFilled,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Untriggered,
    Triggered,
    Deactivated,
    Active,
}

/// Defines the types of order creation mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderCreateTypeV5 {
    #[default]
    CreateByUser,
    CreateByAdminClosing,
    CreateByStopOrder,
    CreateByTakeProfit,
    CreateByPartialTakeProfit,
    CreateByStopLoss,
    CreateByPartialStopLoss,
    CreateByTrailingStop,
    CreateByLiq,
    CreateByTakeOver_PassThrough,
    CreateByAdl_PassThrough,
    CreateByBlock_PassThrough,
    CreateByBlockTradeMovePosition_PassThrough,
    CreateByClosing,
    CreateByFGridBot,
    CloseByFGridBot,
    CreateByTWAP,
    CreateByTVSignal,
    CreateByMmRateClose,
    CreateByMartingaleBot,
    CloseByMartingaleBot,
    CreateByIceBerg,
    CreateByArbitrage,
    CreateByDdh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderCancelTypeV5 {
    #[default]
    CancelByUser,
    CancelByReduceOnly,
    CancelByPrepareLiq,
    CancelAllBeforeLiq,
    CancelByPrepareAdl,
    CancelAllBeforeAdl,
    CancelByAdmin,
    CancelByTpSlTsClear,
    CancelByPzSideCh,
    UNKNOWN,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderRejectReasonV5 {
    #[default]
    EC_NoError,
    EC_Others,
    EC_UnknownMessageType,
    EC_MissingClOrdID,
    EC_MissingOrigClOrdID,
    EC_ClOrdIDOrigClOrdIDAreTheSame,
    EC_DuplicatedClOrdID,
    EC_OrigClOrdIDDoesNotExist,
    EC_TooLateToCancel,
    EC_UnknownOrderType,
    EC_UnknownSide,
    EC_UnknownTimeInForce,
    EC_WronglyRouted,
    EC_MarketOrderPriceIsNotZero,
    EC_LimitOrderInvalidPrice,
    EC_NoEnoughQtyToFill,
    EC_NoImmediateQtyToFill,
    EC_PerCancelRequest,
    EC_MarketOrderCannotBePostOnly,
    EC_PostOnlyWillTakeLiquidity,
    EC_CancelReplaceOrder,
    EC_InvalidSymbolStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum StopOrderTypeV5 {
    #[default]
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
    tpslOrder,
    OcoOrder,
    MmRateClose,
    BidirectionalTpslOrder,
}

/// Position index. Used to identify positions in different position modes.
/// - 0 one-way mode position
/// - 1 Buy side of hedge-mode position
/// - 2 Sell side of hedge-mode position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PositionIdx {
    #[default]
    #[serde(rename = "0")]
    #[strum(serialize = "0")]
    T0,
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
    #[serde(rename = "2")]
    #[strum(serialize = "2")]
    T2,
}

/// Position status.
/// - 'Normal'
/// - 'Liq' in the liquidation progress
/// - 'Adl' in the auto-deleverage progress
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PositionStatusV5 {
    #[default]
    Normal,
    Liq,
    Adl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PositionSideV5 {
    #[default]
    Buy,
    Sell,
    None,
    #[serde(rename = "")]
    #[strum(serialize = "")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OptionTypeV5 {
    #[default]
    Call,
    Put,
}

/// Trade mode.
/// - 0 cross-margin,
/// - 1 isolated margin
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TradeModeV5 {
    #[default]
    #[serde(rename = "0")]
    #[strum(serialize = "0")]
    T0,
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TPSLModeV5 {
    #[default]
    Full,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum AccountMarginModeV5 {
    #[default]
    ISOLATED_MARGIN,
    REGULAR_MARGIN,
    PORTFOLIO_MARGIN,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum UnifiedUpdateStatusV5 {
    #[default]
    FAIL,
    PROCESS,
    SUCCESS,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum AccountTypeV5 {
    #[default]
    CONTRACT,
    SPOT,
    INVESTMENT,
    OPTION,
    UNIFIED,
    FUND,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum TransactionTypeV5 {
    #[default]
    TRANSFER_IN,
    TRANSFER_OUT,
    TRADE,
    SETTLEMENT,
    DELIVERY,
    LIQUIDATION,
    ADL,
    AIRDROP,
    BONUS_RECOLLECT,
    FEE_REFUND,
    INTEREST,
    CURRENCY_BUY,
    CURRENCY_SELL,
    BORROWED_AMOUNT_INS_LOAN,
    PRINCIPLE_REPAYMENT_INS_LOAN,
    INTEREST_REPAYMENT_INS_LOAN,
    AUTO_SOLD_COLLATERAL_INS_LOAN,
    AUTO_BUY_LIABILITY_INS_LOAN,
    AUTO_PRINCIPLE_REPAYMENT_INS_LOAN,
    AUTO_INTEREST_REPAYMENT_INS_LOAN,
    TRANSFER_IN_INS_LOAN,
    TRANSFER_OUT_INS_LOAN,
    SPOT_REPAYMENT_SELL,
    SPOT_REPAYMENT_BUY,
    TOKENS_SUBSCRIPTION,
    TOKENS_REDEMPTION,
    AUTO_DEDUCTION,
    FLEXIBLE_STAKING_SUBSCRIPTION,
    FLEXIBLE_STAKING_REDEMPTION,
    FIXED_STAKING_SUBSCRIPTION,
    FLEXIBLE_STAKING_REFUND,
    FIXED_STAKING_REFUND,
    PREMARKET_TRANSFER_OUT,
    PREMARKET_DELIVERY_SELL_NEW_COIN,
    PREMARKET_DELIVERY_BUY_NEW_COIN,
    PREMARKET_DELIVERY_PLEDGE_PAY_SELLER,
    PREMARKET_DELIVERY_PLEDGE_BACK,
    PREMARKET_ROLLBACK_PLEDGE_BACK,
    PREMARKET_ROLLBACK_PLEDGE_PENALTY_TO_BUYER,
    CUSTODY_NETWORK_FEE,
    CUSTODY_SETTLE_FEE,
    CUSTODY_LOCK,
    CUSTODY_UNLOCK,
    CUSTODY_UNLOCK_REFUND,
    LOANS_BORROW_FUNDS,
    LOANS_PLEDGE_ASSET,
    BONUS_TRANSFER_IN,
    BONUS_TRANSFER_OUT,
    PEF_TRANSFER_IN,
    PEF_TRANSFER_OUT,
    PEF_PROFIT_SHARE,
    ONCHAINEARN_SUBSCRIPTION,
    ONCHAINEARN_REDEMPTION,
    ONCHAINEARN_REFUND,
    STRUCTURE_PRODUCT_SUBSCRIPTION,
    STRUCTURE_PRODUCT_REFUND,
    CLASSIC_WEALTH_MANAGEMENT_SUBSCRIPTION,
    PREMIMUM_WEALTH_MANAGEMENT_SUBSCRIPTION,
    PREMIMUM_WEALTH_MANAGEMENT_REFUND,
    LIQUIDITY_MINING_SUBSCRIPTION,
    LIQUIDITY_MINING_REFUND,
    PWM_SUBSCRIPTION,
    PWM_REFUND,
    DEFI_INVESTMENT_SUBSCRIPTION,
    DEFI_INVESTMENT_REFUND,
    DEFI_INVESTMENT_REDEMPTION,
    INSTITUTION_LOAN_IN,
    INSTITUTION_PAYBACK_PRINCIPAL_OUT,
    INSTITUTION_PAYBACK_INTEREST_OUT,
    INSTITUTION_EXCHANGE_SELL,
    INSTITUTION_EXCHANGE_BUY,
    INSTITUTION_LIQ_PRINCIPAL_OUT,
    INSTITUTION_LIQ_INTEREST_OUT,
    INSTITUTION_LOAN_TRANSFER_IN,
    INSTITUTION_LOAN_TRANSFER_OUT,
    INSTITUTION_LOAN_WITHOUT_WITHDRAW,
    INSTITUTION_LOAN_RESERVE_IN,
    INSTITUTION_LOAN_RESERVE_OUT,
    PLATFORM_TOKEN_MNT_LIQRECALLEDMMNT,
    PLATFORM_TOKEN_MNT_LIQRETURNEDMNT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PermissionTypeV5 {
    #[default]
    ContractTrade,
    Spot,
    Wallet,
    Options,
    Derivatives,
    Exchange,
    NFT,
}

/// Leveraged token status:
/// - '1' LT can be purchased and redeemed
/// - '2' LT can be purchased, but not redeemed
/// - '3' LT can be redeemed, but not purchased
/// - '4' LT cannot be purchased nor redeemed
/// - '5' Adjusting position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum LeverageTokenStatusV5 {
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
    #[serde(rename = "5")]
    #[strum(serialize = "5")]
    T5,
}

/// Leveraged token order type: '1': purchase, '2': redeem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum LTOrderTypeV5 {
    #[default]
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
    #[serde(rename = "2")]
    #[strum(serialize = "2")]
    T2,
}

/// Leveraged token order status: '1': completed, '2': in progress, '3': failed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum LTOrderStatusV5 {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum ExecTypeV5 {
    #[default]
    Trade,
    AdlTrade,
    Funding,
    BustTrade,
    Settle,
    BlockTrade,
    MovePosition,
    UNKNOWN,
}

/// Withdraw type. 0(default): on chain. 1: off chain. 2: all.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WithdrawalTypeV5 {
    #[default]
    #[serde(rename = "0")]
    #[strum(serialize = "0")]
    T0,
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
    #[serde(rename = "2")]
    #[strum(serialize = "2")]
    T2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PermissionsV5 {
    pub ContractTrade: Option<Vec<String>>,
    pub Spot: Option<Vec<String>>,
    pub Wallet: Option<Vec<String>>,
    pub Options: Option<Vec<String>>,
    pub Derivatives: Option<Vec<String>>,
    pub CopyTrading: Option<Vec<String>>,
    pub BlockTrade: Option<Vec<String>>,
    pub Exchange: Option<Vec<String>>,
    /// @deprecated , always returns []
    #[deprecated]
    pub NFT: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CategoryCursorListV5<T: Default, TCategory: Default = CategoryV5> {
    pub category: TCategory,
    pub list: T,
    pub nextPageCursor: Option<String>,
}

/// Next page cursor does not exist for spot!
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CursorListV5<T: Default> {
    pub nextPageCursor: String,
    pub list: T,
}

/// A wrapper type for any responses that have a "nextPageCursor" property, and a "rows" property with an array of elements
/// ```{ nextPageCursor: "something", rows: someData[] }```
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CursorRowsV5<T: Default> {
    pub nextPageCursor: String,
    pub rows: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CategoryListV5<T: Default, TCategory: Default> {
    pub category: TCategory,
    pub list: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CategorySymbolListV5<T: Default, TCategory: Default> {
    pub category: TCategory,
    pub symbol: String,
    pub list: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetSystemStatusParamsV5 {
    pub id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SystemStatusItemV5 {
    pub id: String,
    pub title: String,
    pub state: String,
    pub begin: String,
    pub end: String,
    pub href: String,
    pub serviceTypes: Vec<f64>,
    pub product: Vec<f64>,
    pub uidSuffix: Vec<f64>,
    pub maintainType: String,
    pub env: String,
}

