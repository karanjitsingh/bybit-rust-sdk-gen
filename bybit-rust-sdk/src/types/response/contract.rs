// Auto-generated from TypeScript definitions
// Source: types/response/contract.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PaginatedResult<TList: Default> {
    pub nextPageCursor: String,
    pub list: Vec<TList>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractListResult<TList: Default> {
    pub category: String,
    pub list: Vec<TList>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractHistoricOrder {
    pub symbol: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub side: String,
    pub orderType: String,
    pub price: String,
    pub iv: String,
    pub qty: String,
    pub timeInForce: String,
    pub orderStatus: String,
    pub positionIdx: f64,
    pub lastPriceOnCreated: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub cancelType: String,
    pub rejectReason: String,
    pub stopOrderType: String,
    pub triggerDirection: f64,
    pub triggerBy: String,
    pub triggerPrice: String,
    pub cumExecValue: String,
    pub cumExecFee: String,
    pub cumExecQty: String,
    pub leavesValue: String,
    pub leavesQty: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub tpslMode: String,
    pub tpLimitPrice: String,
    pub slLimitPrice: String,
    pub tpTriggerBy: String,
    pub slTriggerBy: String,
    pub reduceOnly: bool,
    pub closeOnTrigger: bool,
    pub blockTradeId: String,
    pub smpType: String,
    pub smpGroup: f64,
    pub smpOrderId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractSymbolTicker {
    pub symbol: String,
    pub bidPrice: String,
    pub askPrice: String,
    pub lastPrice: String,
    pub lastTickDirection: String,
    pub prevPrice24h: String,
    pub price24hPcnt: String,
    pub highPrice24h: String,
    pub lowPrice24h: String,
    pub prevPrice1h: String,
    pub markPrice: String,
    pub indexPrice: String,
    pub openInterest: String,
    pub turnover24h: String,
    pub volume24h: String,
    pub fundingRate: String,
    pub nextFundingTime: String,
    pub predictedDeliveryPrice: String,
    pub basisRate: String,
    pub deliveryFeeRate: String,
    pub deliveryTime: String,
}

