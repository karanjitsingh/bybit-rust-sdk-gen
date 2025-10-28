// Auto-generated from TypeScript definitions
// Source: types/response/spot.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotBalance {
    pub coin: String,
    pub coinId: String,
    pub coinName: String,
    pub total: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotBalances {
    pub balances: Vec<SpotBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotLastPrice {
    pub symbol: String,
    pub price: String,
}

