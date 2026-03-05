// Auto-generated from TypeScript definitions
// Source: types/websockets/ws-events.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::response::v5_rfq::RFQItemV5;
use crate::types::response::v5_rfq::RFQPublicTradeV5;
use crate::types::response::v5_rfq::RFQQuoteItemV5;
use crate::types::response::v5_rfq::RFQTradeV5;
use crate::types::shared::inline::OrderParamsV5_MarketUnit;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::ExecTypeV5;
use crate::types::shared_v5::OCOTriggerTypeV5;
use crate::types::shared_v5::OrderCancelTypeV5;
use crate::types::shared_v5::OrderCreateTypeV5;
use crate::types::shared_v5::OrderRejectReasonV5;
use crate::types::shared_v5::OrderSMPTypeV5;
use crate::types::shared_v5::OrderSideV5;
use crate::types::shared_v5::OrderStatusV5;
use crate::types::shared_v5::OrderTimeInForceV5;
use crate::types::shared_v5::OrderTriggerByV5;
use crate::types::shared_v5::OrderTypeV5;
use crate::types::shared_v5::PositionIdx;
use crate::types::shared_v5::PositionSideV5;
use crate::types::shared_v5::PositionStatusV5;
use crate::types::shared_v5::StopOrderTypeV5;
use crate::types::shared_v5::SystemStatusItemV5;
use crate::types::shared_v5::TPSLModeV5;
use crate::types::shared_v5::TradeModeV5;
use crate::types::websockets::ws_general::WsKey;

// Import inline types from the submodule
use self::inline::*;

pub type WSOrderbookEventV5 = WSPublicTopicEventV5<String, String, WSOrderbookV5>;

pub type WSTradeEventV5 = WSPublicTopicEventV5<String, String, Vec<WSTradeV5>>;

pub type WSTickerEventV5 = WSPublicTopicEventV5<String, String, serde_json::Value>;

pub type WSKlineEventV5 = WSPublicTopicEventV5<String, String, Vec<WSKlineV5>>;

pub type WSLiquidationEventV5 = WSPublicTopicEventV5<String, String, Vec<WSLiquidationV5>>;

pub type WSPositionEventV5 = WSPrivateTopicEventV5<String, Vec<WSPositionV5>>;

pub type WSAccountOrderEventV5 = WSPrivateTopicEventV5<String, Vec<WSAccountOrderV5>>;

pub type WSExecutionEventV5 = WSPrivateTopicEventV5<String, Vec<WSExecutionV5>>;

pub type WSExecutionFastEventV5 = WSPrivateTopicEventV5<String, Vec<WSExecutionFastV5>>;

pub type WSWalletEventV5 = WSPrivateTopicEventV5<String, Vec<WSWalletV5>>;

pub type WSGreeksEventV5 = WSPrivateTopicEventV5<String, Vec<WSGreeksV5>>;

pub type WSSpreadOrderEventV5 = WSPrivateTopicEventV5<String, Vec<WSSpreadOrderV5>>;

pub type WSSpreadExecutionEventV5 = WSPrivateTopicEventV5<String, Vec<WSSpreadExecutionV5>>;

pub type WSInsuranceEventV5 = WSPublicTopicEventV5<String, String, Vec<WSInsuranceV5>>;

pub type WSPriceLimitEventV5 = WSPublicTopicEventV5<String, String, WSPriceLimitV5>;

pub type WSADLAlertEventV5 = WSPublicTopicEventV5<String, String, Vec<WSADLAlertV5>>;

pub type WSSystemStatusEventV5 = WSPublicTopicEventV5<String, String, Vec<SystemStatusItemV5>>;

/// RFQ WebSocket Events
/// RFQ Inquiry Channel
/// Private push for RFQ inquiries sent or received by the user
/// Topics: rfq.open.rfqs, rfq.site.rfqs
pub type WSRFQInquiryEventV5 = WSPrivateTopicEventV5<String, Vec<RFQItemV5>>;

/// RFQ Quote Channel
/// Private push for quotes sent or received by the user
/// Topics: rfq.open.quotes, rfq.site.quotes
pub type WSRFQQuoteEventV5 = WSPrivateTopicEventV5<String, Vec<RFQQuoteItemV5>>;

/// RFQ Trade Channel
/// Private push for block trades executed by the user
/// Topics: rfq.open.trades, rfq.site.trades
pub type WSRFQTradeEventV5 = WSPrivateTopicEventV5<String, Vec<RFQTradeV5>>;

/// RFQ Public Trade Channel
/// Public push for all block trades
/// Topics: rfq.open.public.trades, rfq.site.public.trades
pub type WSRFQPublicTradeEventV5 = WSPublicTopicEventV5<String, String, Vec<RFQPublicTradeV5>>;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MessageEventLike {
    pub target: serde_json::Value,
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSPublicTopicEventV5<TTopic: Default, TType: Default, TData: Default> {
    pub id: Option<String>,
    pub topic: TTopic,
    #[serde(rename = "type")]
    pub r#type: TType,
    /// Cross sequence
    pub cs: Option<f64>,
    /// Event timestamp
    pub ts: f64,
    pub data: TData,
    /// matching engine timestamp (correlated with T from public trade channel)
    pub cts: f64,
    /// Internal reference, can be used to determine if this is spot/linear/inverse/etc
    pub wsKey: WsKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSPrivateTopicEventV5<TTopic: Default, TData: Default> {
    pub id: Option<String>,
    pub topic: TTopic,
    pub creationTime: f64,
    pub data: TData,
    pub wsKey: WsKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSOrderbookV5 {
    /// Symbol
    pub s: String,
    /// [price, qty][]
    pub b: Vec<(String, String)>,
    /// [price, qty][]
    pub a: Vec<(String, String)>,
    /// Update ID
    pub u: f64,
    /// Cross sequence
    pub seq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSTradeV5 {
    pub T: f64,
    pub s: String,
    pub S: OrderSideV5,
    pub v: String,
    pub p: String,
    pub L: Option<String>,
    pub i: String,
    pub BT: bool,
    pub RPI: Option<bool>,
    pub mP: Option<String>,
    pub iP: Option<String>,
    pub mIv: Option<String>,
    pub iv: Option<String>,
}

/// WSTickerV5 is the data structure for the "linear" ticker channel
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSTickerV5 {
    pub symbol: String,
    pub tickDirection: String,
    pub price24hPcnt: String,
    pub lastPrice: String,
    pub prevPrice24h: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub openInterest: String,
    pub openInterestValue: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub nextFundingTime: String,
    pub fundingRate: String,
    pub bid1Price: String,
    pub bid1Size: String,
    pub ask1Price: String,
    pub ask1Size: String,
    pub deliveryTime: Option<String>,
    pub basisRate: Option<String>,
    pub deliveryFeeRate: Option<String>,
    pub predictedDeliveryPrice: Option<String>,
    pub preOpenPrice: Option<String>,
    pub preQty: Option<String>,
    pub curPreListingPhase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSTickerOptionV5 {
    pub symbol: String,
    pub bidPrice: String,
    pub bidSize: String,
    pub bidIv: String,
    pub askPrice: String,
    pub askSize: String,
    pub askIv: String,
    pub lastPrice: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub markPriceIv: String,
    pub underlyingPrice: String,
    pub openInterest: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub totalVolume: String,
    pub totalTurnover: String,
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    pub predictedDeliveryPrice: String,
    pub change24h: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSTickerSpotV5 {
    pub symbol: String,
    pub lastPrice: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice24h: String,
    pub volume24h: String,
    pub turnover24h: String,
    pub price24hPcnt: String,
    pub usdIndexPrice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSKlineV5 {
    pub start: f64,
    pub end: f64,
    pub interval: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub turnover: String,
    pub confirm: bool,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSLiquidationV5 {
    pub T: f64,
    pub s: String,
    pub S: OrderSideV5,
    pub v: String,
    pub p: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSPositionV5 {
    pub category: String,
    pub symbol: String,
    pub side: PositionSideV5,
    pub size: String,
    pub positionIdx: PositionIdx,
    pub tradeMode: TradeModeV5,
    pub positionValue: String,
    pub riskId: f64,
    pub riskLimitValue: String,
    pub entryPrice: String,
    pub markPrice: String,
    pub leverage: String,
    pub positionBalance: String,
    pub autoAddMargin: f64,
    pub positionMM: String,
    pub positionIM: String,
    pub positionIMByMp: String,
    pub positionMMByMp: String,
    pub liqPrice: String,
    pub bustPrice: String,
    pub tpslMode: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub trailingStop: String,
    pub unrealisedPnl: String,
    pub curRealisedPnl: String,
    pub sessionAvgPrice: String,
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    pub cumRealisedPnl: String,
    pub positionStatus: PositionStatusV5,
    pub adlRankIndicator: f64,
    pub isReduceOnly: bool,
    pub mmrSysUpdatedTime: String,
    pub leverageSysUpdatedTime: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub seq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSAccountOrderV5 {
    pub category: CategoryV5,
    pub orderId: String,
    pub orderLinkId: String,
    pub isLeverage: String,
    pub blockTradeId: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: OrderSideV5,
    pub positionIdx: PositionIdx,
    pub orderStatus: OrderStatusV5,
    pub createType: OrderCreateTypeV5,
    pub cancelType: OrderCancelTypeV5,
    pub rejectReason: Option<OrderRejectReasonV5>,
    pub avgPrice: Option<String>,
    pub leavesQty: Option<String>,
    pub leavesValue: Option<String>,
    pub cumExecQty: String,
    pub cumExecValue: String,
    pub cumExecFee: String,
    pub closedPnl: String,
    pub feeCurrency: String,
    pub timeInForce: OrderTimeInForceV5,
    pub orderType: OrderTypeV5,
    pub stopOrderType: StopOrderTypeV5,
    pub ocoTriggerType: Option<OCOTriggerTypeV5>,
    pub orderIv: String,
    pub marketUnit: Option<OrderParamsV5_MarketUnit>,
    pub triggerPrice: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub tpslMode: Option<TPSLModeV5>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
    pub tpTriggerBy: String,
    pub slTriggerBy: String,
    pub triggerDirection: f64,
    pub triggerBy: OrderTriggerByV5,
    pub lastPriceOnCreated: String,
    pub reduceOnly: bool,
    pub closeOnTrigger: bool,
    pub placeType: String,
    pub smpType: OrderSMPTypeV5,
    pub smpGroup: f64,
    pub smpOrderId: String,
    pub createdTime: String,
    pub updatedTime: String,
    /// Cumulative trading fee details instead of cumExecFee and feeCurrency
    pub cumFeeDetail: Option<indexmap::IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSExecutionV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub isLeverage: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub side: OrderSideV5,
    pub orderPrice: String,
    pub orderQty: String,
    pub leavesQty: String,
    pub createType: OrderCreateTypeV5,
    pub orderType: OrderTypeV5,
    pub stopOrderType: StopOrderTypeV5,
    pub execFee: String,
    /// Trading fee currency
    pub feeCurrency: String,
    pub execId: String,
    pub execPrice: String,
    pub execQty: String,
    pub execPnl: String,
    pub execType: ExecTypeV5,
    pub execValue: String,
    pub execTime: String,
    pub isMaker: bool,
    pub feeRate: String,
    pub tradeIv: String,
    pub markIv: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub underlyingPrice: String,
    pub blockTradeId: String,
    pub closedSize: String,
    pub extraFees: String,
    pub seq: f64,
    pub marketUnit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSExecutionFastV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub execId: String,
    pub execPrice: String,
    pub execQty: String,
    pub orderId: String,
    pub isMaker: bool,
    pub orderLinkId: String,
    pub side: OrderSideV5,
    pub execTime: String,
    pub seq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSCoinV5 {
    pub coin: String,
    pub equity: String,
    pub usdValue: String,
    pub walletBalance: String,
    pub free: Option<String>,
    pub locked: String,
    pub spotHedgingQty: String,
    pub borrowAmount: String,
    pub availableToBorrow: String,
    pub availableToWithdraw: String,
    pub accruedInterest: String,
    pub totalOrderIM: String,
    pub totalPositionIM: String,
    pub totalPositionMM: String,
    pub unrealisedPnl: String,
    pub cumRealisedPnl: String,
    pub bonus: String,
    pub collateralSwitch: bool,
    pub marginCollateral: bool,
    pub spotBorrow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSWalletV5 {
    pub accountType: String,
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
    pub coin: Vec<WSCoinV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSGreeksV5 {
    pub baseCoin: String,
    pub totalDelta: String,
    pub totalGamma: String,
    pub totalVega: String,
    pub totalTheta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSSpreadOrderV5 {
    pub category: WSSpreadOrderV5_Category,
    pub symbol: String,
    pub parentOrderId: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub side: OrderSideV5,
    pub orderStatus: OrderStatusV5,
    pub cancelType: OrderCancelTypeV5,
    pub rejectReason: OrderRejectReasonV5,
    pub timeInForce: OrderTimeInForceV5,
    pub price: String,
    pub qty: String,
    pub avgPrice: String,
    pub leavesQty: String,
    pub leavesValue: String,
    pub cumExecQty: String,
    pub cumExecValue: String,
    pub cumExecFee: String,
    pub orderType: OrderTypeV5,
    pub isLeverage: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub feeCurrency: String,
    pub createType: OrderCreateTypeV5,
    pub closedPnl: String,
    /// Cumulative trading fee details instead of cumExecFee and feeCurrency
    pub cumFeeDetail: Option<indexmap::IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSSpreadExecutionV5 {
    pub category: WSSpreadOrderV5_Category,
    pub symbol: String,
    pub isLeverage: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub side: OrderSideV5,
    pub orderPrice: String,
    pub orderQty: String,
    pub leavesQty: String,
    pub createType: OrderCreateTypeV5,
    pub orderType: OrderTypeV5,
    pub execFee: String,
    pub execFeeV2: String,
    /// Trading fee currency
    pub feeCurrency: String,
    pub parentExecId: String,
    pub execId: String,
    pub execPrice: String,
    pub execQty: String,
    pub execPnl: String,
    pub execType: ExecTypeV5,
    pub execValue: String,
    pub execTime: String,
    pub isMaker: bool,
    pub feeRate: String,
    pub markPrice: String,
    pub closedSize: String,
    pub seq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSInsuranceV5 {
    pub coin: String,
    pub symbols: String,
    pub balance: String,
    pub updateTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSPriceLimitV5 {
    pub symbol: String,
    pub buyLmt: String,
    pub sellLmt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSADLAlertV5 {
    /// Token of the insurance pool
    pub c: String,
    /// Trading pair name
    pub s: String,
    /// Balance of the insurance fund. Used to determine if ADL is triggered
    pub b: String,
    /// Maximum balance of the insurance pool in the last 8 hours
    pub mb: String,
    /// PnL ratio threshold for triggering contract PnL drawdown ADL
    pub i_pr: String,
    /// Symbol's PnL drawdown ratio in the last 8 hours. Used to determine whether ADL is triggered or stopped
    pub pr: String,
    /// Trigger threshold for contract PnL drawdown ADL
    pub adl_tt: String,
    /// Stop ratio threshold for contract PnL drawdown ADL
    pub adl_sr: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `WSSpreadOrderV5:category`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum WSSpreadOrderV5_Category {
        #[default]
        combination,
        future_leg,
        spot_leg,
    }

}
