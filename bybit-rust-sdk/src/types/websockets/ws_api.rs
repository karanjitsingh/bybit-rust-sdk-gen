// Auto-generated from TypeScript definitions
// Source: types/websockets/ws-api.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::websockets::ws_general::WsKey;
use strum_macros::{EnumString, Display};

// Import inline types from the submodule
use self::inline::*;

// ============================================================================
// Skipped Type Aliases (3)
// ============================================================================
// The following type aliases were not converted. See reasons below:
//
// Interface 'WsAPIWsKeyTopicMap' skipped: Type-level map interface (TypeScript compile-time construct only)
// Interface 'WsAPITopicRequestParamMap' skipped: Type-level map interface (TypeScript compile-time construct only)
// Interface 'WsAPIOperationResponseMap' skipped: Type-level map interface (TypeScript compile-time construct only)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WSAPIOperation {
    #[default]
    #[serde(rename = "order.create")]
    #[strum(serialize = "order.create")]
    OrderCreate,
    #[serde(rename = "order.amend")]
    #[strum(serialize = "order.amend")]
    OrderAmend,
    #[serde(rename = "order.cancel")]
    #[strum(serialize = "order.cancel")]
    OrderCancel,
    #[serde(rename = "order.create-batch")]
    #[strum(serialize = "order.create-batch")]
    OrderCreateBatch,
    #[serde(rename = "order.amend-batch")]
    #[strum(serialize = "order.amend-batch")]
    OrderAmendBatch,
    #[serde(rename = "order.cancel-batch")]
    #[strum(serialize = "order.cancel-batch")]
    OrderCancelBatch,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsOperation {
    #[default]
    subscribe,
    unsubscribe,
    auth,
    ping,
    pong,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WsRequestOperationBybit<TWSTopic: Default> {
    pub req_id: String,
    pub op: WsOperation,
    pub args: Option<Vec<WsRequestOperationBybit_Args<TWSTopic>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSAPIRequest<TWSOperation: Default, TRequestParams: Default = ()> {
    pub reqId: String,
    pub op: TWSOperation,
    pub header: WSAPIRequest_Header,
    pub args: TRequestParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSAPIResponse<TResponseData: Default, TOperation: Default = WSAPIOperation, TResponseExtInfo: Default = ()> {
    pub wsKey: WsKey,
    /// Auto-generated
    pub reqId: String,
    pub retCode: f64,
    pub retMsg: String,
    pub op: TOperation,
    pub data: TResponseData,
    pub retExtInfo: TResponseExtInfo,
    pub header: Option<WSAPIResponse_Header>,
    pub connId: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSAPIRequest_Header {
    #[serde(rename = "X-BAPI-TIMESTAMP")]
    pub X_BAPI_TIMESTAMP: String,
    #[serde(rename = "X-BAPI-RECV-WINDOW")]
    pub X_BAPI_RECV_WINDOW: String,
    pub Referer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSAPIResponse_Header {
    #[serde(rename = "X-Bapi-Limit")]
    pub X_Bapi_Limit: String,
    #[serde(rename = "X-Bapi-Limit-Status")]
    pub X_Bapi_Limit_Status: String,
    #[serde(rename = "X-Bapi-Limit-Reset-Timestamp")]
    pub X_Bapi_Limit_Reset_Timestamp: String,
    pub Traceid: String,
    pub Timenow: String,
}

// ============================================================================
// Inline Types Submodule
// ============================================================================
// These types are inline unions/literals used only within this file

pub mod inline {
    use serde::{Deserialize, Serialize};

    /// `WsRequestOperationBybit:args`
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum WsRequestOperationBybit_Args<TWSTopic> {
        TWSTopic(TWSTopic),
        Number(f64),
        String(String),
    }

}
