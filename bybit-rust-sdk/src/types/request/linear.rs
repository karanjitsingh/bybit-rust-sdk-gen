// Auto-generated from TypeScript definitions
// Source: types/request/linear.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::constants::enums::LinearPositionIdx;
use crate::types::shared::OrderSide;
use crate::types::shared::inline::ContractOrderRequest_PositionIdx;
use crate::types::shared::inline::ContractOrderRequest_TpslMode;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum LinearOrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum LinearTimeInForce {
    #[default]
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetOrdersRequest {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub order: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
    pub order_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearCancelOrderRequest {
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearReplaceOrderRequest {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub p_r_qty: Option<f64>,
    pub p_r_price: Option<f64>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetOrderRequest {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct NewLinearOrder {
    pub side: OrderSide,
    pub symbol: String,
    pub order_type: LinearOrderType,
    pub qty: f64,
    pub price: Option<f64>,
    pub time_in_force: LinearTimeInForce,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub order_link_id: Option<String>,
    pub position_idx: Option<LinearPositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearConditionalOrderRequest {
    pub side: String,
    pub symbol: String,
    pub order_type: String,
    pub qty: f64,
    pub price: Option<f64>,
    pub base_price: f64,
    pub stop_px: f64,
    pub time_in_force: String,
    pub trigger_by: Option<String>,
    pub close_on_trigger: Option<bool>,
    pub order_link_id: Option<String>,
    pub reduce_only: bool,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetConditionalOrderRequest {
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub stop_order_status: Option<String>,
    pub order: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearCancelConditionalOrderRequest {
    pub symbol: String,
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearReplaceConditionalOrderRequest {
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub p_r_qty: Option<f64>,
    pub p_r_price: Option<f64>,
    pub p_r_trigger_price: Option<f64>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearQueryConditionalOrderRequest {
    pub symbol: String,
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetAutoAddMarginRequest {
    pub symbol: String,
    pub side: String,
    pub auto_add_margin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetMarginSwitchRequest {
    pub symbol: String,
    pub is_isolated: bool,
    pub buy_leverage: f64,
    pub sell_leverage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetPositionModeRequest {
    pub symbol: String,
    pub mode: LinearSetPositionModeRequest_Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetPositionTpSlModeRequest {
    pub symbol: String,
    pub tp_sl_mode: ContractOrderRequest_TpslMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetAddReduceMarginRequest {
    pub symbol: String,
    pub side: String,
    pub margin: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetUserLeverageRequest {
    pub symbol: String,
    pub buy_leverage: f64,
    pub sell_leverage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetTradingStopRequest {
    pub symbol: String,
    pub side: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub trailing_stop: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
    pub sl_size: Option<f64>,
    pub tp_size: Option<f64>,
    pub position_idx: Option<ContractOrderRequest_PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetTradeRecordsRequest {
    pub symbol: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub exec_type: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetHistoryTradeRecordsRequest {
    pub symbol: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub exec_type: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearGetClosedPnlRequest {
    pub symbol: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub exec_type: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearSetRiskLimitRequest {
    pub symbol: String,
    pub side: String,
    pub risk_id: f64,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `LinearSetPositionModeRequest:mode`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum LinearSetPositionModeRequest_Mode {
        #[default]
        BothSide,
        MergedSingle,
    }

}
