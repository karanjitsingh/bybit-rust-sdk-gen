// Auto-generated from TypeScript definitions
// Source: types/request/v5-broker.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetExchangeBrokerEarningsParamsV5_BizType;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetExchangeBrokerEarningsParamsV5 {
    pub bizType: Option<GetExchangeBrokerEarningsParamsV5_BizType>,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub uid: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBrokerSubAccountDepositsV5 {
    pub id: Option<String>,
    pub txID: Option<String>,
    pub subMemberId: Option<String>,
    pub coin: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct IssueVoucherParamsV5 {
    pub accountId: String,
    pub awardId: String,
    pub specCode: String,
    pub amount: String,
    pub brokerId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetBrokerIssuedVoucherParamsV5 {
    pub accountId: String,
    pub awardId: String,
    pub specCode: String,
    pub withUsedAmount: Option<bool>,
}

