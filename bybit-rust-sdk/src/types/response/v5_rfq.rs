// Auto-generated from TypeScript definitions
// Source: types/response/v5-rfq.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Category;
use crate::types::shared::inline::GetRFQListParamsV5_Status;
use crate::types::shared::inline::GetRFQTradeListParamsV5_Status;
use crate::types::shared::inline::MovePositionResultV5_Status;
use crate::types::shared::inline::RFQTransactionV5_Side;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQConfigV5 {
    /// Own deskCode, unique identification code
    pub deskCode: String,
    /// Maximum number of legs
    pub maxLegs: f64,
    /// Maximum number of LPs selected in inquiry form
    pub maxLP: f64,
    /// Maximum number of unfinished inquiry orders allowed
    pub maxActiveRfq: f64,
    /// Spot minimum order quantity multiplier
    pub minLimitQtySpotOrder: f64,
    /// Contract minimum order quantity multiplier
    pub minLimitQtyContractOrder: f64,
    /// Option minimum order multiplier
    pub minLimitQtyOptionOrder: f64,
    pub strategyTypes: Vec<RFQConfigV5_StrategyTypes>,
    pub counterparties: Vec<RFQConfigV5_Counterparties>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQCounterpartyV5 {
    /// Name of the bidder
    pub traderName: String,
    /// Unique identification code of the quotation party
    pub deskCode: String,
    /// Quoter type. LP is automated market maker, null means normal quote party
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRFQResultV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Inquiry Custom ID
    pub rfqLinkId: String,
    /// Status of the inquiry form
    pub status: CreateRFQResultV5_Status,
    /// Expiration time in milliseconds Unix timestamp
    pub expiresAt: String,
    /// Inquiry party unique identification code
    pub deskCode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQResultV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Inquiry Custom ID
    pub rfqLinkId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQItemV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Inquiry Custom ID
    pub rfqLinkId: String,
    /// Cancel success or failure, 0 means success
    pub code: f64,
    /// Cancellation failure reason
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelAllRFQResultV5 {
    /// Array of cancellation results
    pub data: Vec<CancelRFQItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRFQQuoteResultV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Quotation ID
    pub quoteId: String,
    /// Quotation Custom ID
    pub quoteLinkId: String,
    /// Expiration time in milliseconds Unix timestamp
    pub expiresAt: String,
    /// Quoter's unique identification code
    pub deskCode: String,
    /// Status of quotation
    pub status: CreateRFQResultV5_Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExecuteRFQQuoteResultV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Inquiry Custom ID
    pub rfqLinkId: String,
    /// Quotation ID
    pub quoteId: String,
    /// Order status
    pub status: MovePositionResultV5_Status,
    /// Empty means passed, "Taker", "Maker", "Bybit" when rejected
    pub rejectParty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQQuoteResultV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Quotation ID
    pub quoteId: String,
    /// Quotation Custom ID
    pub quoteLinkId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQQuoteItemV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Quotation ID
    pub quoteId: String,
    /// Quotation Custom ID
    pub quoteLinkId: String,
    /// Cancel success or failure, 0 means success
    pub code: f64,
    /// Cancellation failure reason
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQLegV5 {
    /// Product category
    pub category: GetMovePositionHistoryParamsV5_Category,
    /// The unique instrument ID
    pub symbol: String,
    /// Inquiry direction
    pub side: RFQTransactionV5_Side,
    /// Order quantity of the instrument
    pub qty: String,
    /// For spot lending
    pub isLeverage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQItemV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Custom ID for inquiry form
    pub rfqLinkId: String,
    /// List of bidders
    pub counterparties: Vec<String>,
    /// Expiration time in milliseconds Unix timestamp
    pub expiresAt: String,
    /// Inquiry label
    pub strategyType: String,
    pub status: GetRFQListParamsV5_Status,
    /// Unique identification code of the inquiry party
    pub deskCode: String,
    /// Time when the trade is created in epoch
    pub createdAt: f64,
    /// Time when the trade is updated in epoch
    pub updatedAt: f64,
    /// Combination transaction
    pub legs: Vec<RFQLegV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQRealtimeResultV5 {
    /// Array of RFQ items
    pub list: Vec<RFQItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQHistory {
    /// Page turning mark
    pub cursor: String,
    /// Array of RFQ items
    pub list: Vec<RFQItemV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct QuoteLegV5 {
    /// Product type
    pub category: GetMovePositionHistoryParamsV5_Category,
    /// The unique instrument ID or name of trading contract
    pub symbol: String,
    /// Order price or quote price
    pub price: String,
    /// Order quantity
    pub qty: Option<String>,
    /// For spot lending
    pub isLeverage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQQuoteItemV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Custom ID for inquiry form
    pub rfqLinkId: String,
    /// Quotation ID
    pub quoteId: String,
    /// Quotation custom ID
    pub quoteLinkId: String,
    /// Expiration time in milliseconds Unix timestamp
    pub expiresAt: String,
    /// Unique identification code of quotation party
    pub deskCode: String,
    pub status: GetRFQListParamsV5_Status,
    /// Execute quote direction, buy or sell
    pub execQuoteSide: String,
    /// Time when the trade is created in epoch
    pub createdAt: f64,
    /// Time when the trade is updated in epoch
    pub updatedAt: f64,
    /// Quotation buy direction
    pub quoteBuyList: Vec<QuoteLegV5>,
    /// Quotation sell direction
    pub quoteSellList: Vec<QuoteLegV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQTradeLegV5 {
    /// Product category
    pub category: GetMovePositionHistoryParamsV5_Category,
    /// Bybit order ID
    pub orderId: String,
    /// The unique instrument ID
    pub symbol: String,
    /// Direction
    pub side: RFQTransactionV5_Side,
    /// Execution price
    pub price: String,
    /// Number of executions
    pub qty: String,
    /// For spot lending
    pub isLeverage: Option<bool>,
    /// MarkPrice (contract) at transaction time, indexPrice for spot
    pub markPrice: String,
    /// Fee for taker or maker in base currency
    pub execFee: String,
    /// Unique exec(trade) ID from exchange
    pub execId: String,
    /// Status code, 0 means success
    pub resultCode: f64,
    /// Error message about resultCode
    pub resultMessage: String,
    /// Empty if Filled, "Taker"/"Maker"/"bybit" if Rejected
    pub rejectParty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQTradeV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Completed inquiry form and executed quotation ID
    pub quoteId: String,
    /// Executed quotation direction
    pub quoteSide: RFQTransactionV5_Side,
    /// Inquiry label
    pub strategyType: String,
    /// Status
    pub status: GetRFQTradeListParamsV5_Status,
    /// Unique identification code of inquiry party
    pub rfqDeskCode: String,
    /// Unique identification code of quotation party
    pub quoteDestCode: String,
    /// Time when trade is created in epoch
    pub createdAt: f64,
    /// Time when trade is updated in epoch
    pub updatedAt: f64,
    /// Combination transaction
    pub legs: Vec<RFQTradeLegV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQPublicTradeLegV5 {
    /// Product category
    pub category: GetMovePositionHistoryParamsV5_Category,
    /// The unique instrument ID
    pub symbol: String,
    /// Inquiry direction
    pub side: RFQTransactionV5_Side,
    /// Execution price
    pub price: String,
    /// Number of executions
    pub qty: String,
    /// MarkPrice (contract) at transaction time, indexPrice for spot
    pub markPrice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQPublicTradeV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Inquiry label
    pub strategyType: String,
    /// Time when trade is created in epoch
    pub createdAt: f64,
    /// Time when trade is updated in epoch
    pub updatedAt: f64,
    /// Combination transaction
    pub legs: Vec<RFQPublicTradeLegV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQConfigV5_StrategyTypes {
    pub strategyName: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQConfigV5_Counterparties {
    pub strategyName: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `CreateRFQResultV5:status`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum CreateRFQResultV5_Status {
        #[default]
        Active,
        Canceled,
        Expired,
        Failed,
        Filled,
    }

}
