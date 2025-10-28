// Auto-generated from TypeScript definitions
// Source: types/websockets/ws-confirmations.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;


#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebsocketTopicSubscriptionConfirmationEvent {
    pub op: String,
    pub req_id: String,
    pub conn_id: String,
    pub ret_msg: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebsocketSucceededTopicSubscriptionConfirmationEvent {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebsocketFailedTopicSubscriptionConfirmationEvent {
    pub success: bool,
}

