// Auto-generated from TypeScript definitions
// Source: types/shared.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

// ============================================================================
// Skipped Type Aliases (1)
// ============================================================================
// The following type aliases were not converted. See reasons below:
//
// Type alias 'RESTClient' skipped: Union with non-string-literal types: SpotClientV3 | RestClientV5

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum KlineInterval {
    #[default]
    #[serde(rename = "1m")]
    #[strum(serialize = "1m")]
    T1m,
    #[serde(rename = "3m")]
    #[strum(serialize = "3m")]
    T3m,
    #[serde(rename = "5m")]
    #[strum(serialize = "5m")]
    T5m,
    #[serde(rename = "15m")]
    #[strum(serialize = "15m")]
    T15m,
    #[serde(rename = "30m")]
    #[strum(serialize = "30m")]
    T30m,
    #[serde(rename = "1h")]
    #[strum(serialize = "1h")]
    T1h,
    #[serde(rename = "2h")]
    #[strum(serialize = "2h")]
    T2h,
    #[serde(rename = "4h")]
    #[strum(serialize = "4h")]
    T4h,
    #[serde(rename = "6h")]
    #[strum(serialize = "6h")]
    T6h,
    #[serde(rename = "12h")]
    #[strum(serialize = "12h")]
    T12h,
    #[serde(rename = "1d")]
    #[strum(serialize = "1d")]
    T1d,
    #[serde(rename = "1w")]
    #[strum(serialize = "1w")]
    T1w,
    #[serde(rename = "1M")]
    #[strum(serialize = "1M")]
    T1M,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum KlineIntervalV3 {
    #[default]
    #[serde(rename = "1")]
    #[strum(serialize = "1")]
    T1,
    #[serde(rename = "3")]
    #[strum(serialize = "3")]
    T3,
    #[serde(rename = "5")]
    #[strum(serialize = "5")]
    T5,
    #[serde(rename = "15")]
    #[strum(serialize = "15")]
    T15,
    #[serde(rename = "30")]
    #[strum(serialize = "30")]
    T30,
    #[serde(rename = "60")]
    #[strum(serialize = "60")]
    T60,
    #[serde(rename = "120")]
    #[strum(serialize = "120")]
    T120,
    #[serde(rename = "240")]
    #[strum(serialize = "240")]
    T240,
    #[serde(rename = "360")]
    #[strum(serialize = "360")]
    T360,
    #[serde(rename = "720")]
    #[strum(serialize = "720")]
    T720,
    D,
    W,
    M,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct numberInString(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct APIRateLimit {
    /// Remaining requests to this endpoint before the next reset
    pub remainingRequests: f64,
    /// Max requests for this endpoint per rollowing window (before next reset)
    pub maxRequests: f64,
    /// Timestamp when the rate limit resets if you have exceeded your current maxRequests.
    /// Otherwise, this is approximately your current timestamp.
    pub resetAtTimestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct APIResponseV3<TResult: Default, TExtInfo: Default = ()> {
    pub retCode: f64,
    pub retMsg: String,
    pub result: TResult,
    pub retExtInfo: TExtInfo,
    /// These are per-UID per-endpoint rate limits, automatically parsed from response headers if available.
    /// Note:
    /// - this is primarily for V5 (or newer) APIs.
    /// - these rate limits are per-endpoint per-account, so will not appear for public API calls
    pub rateLimitApi: Option<APIRateLimit>,
}

/// Request Parameter Types
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolParam {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolLimitParam<TLimit: Default = f64> {
    pub symbol: String,
    pub limit: Option<TLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolPeriodLimitParam<TLimit: Default = f64> {
    pub symbol: String,
    pub period: String,
    pub limit: Option<TLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolFromLimitParam {
    pub symbol: String,
    pub from: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolIntervalFromLimitParam {
    pub symbol: String,
    pub interval: String,
    pub from: f64,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CoinParam {
    pub coin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WalletFundRecordsReq {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub currency: Option<String>,
    pub coin: Option<String>,
    pub wallet_fund_type: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WithdrawRecordsReq {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub coin: Option<String>,
    pub status: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AssetExchangeRecordsReq {
    pub limit: Option<f64>,
    pub from: Option<f64>,
    pub direction: Option<String>,
}

/// Response types
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LeverageFilter {
    pub min_leverage: numberInString,
    pub max_leverage: numberInString,
    pub leverage_step: numberInString,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PriceFilter {
    pub min_price: numberInString,
    pub max_price: numberInString,
    pub tick_size: numberInString,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LotSizeFilter {
    pub max_trading_qty: f64,
    pub min_trading_qty: f64,
    pub qty_step: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolInfo {
    pub name: String,
    pub alias: String,
    pub status: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_scale: f64,
    pub taker_fee: numberInString,
    pub maker_fee: numberInString,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `SingleAccountCoinBalanceRequestV3:withBonus`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SingleAccountCoinBalanceRequestV3_WithBonus {
        #[default]
        #[serde(rename = "0")]
        #[strum(serialize = "0")]
        T0,
        #[serde(rename = "1")]
        #[strum(serialize = "1")]
        T1,
    }

    /// `ContractOrderRequest:triggerDirection`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractOrderRequest_TriggerDirection {
        #[default]
        #[serde(rename = "1")]
        #[strum(serialize = "1")]
        T1,
        #[serde(rename = "2")]
        #[strum(serialize = "2")]
        T2,
    }

    /// `ContractOrderRequest:positionIdx`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractOrderRequest_PositionIdx {
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

    /// `ContractOrderRequest:tpslMode`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractOrderRequest_TpslMode {
        #[default]
        Full,
        Partial,
    }

    /// `ContractSetAutoAddMarginRequest:side`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractSetAutoAddMarginRequest_Side {
        #[default]
        Buy,
        Sell,
    }

    /// `UMPublicTradesRequest:optionType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum UMPublicTradesRequest_OptionType {
        #[default]
        Call,
        Put,
    }

    /// `GetExchangeBrokerEarningsParamsV5:bizType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetExchangeBrokerEarningsParamsV5_BizType {
        #[default]
        CONVERT,
        DERIVATIVES,
        OPTIONS,
        SPOT,
    }

    /// `SubmitStakeRedeemParamsV5:orderType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubmitStakeRedeemParamsV5_OrderType {
        #[default]
        Redeem,
        Stake,
    }

    /// `GetMarkPriceKlineParamsV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetMarkPriceKlineParamsV5_Category {
        #[default]
        inverse,
        linear,
    }

    /// `MovePositionParamsV5_List:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum MovePositionParamsV5_List_Category {
        #[default]
        inverse,
        linear,
        option,
        spot,
    }

    /// `GetMovePositionHistoryParamsV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetMovePositionHistoryParamsV5_Category {
        #[default]
        linear,
        option,
        spot,
    }

    /// `GetMovePositionHistoryParamsV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetMovePositionHistoryParamsV5_Status {
        #[default]
        Filled,
        Processing,
        Rejected,
    }

    /// `RFQTransactionV5:side`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum RFQTransactionV5_Side {
        #[default]
        buy,
        sell,
    }

    /// `GetRFQListParamsV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetRFQListParamsV5_Status {
        #[default]
        Active,
        Canceled,
        Expired,
        Failed,
        Filled,
        PendingFill,
    }

    /// `GetRFQTradeListParamsV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetRFQTradeListParamsV5_Status {
        #[default]
        Filled,
        Rejected,
    }

    /// `SubmitSpreadOrderParamsV5:orderType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubmitSpreadOrderParamsV5_OrderType {
        #[default]
        Limit,
        Market,
    }

    /// `SubmitSpreadOrderParamsV5:timeInForce`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SubmitSpreadOrderParamsV5_TimeInForce {
        #[default]
        FOK,
        GTC,
        IOC,
        PostOnly,
    }

    /// `OrderParamsV5:marketUnit`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum OrderParamsV5_MarketUnit {
        #[default]
        baseCoin,
        quoteCoin,
    }

    /// `AssetInfoResponseV3_Spot:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum AssetInfoResponseV3_Spot_Status {
        #[default]
        ACCOUNT_STATUS_NORMAL,
        ACCOUNT_STATUS_UNSPECIFIED,
    }

    /// `MovePositionResultV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum MovePositionResultV5_Status {
        #[default]
        Processing,
        Rejected,
    }

}
