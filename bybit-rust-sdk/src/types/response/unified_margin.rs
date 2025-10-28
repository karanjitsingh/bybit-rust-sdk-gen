// Auto-generated from TypeScript definitions
// Source: types/response/unified-margin.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMPaginatedResult<List: Default> {
    pub nextPageCursor: String,
    pub category: String,
    pub list: Vec<List>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMLeverageFilter {
    pub minLeverage: String,
    pub maxLeverage: String,
    pub leverageStep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMPriceFilter {
    pub minPrice: String,
    pub maxPrice: String,
    pub tickSize: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMLotSizeFilter {
    pub maxTradingQty: String,
    pub minTradingQty: String,
    pub qtyStep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMInstrumentInfo {
    pub symbol: String,
    pub contractType: String,
    pub status: String,
    pub baseCoin: String,
    pub quoteCoin: String,
    pub launchTime: String,
    pub deliveryTime: String,
    pub deliveryFeeRate: String,
    pub priceScale: String,
    pub leverageFilter: UMLeverageFilter,
    pub priceFilter: UMPriceFilter,
    pub lotSizeFilter: UMLotSizeFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct UMHistoricOrder {
    pub symbol: String,
    pub orderType: String,
    pub orderLinkId: String,
    pub orderId: String,
    pub stopOrderType: String,
    pub orderStatus: String,
    pub takeProfit: String,
    pub cumExecValue: String,
    pub blockTradeId: String,
    pub rejectReason: String,
    pub price: String,
    pub createdTime: f64,
    pub tpTriggerBy: String,
    pub timeInForce: String,
    pub basePrice: String,
    pub leavesValue: String,
    pub updatedTime: f64,
    pub side: String,
    pub triggerPrice: String,
    pub cumExecFee: String,
    pub slTriggerBy: String,
    pub leavesQty: String,
    pub closeOnTrigger: bool,
    pub cumExecQty: String,
    pub reduceOnly: bool,
    pub qty: String,
    pub stopLoss: String,
    pub triggerBy: String,
    pub orderIM: String,
}

