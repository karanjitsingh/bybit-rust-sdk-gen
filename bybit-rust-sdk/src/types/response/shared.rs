// Auto-generated from TypeScript definitions
// Source: types/response/shared.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SymbolWalletBalance {
    pub equity: f64,
    pub available_balance: f64,
    pub used_margin: f64,
    pub order_margin: f64,
    pub position_margin: f64,
    pub occ_closing_fee: f64,
    pub occ_funding_fee: f64,
    pub wallet_balance: f64,
    pub realised_pnl: f64,
    pub unrealised_pnl: f64,
    pub cum_realised_pnl: f64,
    pub given_cash: f64,
    pub service_cash: f64,
}

