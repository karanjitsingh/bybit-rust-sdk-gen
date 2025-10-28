// Auto-generated from TypeScript definitions
// Source: types/request/v5-trade.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractOrderRequest_PositionIdx;
use crate::types::shared::inline::ContractOrderRequest_TpslMode;
use crate::types::shared::inline::ContractOrderRequest_TriggerDirection;
use crate::types::shared::inline::OrderParamsV5_MarketUnit;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::OrderFilterV5;
use crate::types::shared_v5::OrderSMPTypeV5;
use crate::types::shared_v5::OrderSideV5;
use crate::types::shared_v5::OrderStatusV5;
use crate::types::shared_v5::OrderTimeInForceV5;
use crate::types::shared_v5::OrderTriggerByV5;
use crate::types::shared_v5::OrderTypeV5;
use crate::types::shared_v5::PositionIdx;
use crate::types::shared_v5::StopOrderTypeV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OrderParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub isLeverage: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub side: OrderSideV5,
    pub orderType: OrderTypeV5,
    pub qty: String,
    pub marketUnit: Option<OrderParamsV5_MarketUnit>,
    pub slippageToleranceType: Option<String>,
    pub slippageTolerance: Option<String>,
    pub price: Option<String>,
    pub triggerDirection: Option<ContractOrderRequest_TriggerDirection>,
    pub orderFilter: Option<OrderFilterV5>,
    pub triggerPrice: Option<String>,
    pub triggerBy: Option<OrderTriggerByV5>,
    pub orderIv: Option<String>,
    pub timeInForce: Option<OrderTimeInForceV5>,
    pub positionIdx: Option<PositionIdx>,
    pub orderLinkId: Option<String>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<OrderTriggerByV5>,
    pub slTriggerBy: Option<OrderTriggerByV5>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub smpType: Option<OrderSMPTypeV5>,
    pub mmp: Option<bool>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
    pub tpOrderType: Option<OrderTypeV5>,
    pub slOrderType: Option<OrderTypeV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AmendOrderParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderIv: Option<String>,
    pub triggerPrice: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<OrderTriggerByV5>,
    pub slTriggerBy: Option<OrderTriggerByV5>,
    pub triggerBy: Option<OrderTriggerByV5>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelOrderParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<OrderFilterV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetAccountOrdersParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub openOnly: Option<ContractOrderRequest_PositionIdx>,
    pub orderFilter: Option<OrderFilterV5>,
    pub orderStatus: Option<OrderStatusV5>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetAccountHistoricOrdersParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderFilter: Option<OrderFilterV5>,
    pub orderStatus: Option<OrderStatusV5>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelAllOrdersParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub orderFilter: Option<OrderFilterV5>,
    pub stopOrderType: Option<StopOrderTypeV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchOrderParamsV5 {
    pub symbol: String,
    pub side: OrderSideV5,
    pub isLeverage: Option<SingleAccountCoinBalanceRequestV3_WithBonus>,
    pub orderType: OrderTypeV5,
    pub qty: String,
    pub price: Option<String>,
    pub triggerDirection: Option<ContractOrderRequest_TriggerDirection>,
    pub triggerBy: Option<OrderTriggerByV5>,
    pub orderIv: Option<String>,
    pub timeInForce: Option<OrderTimeInForceV5>,
    pub positionIdx: Option<PositionIdx>,
    pub orderLinkId: Option<String>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<OrderTriggerByV5>,
    pub slTriggerBy: Option<OrderTriggerByV5>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub smpType: Option<OrderSMPTypeV5>,
    pub mmp: Option<bool>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
    pub tpOrderType: Option<OrderTypeV5>,
    pub slOrderType: Option<OrderTypeV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchAmendOrderParamsV5 {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub orderIv: Option<String>,
    pub triggerPrice: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<OrderTriggerByV5>,
    pub slTriggerBy: Option<OrderTriggerByV5>,
    pub triggerBy: Option<OrderTriggerByV5>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchCancelOrderParamsV5 {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

