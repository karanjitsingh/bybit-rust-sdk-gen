// Auto-generated from TypeScript definitions
// Source: types/response/v5-position.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::ContractSetAutoAddMarginRequest_Side;
use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Category;
use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Status;
use crate::types::shared::inline::MovePositionResultV5_Status;
use crate::types::shared::inline::SingleAccountCoinBalanceRequestV3_WithBonus;
use crate::types::shared_v5::CategoryV5;
use crate::types::shared_v5::ExecTypeV5;
use crate::types::shared_v5::OrderSideV5;
use crate::types::shared_v5::OrderTypeV5;
use crate::types::shared_v5::PositionIdx;
use crate::types::shared_v5::PositionSideV5;
use crate::types::shared_v5::PositionStatusV5;
use crate::types::shared_v5::StopOrderTypeV5;
use crate::types::shared_v5::TPSLModeV5;
use crate::types::shared_v5::TradeModeV5;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PositionV5 {
    pub positionIdx: PositionIdx,
    pub riskId: f64,
    pub riskLimitValue: String,
    pub symbol: String,
    pub side: PositionSideV5,
    pub size: String,
    pub avgPrice: String,
    pub positionValue: String,
    pub tradeMode: TradeModeV5,
    pub autoAddMargin: Option<f64>,
    pub positionStatus: PositionStatusV5,
    pub leverage: Option<String>,
    pub markPrice: String,
    pub liqPrice: String,
    pub bustPrice: Option<String>,
    pub positionIM: Option<String>,
    pub positionMM: Option<String>,
    pub positionBalance: Option<String>,
    pub tpslMode: Option<TPSLModeV5>,
    pub takeProfit: Option<String>,
    pub stopLoss: Option<String>,
    pub trailingStop: Option<String>,
    pub sessionAvgPrice: String,
    pub delta: Option<String>,
    pub gamma: Option<String>,
    pub vega: Option<String>,
    pub theta: Option<String>,
    pub unrealisedPnl: String,
    pub curRealisedPnl: String,
    pub cumRealisedPnl: String,
    pub adlRankIndicator: f64,
    pub isReduceOnly: bool,
    pub mmrSysUpdatedTime: String,
    pub leverageSysUpdatedTime: String,
    pub createdTime: String,
    pub updatedTime: String,
    pub positionIMByMp: String,
    pub positionMMByMp: String,
    pub seq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SetRiskLimitResultV5 {
    pub category: CategoryV5,
    pub riskId: f64,
    pub riskLimitValue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AddOrReduceMarginResultV5 {
    pub category: CategoryV5,
    pub symbol: String,
    pub positionIdx: PositionIdx,
    pub riskId: f64,
    pub riskLimitValue: String,
    pub size: String,
    pub avgPrice: String,
    pub liqPrice: String,
    pub bustPrice: String,
    pub markPrice: String,
    pub positionValue: String,
    pub leverage: String,
    pub autoAddMargin: SingleAccountCoinBalanceRequestV3_WithBonus,
    pub positionStatus: PositionStatusV5,
    pub positionIM: String,
    pub positionMM: String,
    pub takeProfit: String,
    pub stopLoss: String,
    pub trailingStop: String,
    pub unrealisedPnl: String,
    pub cumRealisedPnl: String,
    pub createdTime: String,
    pub updatedTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExecutionV5 {
    pub symbol: String,
    pub orderId: String,
    pub orderLinkId: String,
    pub side: OrderSideV5,
    pub orderPrice: String,
    pub orderQty: String,
    pub leavesQty: String,
    pub orderType: OrderTypeV5,
    pub stopOrderType: Option<StopOrderTypeV5>,
    pub execFee: String,
    pub execFeeV2: String,
    /// Trading fee currency
    pub feeCurrency: String,
    pub execId: String,
    pub execPrice: String,
    pub execQty: String,
    pub execType: ExecTypeV5,
    pub execValue: String,
    pub execTime: String,
    pub isMaker: bool,
    pub feeRate: String,
    pub tradeIv: Option<String>,
    pub markIv: Option<String>,
    pub markPrice: String,
    pub indexPrice: String,
    pub underlyingPrice: Option<String>,
    pub blockTradeId: Option<String>,
    pub closedSize: Option<String>,
    pub seq: f64,
    pub extraFees: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ClosedPnLV5 {
    pub symbol: String,
    pub orderId: String,
    pub side: String,
    pub qty: String,
    pub orderPrice: String,
    pub orderType: OrderTypeV5,
    pub execType: ExecTypeV5,
    pub closedSize: String,
    pub openFee: String,
    pub closeFee: String,
    pub cumEntryValue: String,
    pub avgEntryPrice: String,
    pub cumExitValue: String,
    pub avgExitPrice: String,
    pub closedPnl: String,
    pub fillCount: String,
    pub leverage: String,
    pub createdTime: String,
    pub updatedTime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MovePositionResultV5 {
    pub blockTradeId: String,
    pub status: MovePositionResultV5_Status,
    pub rejectParty: MovePositionResultV5_RejectParty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MovePositionHistoryV5 {
    pub blockTradeId: String,
    pub category: GetMovePositionHistoryParamsV5_Category,
    pub orderId: String,
    pub userId: f64,
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub price: String,
    pub qty: String,
    pub execFee: String,
    pub status: GetMovePositionHistoryParamsV5_Status,
    pub execId: String,
    pub resultCode: f64,
    pub resultMessage: String,
    pub createdAt: f64,
    pub updatedAt: f64,
    pub rejectParty: MovePositionResultV5_RejectParty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ClosedOptionsPositionV5 {
    pub symbol: String,
    pub side: ContractSetAutoAddMarginRequest_Side,
    pub totalOpenFee: String,
    pub deliveryFee: String,
    pub totalCloseFee: String,
    pub qty: String,
    pub closeTime: f64,
    pub avgExitPrice: String,
    pub deliveryPrice: String,
    pub openTime: f64,
    pub avgEntryPrice: String,
    pub totalPnl: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `MovePositionResultV5:rejectParty`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum MovePositionResultV5_RejectParty {
        #[default]
        #[serde(rename = "")]
        #[strum(serialize = "")]
        Empty,
        Maker,
        Taker,
        bybit,
    }

}
