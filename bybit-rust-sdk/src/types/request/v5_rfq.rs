// Auto-generated from TypeScript definitions
// Source: types/request/v5-rfq.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::GetMovePositionHistoryParamsV5_Category;
use crate::types::shared::inline::GetRFQListParamsV5_Status;
use crate::types::shared::inline::GetRFQTradeListParamsV5_Status;
use crate::types::shared::inline::MovePositionParamsV5_List_Category;
use crate::types::shared::inline::RFQTransactionV5_Side;

// Import inline types from the submodule
use self::inline::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQTransactionV5 {
    /// Product type
    pub category: MovePositionParamsV5_List_Category,
    /// Name of the trading contract
    pub symbol: String,
    /// Inquiry transaction direction
    pub side: RFQTransactionV5_Side,
    /// Transaction quantity
    pub qty: String,
    /// For spot lending, default false
    pub isLeverage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRFQParamsV5 {
    /// Array of deskCode
    pub counterparties: Vec<String>,
    /// Custom ID for inquiry form, 1-32 characters
    pub rfqLinkId: Option<String>,
    /// Whether it is anonymous inquiry, default false
    pub anonymous: Option<bool>,
    /// Inquiry label, max 36 characters
    pub strategyType: Option<String>,
    /// Transaction list, up to 10 sets
    pub list: Vec<RFQTransactionV5>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Inquiry Custom ID
    pub rfqLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RFQQuoteV5 {
    /// Product type
    pub category: GetMovePositionHistoryParamsV5_Category,
    /// Name of the trading contract
    pub symbol: String,
    /// Quote price
    pub price: String,
    /// For spot lending, default false
    pub isLeverage: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRFQQuoteParamsV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Quotation custom ID, 1-32 characters
    pub quoteLinkId: Option<String>,
    /// Whether it is anonymous quotation, default false
    pub anonymous: Option<bool>,
    /// Validity period in seconds, default 60
    pub expiresIn: Option<f64>,
    /// Quotation buy direction
    pub quoteBuyList: Option<Vec<RFQQuoteV5>>,
    /// Quotation sell direction
    pub quoteSellList: Option<Vec<RFQQuoteV5>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExecuteRFQQuoteParamsV5 {
    /// Inquiry ID
    pub rfqId: String,
    /// Quotation ID
    pub quoteId: String,
    /// The direction of the quote
    pub quoteSide: RFQTransactionV5_Side,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct CancelRFQQuoteParamsV5 {
    /// Quotation ID
    pub quoteId: Option<String>,
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Quotation Custom ID
    pub quoteLinkId: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQRealtimeParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Inquiry Custom ID
    pub rfqLinkId: Option<String>,
    /// Trader type, default 'request'
    pub traderType: Option<GetRFQRealtimeParamsV5_TraderType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQListParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Custom ID for inquiry form
    pub rfqLinkId: Option<String>,
    /// Trader type, default 'request'
    pub traderType: Option<GetRFQListParamsV5_TraderType>,
    pub status: Option<GetRFQListParamsV5_Status>,
    /// Return number of items, max 100, default 50
    pub limit: Option<f64>,
    /// Page turning mark
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQQuoteRealtimeParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Quotation ID
    pub quoteId: Option<String>,
    /// Quotation Custom ID
    pub quoteLinkId: Option<String>,
    /// Trader type, default 'quote'
    pub traderType: Option<GetRFQRealtimeParamsV5_TraderType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQHistoryParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Quotation ID
    pub quoteId: Option<String>,
    /// Quotation custom ID, can only check last 3 months
    pub quoteLinkId: Option<String>,
    /// Trader type, default 'quote'
    pub traderType: Option<GetRFQRealtimeParamsV5_TraderType>,
    pub status: Option<GetRFQListParamsV5_Status>,
    /// Return number of items, max 100, default 50
    pub limit: Option<f64>,
    /// Page turning mark
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQTradeListParamsV5 {
    /// Inquiry ID
    pub rfqId: Option<String>,
    /// Custom ID for inquiry form, can only check last 3 months
    pub rfqLinkId: Option<String>,
    /// Quotation ID
    pub quoteId: Option<String>,
    /// Quotation custom ID, can only check last 3 months
    pub quoteLinkId: Option<String>,
    /// Status
    pub status: Option<GetRFQTradeListParamsV5_Status>,
    /// Return number of items, max 100, default 50
    pub limit: Option<f64>,
    /// Page turning mark
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GetRFQPublicTradesParamsV5 {
    /// Timestamp in milliseconds, time range is 7 days
    pub startTime: Option<f64>,
    /// Timestamp in milliseconds, time range is 7 days
    pub endTime: Option<f64>,
    /// Return number of items, max 100, default 50
    pub limit: Option<f64>,
    /// Page turning mark
    pub cursor: Option<String>,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};
    use strum_macros::{EnumString, Display};

    /// `GetRFQRealtimeParamsV5:traderType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetRFQRealtimeParamsV5_TraderType {
        #[default]
        quote,
        request,
    }

    /// `GetRFQListParamsV5:traderType`
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
    pub enum GetRFQListParamsV5_TraderType {
        #[default]
        quoter,
        request,
    }

}
