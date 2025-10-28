// Auto-generated from TypeScript definitions
// Source: types/request/contract.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::unified_margin::UMOrderType;
use crate::types::request::usdc_shared::USDCOrderFilter;
use crate::types::request::usdc_shared::USDCTimeInForce;
use crate::types::shared::OrderSide;
use crate::types::shared::inline::ContractOrderRequest_PositionIdx;
use crate::types::shared::inline::ContractOrderRequest_TpslMode;
use crate::types::shared::inline::ContractOrderRequest_TriggerDirection;
use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub orderType: UMOrderType,
    pub qty: String,
    pub timeInForce: USDCTimeInForce,
    pub price: Option<String>,
    pub triggerDirection: Option<ContractOrderRequest_TriggerDirection>,
    pub triggerPrice: Option<String>,
    pub triggerBy: Option<String>,
    pub positionIdx: Option<ContractOrderRequest_PositionIdx>,
    pub orderLinkId: Option<String>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub reduceOnly: Option<bool>,
    pub closeOnTrigger: Option<bool>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub tpOrderType: Option<UMOrderType>,
    pub slOrderType: Option<UMOrderType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractHistoricOrdersRequest {
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub symbol: String,
    pub orderStatus: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractCancelOrderRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractModifyOrderRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub triggerPrice: Option<String>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub triggerBy: Option<String>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractActiveOrdersRequest {
    pub symbol: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub settleCoin: Option<String>,
    pub orderFilter: Option<USDCOrderFilter>,
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractPositionsRequest {
    pub symbol: Option<String>,
    pub settleCoin: Option<String>,
    pub dataFilter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractSetAutoAddMarginRequest {
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub autoAddMargin: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub positionIdx: Option<ContractOrderRequest_PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractSetMarginSwitchRequest {
    pub symbol: String,
    pub tradeMode: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub buyLeverage: String,
    pub sellLeverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractSetPositionModeRequest {
    pub symbol: Option<String>,
    pub coin: Option<String>,
    pub mode: ContractSetPositionModeRequest_Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractSetTPSLRequest {
    pub symbol: String,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub tpslMode: Option<ContractOrderRequest_TpslMode>,
    pub tpSize: Option<String>,
    pub slSize: Option<String>,
    pub tpTriggerBy: Option<String>,
    pub slTriggerBy: Option<String>,
    pub trailingStop: Option<String>,
    pub activePrice: Option<String>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
    pub tpOrderType: Option<UMOrderType>,
    pub slOrderType: Option<UMOrderType>,
    /// 0-one-way, 1-buy side, 2-sell side
    pub positionIdx: Option<ContractOrderRequest_PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractUserExecutionHistoryRequest {
    pub symbol: String,
    pub orderId: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub execType: Option<ContractUserExecutionHistoryRequest_ExecType>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractClosedPNLRequest {
    pub symbol: String,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContractWalletFundRecordRequest {
    pub startTime: Option<String>,
    pub endTime: Option<String>,
    pub coin: Option<String>,
    pub walletFundType: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `ContractSetPositionModeRequest:mode`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractSetPositionModeRequest_Mode {
        #[default]
        #[serde(rename = "0")]
        #[strum(serialize = "0")]
        T0,
        #[serde(rename = "3")]
        #[strum(serialize = "3")]
        T3,
    }

    /// `ContractUserExecutionHistoryRequest:execType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum ContractUserExecutionHistoryRequest_ExecType {
        #[default]
        AdlTrade,
        BustTrade,
        Funding,
        Trade,
    }

}
