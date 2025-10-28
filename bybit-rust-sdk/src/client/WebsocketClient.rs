// Auto-generated from TypeScript definitions
// Source: /Users/karan/github/bybit-rust-sdk-gen/bybit-api/src/websocket-client.ts

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::client::BaseWebsocketClient;
use crate::client::ClientResult;
use crate::types::shared_v5::{CategoryV5};
use crate::types::websockets::ws_api::{WsOperation, WsRequestOperationBybit};
use crate::types::websockets::ws_general::{WsKey};
use crate::util::{EmittableEvent, MidflightWsRequestEvent, WsTopicRequest};


// Generated client: WebsocketClient
pub struct WebsocketClient<'a> {
    base: &'a BaseWebsocketClient,
}

impl<'a> WebsocketClient<'a> {
    /// Create a new instance of WebsocketClient
    pub fn new(base: &'a BaseWebsocketClient) -> Self {
        Self { base }
    }

    /// Request connection of all dependent (public & private) websockets, instead of waiting
    /// for automatic connection by SDK.
    pub async fn connect_all(&self) -> ClientResult<Vec<Option<serde_json::Value>>> {
        unimplemented!("Abstract method 'connectAll' must be implemented by subclass")
    }

    /// Ensures the WS API connection is active and ready.
    /// You do not need to call this, but if you call this before making any WS API requests,
    /// it can accelerate the first request (by preparing the connection in advance).
    pub async fn connect_ws_api_(&self) -> ClientResult<serde_json::Value> {
        todo!("Method implementation: connectWSAPI")
    }

    pub async fn connect_public(&self) -> ClientResult<Vec<Option<serde_json::Value>>> {
        todo!("Method implementation: connectPublic")
    }

    pub async fn connect_private(&self) -> ClientResult<serde_json::Value> {
        todo!("Method implementation: connectPrivate")
    }

    /// Subscribe to V5 topics & track/persist them.
    pub async fn subscribe_v5(&self, wsTopics: Vec<serde_json::Value>, category: CategoryV5, isPrivateTopic: Option<bool>) -> ClientResult<Vec<serde_json::Value>> {
        todo!("Method implementation: subscribeV5")
    }

    /// Unsubscribe from V5 topics & remove them from memory. They won't be re-subscribed to if the
    /// connection reconnects.
    pub async fn unsubscribe_v5(&self, wsTopics: Vec<serde_json::Value>, category: CategoryV5, isPrivateTopic: Option<bool>) -> ClientResult<Vec<serde_json::Value>> {
        todo!("Method implementation: unsubscribeV5")
    }

    /// Note: subscribeV5() might be simpler to use. The end result is the same.
    /// Request subscription to one or more topics. Pass topics as either an array of strings,
    /// or array of objects (if the topic has parameters).
    /// Objects should be formatted as {topic: string, params: object, category: CategoryV5}.
    /// - Subscriptions are automatically routed to the correct websocket connection.
    /// - Authentication/connection is automatic.
    /// - Resubscribe after network issues is automatic.
    /// Call \`unsubscribe(topics)\` to remove topics
    pub fn subscribe(&self, requests: Vec<serde_json::Value>, requestedWsKey: Option<WsKey>) -> ClientResult<()> {
        todo!("Method implementation: subscribe")
    }

    /// Note: unsubscribe() might be simpler to use. The end result is the same.
    /// Unsubscribe from one or more topics. Similar to subscribe() but in reverse.
    /// - Requests are automatically routed to the correct websocket connection.
    /// - These topics will be removed from the topic cache, so they won't be subscribed to again.
    pub fn unsubscribe(&self, requests: Vec<serde_json::Value>, wsKey: Option<WsKey>) -> ClientResult<()> {
        todo!("Method implementation: unsubscribe")
    }

    pub async fn send_ws_api_request(&self, wsKey: Option<WsKey>, operation: serde_json::Value, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        unimplemented!("Abstract method 'sendWSAPIRequest' must be implemented by subclass")
    }

    /// Internal methods - not intended for public use
    pub async fn get_ws_url(&self, wsKey: WsKey) -> ClientResult<String> {
        todo!("Method implementation: getWsUrl")
    }

    pub async fn get_ws_auth_request_event(&self, wsKey: WsKey) -> ClientResult<WsRequestOperationBybit<String>> {
        unimplemented!("Abstract method 'getWsAuthRequestEvent' must be implemented by subclass")
    }

    pub fn send_ping_event(&self, wsKey: WsKey) -> ClientResult<()> {
        todo!("Method implementation: sendPingEvent")
    }

    pub fn send_pong_event(&self, wsKey: WsKey) -> ClientResult<()> {
        todo!("Method implementation: sendPongEvent")
    }

    /// Force subscription requests to be sent in smaller batches, if a number is returned
    pub fn get_max_topics_per_subscribe_event(&self, wsKey: WsKey) -> ClientResult<serde_json::Value> {
        todo!("Method implementation: getMaxTopicsPerSubscribeEvent")
    }

    pub async fn get_ws_request_events(&self, market: String, operation: WsOperation, requests: Vec<WsTopicRequest<String, serde_json::Value>>, wsKey: WsKey) -> ClientResult<Vec<MidflightWsRequestEvent<WsRequestOperationBybit<String>>>> {
        unimplemented!("Abstract method 'getWsRequestEvents' must be implemented by subclass")
    }

    pub fn get_private_ws_keys(&self) -> ClientResult<Vec<WsKey>> {
        todo!("Method implementation: getPrivateWSKeys")
    }

    pub fn is_auth_on_connect_ws_key(&self, wsKey: WsKey) -> ClientResult<bool> {
        todo!("Method implementation: isAuthOnConnectWsKey")
    }

    /// Determines if a topic is for a private channel, using a hardcoded list of strings
    pub fn is_private_topic_request(&self, request: WsTopicRequest<String, serde_json::Value>) -> ClientResult<bool> {
        todo!("Method implementation: isPrivateTopicRequest")
    }

    pub fn is_ws_ping(&self, msg: serde_json::Value) -> ClientResult<bool> {
        todo!("Method implementation: isWsPing")
    }

    pub fn is_ws_pong(&self, msg: serde_json::Value) -> ClientResult<bool> {
        todo!("Method implementation: isWsPong")
    }

    /// Abstraction called to sort ws events into emittable event types (response to a request, data update, etc)
    pub fn resolve_emittable_events(&self, wsKey: WsKey, event: serde_json::Value) -> ClientResult<Vec<EmittableEvent<serde_json::Value>>> {
        todo!("Method implementation: resolveEmittableEvents")
    }

}

