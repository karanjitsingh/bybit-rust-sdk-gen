// Auto-generated from TypeScript definitions
// Source: types/request/inverse.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractOrderRequest_TpslMode;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseOrderRequest {
    pub side: String,
    pub symbol: String,
    pub order_type: String,
    pub qty: f64,
    pub price: Option<f64>,
    pub time_in_force: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub reduce_only: Option<bool>,
    pub tp_trigger_by: Option<InverseOrderRequest_TpTriggerBy>,
    pub sl_trigger_by: Option<InverseOrderRequest_TpTriggerBy>,
    pub close_on_trigger: Option<bool>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseActiveOrdersRequest {
    pub symbol: String,
    pub order_status: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseCancelOrderRequest {
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseReplaceOrderRequest {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub p_r_qty: Option<f64>,
    pub p_r_price: Option<String>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseGetOrderRequest {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseConditionalOrderRequest {
    pub side: String,
    pub symbol: String,
    pub order_type: String,
    pub qty: String,
    pub price: Option<String>,
    pub base_price: String,
    pub stop_px: String,
    pub time_in_force: String,
    pub trigger_by: Option<String>,
    pub close_on_trigger: Option<bool>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseActiveConditionalOrderRequest {
    pub symbol: String,
    pub stop_order_status: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseCancelConditionalOrderRequest {
    pub symbol: String,
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseReplaceConditionalOrderRequest {
    pub stop_order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub symbol: String,
    pub p_r_qty: Option<f64>,
    pub p_r_price: Option<String>,
    pub p_r_trigger_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseChangePositionMarginRequest {
    pub symbol: String,
    pub margin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseSetTradingStopRequest {
    pub symbol: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub trailing_stop: Option<f64>,
    pub tp_trigger_by: Option<String>,
    pub sl_trigger_by: Option<String>,
    pub new_trailing_active: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseSetLeverageRequest {
    pub symbol: String,
    pub leverage: f64,
    pub leverage_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseGetTradeRecordsRequest {
    pub order_id: Option<String>,
    pub symbol: String,
    pub start_time: Option<f64>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
    pub order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseGetClosedPnlRequest {
    pub symbol: String,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub exec_type: Option<String>,
    pub page: Option<f64>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseSetSlTpPositionModeRequest {
    pub symbol: String,
    pub tp_sl_mode: ContractOrderRequest_TpslMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InverseSetMarginTypeRequest {
    pub symbol: String,
    pub is_isolated: bool,
    pub buy_leverage: f64,
    pub sell_leverage: f64,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `InverseOrderRequest:tp_trigger_by`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum InverseOrderRequest_TpTriggerBy {
        #[default]
        IndexPrice,
        LastPrice,
        MarkPrice,
    }

}
