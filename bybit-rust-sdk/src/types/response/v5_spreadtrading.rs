// Auto-generated from TypeScript definitions
// Source: types/response/v5-spreadtrading.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;
use crate::types::shared::inline::SubmitSpreadOrderParamsV5_OrderType;
use crate::types::shared::inline::SubmitSpreadOrderParamsV5_TimeInForce;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadInstrumentInfoV5 {
    pub symbol: String,
    pub contractType: SpreadInstrumentInfoV5_ContractType,
    pub status: SpreadInstrumentInfoV5_Status,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub settleCoin: String,
    pub tickSize: String,
    pub minPrice: String,
    pub maxPrice: String,
    pub lotSize: String,
    pub minSize: String,
    pub maxSize: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub legs: Vec<SpreadInstrumentInfoV5_Legs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadOrderbookResponseV5 {
    /// Symbol
    pub s: String,
    /// Bids array [price, size]
    pub b: Vec<(String, String)>,
    /// Asks array [price, size]
    pub a: Vec<(String, String)>,
    /// Update ID
    pub u: f64,
    /// Timestamp
    pub ts: f64,
    /// Sequence
    pub seq: f64,
    /// Cross timestamp
    pub cts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadTickerV5 {
    /// Spread combination symbol name
    pub symbol: String,
    /// Bid 1 price
    pub bidPrice: String,
    /// Bid 1 size
    pub bidSize: String,
    /// Ask 1 price
    pub askPrice: String,
    /// Ask 1 size
    pub askSize: String,
    /// Last trade price
    pub lastPrice: String,
    /// The highest price in the last 24 hours
    pub highPrice24h: String,
    /// The lowest price in the last 24 hours
    pub lowPrice24h: String,
    /// Price 24 hours ago
    pub prevPrice24h: String,
    /// Volume for 24h
    pub volume24h: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadRecentTradeV5 {
    /// Execution ID
    pub execId: String,
    /// Spread combination symbol name
    pub symbol: String,
    /// Trade price
    pub price: String,
    /// Trade size
    pub size: String,
    /// Side of taker
    pub side: ContractSetAutoAddMarginRequest_Side,
    /// Trade time (ms)
    pub time: String,
    pub seq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadOpenOrderV5 {
    pub symbol: String,
    pub baseCoin: String,
    pub orderType: SubmitSpreadOrderParamsV5_OrderType,
    pub orderLinkId: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub timeInForce: SubmitSpreadOrderParamsV5_TimeInForce,
    pub orderId: String,
    pub leavesQty: String,
    pub orderStatus: SpreadOpenOrderV5_OrderStatus,
    pub cumExecQty: String,
    pub price: String,
    pub qty: String,
    pub createdTime: String,
    pub updatedTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadOrderHistoryV5 {
    pub symbol: String,
    pub orderType: SubmitSpreadOrderParamsV5_OrderType,
    pub orderLinkId: String,
    pub orderId: String,
    pub contractType: SpreadInstrumentInfoV5_ContractType,
    pub orderStatus: SpreadOrderHistoryV5_OrderStatus,
    pub price: String,
    pub orderQty: String,
    pub timeInForce: SubmitSpreadOrderParamsV5_TimeInForce,
    pub baseCoin: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub leavesQty: String,
    pub settleCoin: String,
    pub cumExecQty: String,
    pub qty: String,
    pub leg1Symbol: String,
    pub leg1ProdType: SpreadOrderHistoryV5_Leg1ProdType,
    pub leg1OrderId: String,
    pub leg1Side: String,
    pub leg2ProdType: SpreadOrderHistoryV5_Leg1ProdType,
    pub leg2OrderId: String,
    pub leg2Symbol: String,
    pub leg2Side: String,
    pub cxlRejReason: String,
    /// Cumulative trading fee details instead of cumExecFee
    pub cumFeeDetail: Option<indexmap::IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadTradeLegV5 {
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub execPrice: String,
    pub execTime: String,
    pub execValue: String,
    pub execType: String,
    pub category: SpreadTradeLegV5_Category,
    pub execQty: String,
    pub execFee: String,
    /// Trading fee currency
    pub feeCurrency: String,
    pub execId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadTradeV5 {
    pub symbol: String,
    pub orderLinkId: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub orderId: String,
    pub execPrice: String,
    pub execTime: String,
    pub execType: String,
    pub execQty: String,
    pub execId: String,
    pub legs: Vec<SpreadTradeLegV5>,
    pub extraFees: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpreadInstrumentInfoV5_Legs {
    pub symbol: String,
    pub contractType: SpreadInstrumentInfoV5_Legs_ContractType,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `SpreadInstrumentInfoV5:contractType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadInstrumentInfoV5_ContractType {
        #[default]
        CarryTrade,
        FundingRateArb,
        FutureSpread,
        PerpBasis,
    }

    /// `SpreadInstrumentInfoV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadInstrumentInfoV5_Status {
        #[default]
        Settling,
        Trading,
    }

    /// `SpreadInstrumentInfoV5_Legs:contractType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadInstrumentInfoV5_Legs_ContractType {
        #[default]
        LinearFutures,
        LinearPerpetual,
        Spot,
    }

    /// `SpreadOpenOrderV5:orderStatus`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadOpenOrderV5_OrderStatus {
        #[default]
        New,
        PartiallyFilled,
    }

    /// `SpreadOrderHistoryV5:orderStatus`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadOrderHistoryV5_OrderStatus {
        #[default]
        Cancelled,
        Filled,
        Rejected,
    }

    /// `SpreadOrderHistoryV5:leg1ProdType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadOrderHistoryV5_Leg1ProdType {
        #[default]
        Futures,
        Spot,
    }

    /// `SpreadTradeLegV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum SpreadTradeLegV5_Category {
        #[default]
        linear,
        spot,
    }

}
