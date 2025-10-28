// Auto-generated from TypeScript definitions
// Source: util/typeGuards.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use crate::types::shared::inline::MovePositionParamsV5_List_Category;
use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum SignEncodeMethod {
    #[default]
    hex,
    base64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum SignAlgorithm {
    #[default]
    #[serde(rename = "SHA-256")]
    #[strum(serialize = "SHA-256")]
    SHA256,
    #[serde(rename = "SHA-512")]
    #[strum(serialize = "SHA-512")]
    SHA512,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum KeyType {
    #[default]
    HMAC,
    #[serde(rename = "RSASSA-PKCS1-v1_5")]
    #[strum(serialize = "RSASSA-PKCS1-v1_5")]
    RSASSAPKCS1V15,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PublicPrivateNetwork {
    #[default]
    public,
    private,
}

/// The following WS keys are logical.
/// They're not directly used as a market. They usually have one private endpoint but many public ones,
/// so they need a bit of extra handling for seamless messaging between endpoints.
/// For the unified keys, the "split" happens using the symbol. Symbols suffixed with USDT are obviously USDT topics.
/// For the v5 endpoints, the subscribe/unsubscribe call must specify the category the subscription should route to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum PublicOnlyWsKeys {
    #[default]
    v5SpotPublic,
    v5LinearPublic,
    v5InversePublic,
    v5OptionPublic,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BybitEventV5<TData: Default> {
    pub topic: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub ts: f64,
    pub data: TData,
    pub wsKey: String,
}

/// Normalised internal format for a request (subscribe/unsubscribe/etc) on a topic, with optional parameters.
/// - Topic: the topic this event is for
/// - Payload: the parameters to include, optional. E.g. auth requires key + sign. Some topics allow configurable parameters.
/// - Category: required for bybit, since different categories have different public endpoints
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WsTopicRequest<TWSTopic: Default, TWSPayload: Default> {
    pub topic: TWSTopic,
    pub payload: Option<TWSPayload>,
    pub category: Option<MovePositionParamsV5_List_Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct NetworkMapV3 {
    pub livenet: String,
    pub livenet2: Option<String>,
    pub testnet: String,
    pub testnet2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EmittableEvent<TEvent: Default> {
    pub eventType: EmittableEvent_EventType,
    pub event: TEvent,
    pub isWSAPIResponse: Option<bool>,
}

/// A midflight WS request event (e.g. subscribe to these topics).
/// - requestKey: unique identifier for this request, if available. Can be anything as a string.
/// - requestEvent: the raw request, as an object, that will be sent on the ws connection. This may contain multiple topics/requests in one object, if the exchange supports it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MidflightWsRequestEvent<TEvent: Default> {
    pub requestKey: String,
    pub requestEvent: TEvent,
}

