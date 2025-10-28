// Auto-generated from TypeScript definitions
// Source: types/response/usdt-perp.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PerpPosition {
    pub user_id: f64,
    pub symbol: String,
    pub side: String,
    pub size: f64,
    pub position_value: f64,
    pub entry_price: f64,
    pub liq_price: f64,
    pub bust_price: f64,
    pub leverage: f64,
    pub auto_add_margin: f64,
    pub is_isolated: bool,
    pub position_margin: f64,
    pub occ_closing_fee: f64,
    pub realised_pnl: f64,
    pub cum_realised_pnl: f64,
    pub free_qty: f64,
    pub tp_sl_mode: String,
    pub unrealised_pnl: f64,
    pub deleverage_indicator: f64,
    pub risk_id: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub trailing_stop: f64,
    pub position_idx: f64,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PerpPositionRoot {
    pub data: PerpPosition,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LinearOrder {
    pub order_id: String,
    pub user_id: f64,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: f64,
    pub qty: f64,
    pub time_in_force: String,
    pub order_status: String,
    pub last_exec_price: f64,
    pub cum_exec_qty: f64,
    pub cum_exec_value: f64,
    pub cum_exec_fee: f64,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub order_link_id: String,
    pub created_time: String,
    pub updated_time: String,
    pub take_profit: f64,
    pub stop_loss: f64,
    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub position_idx: f64,
}

