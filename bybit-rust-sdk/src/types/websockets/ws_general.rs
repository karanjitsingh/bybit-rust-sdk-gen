// Auto-generated from TypeScript definitions
// Source: types/websockets/ws-general.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use derive_builder::Builder;

use strum_macros::{EnumString, Display};

// ============================================================================
// Skipped Type Aliases (3)
// ============================================================================
// The following type aliases were not converted. See reasons below:
//
// Type alias 'WsPublicTopics' skipped: Union with non-string-literal types: | WsPublicInverseTopic | WsPublicUSDTPerpTopic | WsPublicSpotV1Topic | WsPublicSpotV2Topic | string
// Type alias 'WsPrivateTopic' skipped: Union with non-string-literal types: | WsPrivateInverseTopic | WsPrivateUSDTPerpTopic | WsPrivateSpotTopic | string
// Type alias 'WsTopic' skipped: Union with non-string-literal types: WsPublicTopics | WsPrivateTopic

/// For spot markets, spotV3 is recommended
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum APIMarket {
    #[default]
    #[serde(rename = "v5")]
    #[strum(serialize = "v5")]
    V5,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPublicInverseTopic {
    #[default]
    orderBookL2_25,
    orderBookL2_200,
    trade,
    insurance,
    instrument_info,
    klineV2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPublicUSDTPerpTopic {
    #[default]
    orderBookL2_25,
    orderBookL2_200,
    trade,
    insurance,
    instrument_info,
    kline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPublicSpotV1Topic {
    #[default]
    trade,
    realtimes,
    kline,
    depth,
    mergedDepth,
    diffDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPublicSpotV2Topic {
    #[default]
    depth,
    kline,
    trade,
    bookTicker,
    realtimes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPrivateInverseTopic {
    #[default]
    position,
    execution,
    order,
    stop_order,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPrivateUSDTPerpTopic {
    #[default]
    position,
    execution,
    order,
    stop_order,
    wallet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsPrivateSpotTopic {
    #[default]
    outboundAccountInfo,
    executionReport,
    ticketInfo,
}

/// This is used to differentiate between each of the available websocket streams (as bybit has multiple websockets)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsKey {
    #[default]
    #[serde(rename = "v5SpotPublic")]
    #[strum(serialize = "v5SpotPublic")]
    V5SpotPublic,
    #[serde(rename = "v5LinearPublic")]
    #[strum(serialize = "v5LinearPublic")]
    V5LinearPublic,
    #[serde(rename = "v5InversePublic")]
    #[strum(serialize = "v5InversePublic")]
    V5InversePublic,
    #[serde(rename = "v5OptionPublic")]
    #[strum(serialize = "v5OptionPublic")]
    V5OptionPublic,
    #[serde(rename = "v5Private")]
    #[strum(serialize = "v5Private")]
    V5Private,
    #[serde(rename = "v5PrivateTrade")]
    #[strum(serialize = "v5PrivateTrade")]
    V5PrivateTrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]
pub enum WsMarket {
    #[default]
    #[serde(rename = "all")]
    #[strum(serialize = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WSClientConfigurableOptions {
    /// Your API key
    pub key: Option<String>,
    /// Your API secret
    pub secret: Option<String>,
    /// Set to `true` to connect to Bybit's testnet environment.
    /// Notes:
    /// - If demo trading, `testnet` should be set to false!
    /// - If testing a strategy, use demo trading instead. Testnet market data is very different from real market conditions.
    pub testnet: Option<bool>,
    /// Set to `true` to connect to Bybit's V5 demo trading: https://bybit-exchange.github.io/docs/v5/demo
    /// Only the "V5" "market" is supported here.
    pub demoTrading: Option<bool>,
    /// The API group this client should connect to. The V5 market is currently used by default.
    /// Only the "V5" "market" is supported here.
    pub market: Option<APIMarket>,
    /// Define a recv window when preparing a private websocket signature. This is in milliseconds, so 5000 == 5 seconds
    pub recvWindow: Option<f64>,
    /// How often to check if the connection is alive
    pub pingInterval: Option<f64>,
    /// How long to wait for a pong (heartbeat reply) before assuming the connection is dead
    pub pongTimeout: Option<f64>,
    /// Delay in milliseconds before respawning the connection
    pub reconnectTimeout: Option<f64>,
    pub restOptions: Option<serde_json::Value>,
    pub requestOptions: Option<serde_json::Value>,
    pub wsUrl: Option<String>,
    /// Default: false.
    /// When enabled, any calls to the subscribe method will return a promise.
    /// Note: internally, subscription requests are sent in batches. This may not behave as expected when
    /// subscribing to a large number of topics, especially if you are not yet connected when subscribing.
    pub promiseSubscribeRequests: Option<bool>,
    /// Allows you to provide a custom "signMessage" function, e.g. to use node's much faster createHmac method
    /// Look in the examples folder for a demonstration on using node's createHmac instead.
    pub customSignMessageFn: Option<serde_json::Value>,
}

/// WS configuration that's always defined, regardless of user configuration
/// (usually comes from defaults if there's no user-provided values)
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebsocketClientOptions {
    pub market: APIMarket,
    pub pongTimeout: f64,
    pub pingInterval: f64,
    pub reconnectTimeout: f64,
    pub recvWindow: f64,
    pub authPrivateConnectionsOnConnect: bool,
    pub authPrivateRequests: bool,
}

