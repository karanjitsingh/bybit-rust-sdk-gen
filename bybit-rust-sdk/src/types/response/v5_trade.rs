// Auto-generated from TypeScript definitions
// Source: types/response/v5-trade.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractOrderRequest_TriggerDirection;
use crate::types::shared::inline::OrderParamsV5_MarketUnit;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::OrderCancelTypeV5;
use crate::types::shared_v5::OrderCreateTypeV5;
use crate::types::shared_v5::OrderRejectReasonV5;
use crate::types::shared_v5::OrderSideV5;
use crate::types::shared_v5::OrderStatusV5;
use crate::types::shared_v5::OrderTimeInForceV5;
use crate::types::shared_v5::OrderTriggerByV5;
use crate::types::shared_v5::OrderTypeV5;
use crate::types::shared_v5::PositionIdx;
use crate::types::shared_v5::StopOrderTypeV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OrderResultV5 {
    pub orderId: String,
    pub orderLinkId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AccountOrderV5 {
    pub orderId: String,
    pub orderLinkId: String,
    pub blockTradeId: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: OrderSideV5,
    pub isLeverage: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub positionIdx: PositionIdx,
    pub orderStatus: OrderStatusV5,
    pub createType: OrderCreateTypeV5,
    pub cancelType: OrderCancelTypeV5,
    pub rejectReason: OrderRejectReasonV5,
    pub avgPrice: String,
    pub leavesQty: String,
    pub leavesValue: String,
    pub cumExecQty: String,
    pub cumExecValue: String,
    pub cumExecFee: String,
    pub timeInForce: OrderTimeInForceV5,
    pub orderType: OrderTypeV5,
    pub stopOrderType: StopOrderTypeV5,
    pub orderIv: String,
    pub marketUnit: OrderParamsV5_MarketUnit,
    pub slippageToleranceType: String,
    pub slippageTolerance: String,
    pub triggerPrice: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub tpslMode: AccountOrderV5_TpslMode,
    pub ocoTriggerType: AccountOrderV5_OcoTriggerType,
    pub tpLimitPrice: String,
    pub slLimitPrice: String,
    pub tpTriggerBy: OrderTriggerByV5,
    pub slTriggerBy: OrderTriggerByV5,
    pub triggerDirection: ContractOrderRequest_TriggerDirection,
    pub triggerBy: OrderTriggerByV5,
    pub lastPriceOnCreated: String,
    pub reduceOnly: bool,
    pub closeOnTrigger: bool,
    pub placeType: AccountOrderV5_PlaceType,
    pub smpType: String,
    pub smpGroup: f64,
    pub smpOrderId: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub extraFees: String,
    /// Cumulative trading fee details instead of cumExecFee
    pub cumFeeDetail: Option<indexmap::IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchCreateOrderResultV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub createAt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchOrdersRetExtInfoV5 {
    pub list: Vec<BatchOrdersRetExtInfoV5_List>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchAmendOrderResultV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub orderId: String,
    pub orderLinkId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchCancelOrderResultV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub orderId: String,
    pub orderLinkId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SpotBorrowCheckResultV5 {
    pub symbol: String,
    pub side: OrderSideV5,
    pub maxTradeQty: String,
    pub maxTradeAmount: String,
    pub spotMaxTradeQty: String,
    pub spotMaxTradeAmount: String,
    pub borrowCoin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PreCheckOrderResultV5 {
    pub orderId: String,
    pub orderLinkId: String,
    /// Initial margin rate before checking (in basis points)
    pub preImrE4: f64,
    /// Maintenance margin rate before checking (in basis points)
    pub preMmrE4: f64,
    /// Initial margin rate after checking (in basis points)
    pub postImrE4: f64,
    /// Maintenance margin rate after checking (in basis points)
    pub postMmrE4: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BatchOrdersRetExtInfoV5_List {
    pub code: f64,
    pub msg: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `AccountOrderV5:tpslMode`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum AccountOrderV5_TpslMode {
        #[default]
        #[serde(rename = "")]
        #[strum(serialize = "")]
        Empty,
        Full,
        Partial,
    }

    /// `AccountOrderV5:ocoTriggerType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum AccountOrderV5_OcoTriggerType {
        #[default]
        #[serde(rename = "")]
        #[strum(serialize = "")]
        Empty,
        OcoTriggerBySl,
        OcoTriggerByUnknown,
        OcoTriggerTp,
    }

    /// `AccountOrderV5:placeType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum AccountOrderV5_PlaceType {
        #[default]
        #[serde(rename = "")]
        #[strum(serialize = "")]
        Empty,
        iv,
        price,
    }

}
