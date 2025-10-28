// Auto-generated from TypeScript definitions
// Source: types/request/v5-position.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::request::contract::inline::ContractSetPositionModeRequest_Mode;
use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;
use crate::types::shared::inline::GetMarkPriceKlineParamsV5_Category;
use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Category;
use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Status;
use crate::types::shared::inline::MovePositionParamsV5_List_Category;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::ExecTypeV5;
use crate::types::shared_v5::OrderTriggerByV5;
use crate::types::shared_v5::OrderTypeV5;
use crate::types::shared_v5::PositionIdx;
use crate::types::shared_v5::TPSLModeV5;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PositionInfoParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub baseCoin: Option<String>,
    pub settleCoin: Option<String>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetLeverageParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub buyLeverage: String,
    pub sellLeverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SwitchIsolatedMarginParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub tradeMode: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub buyLeverage: String,
    pub sellLeverage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetTPSLModeParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub tpSlMode: TPSLModeV5,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SwitchPositionModeParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: Option<String>,
    pub coin: Option<String>,
    pub mode: ContractSetPositionModeRequest_Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetRiskLimitParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub riskId: f64,
    pub positionIdx: Option<PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetTradingStopParamsV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub trailingStop: Option<String>,
    pub tpTriggerBy: Option<OrderTriggerByV5>,
    pub slTriggerBy: Option<OrderTriggerByV5>,
    pub activePrice: Option<String>,
    pub tpslMode: Option<TPSLModeV5>,
    pub tpSize: Option<String>,
    pub slSize: Option<String>,
    pub tpLimitPrice: Option<String>,
    pub slLimitPrice: Option<String>,
    pub tpOrderType: Option<OrderTypeV5>,
    pub slOrderType: Option<OrderTypeV5>,
    pub positionIdx: PositionIdx,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetAutoAddMarginParamsV5 {
    pub category: String,
    pub symbol: String,
    pub autoAddMargin: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub positionIdx: Option<PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AddOrReduceMarginParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
    pub margin: String,
    pub positionIDex: Option<PositionIdx>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetExecutionListParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub orderId: Option<String>,
    pub orderLinkId: Option<String>,
    pub baseCoin: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub execType: Option<ExecTypeV5>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetClosedPnLParamsV5 {
    pub category: CategoryV5,
    pub symbol: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MovePositionParamsV5 {
    pub fromUid: String,
    pub toUid: String,
    pub list: Vec<MovePositionParamsV5_List>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetMovePositionHistoryParamsV5 {
    pub category: Option<GetMovePositionHistoryParamsV5_Category>,
    pub symbol: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub status: Option<GetMovePositionHistoryParamsV5_Status>,
    pub blockTradeId: Option<String>,
    pub limit: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ConfirmNewRiskLimitParamsV5 {
    pub category: GetMarkPriceKlineParamsV5_Category,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetClosedOptionsPositionsParamsV5 {
    pub category: String,
    pub symbol: Option<String>,
    pub startTime: Option<f64>,
    pub endTime: Option<f64>,
    pub limit: Option<f64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MovePositionParamsV5_List {
    pub category: MovePositionParamsV5_List_Category,
    pub symbol: String,
    pub price: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub qty: String,
}

