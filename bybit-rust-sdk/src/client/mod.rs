// Client module declarations

// Type aliases and error types
pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}

// Client configuration types (extracted from TypeScript client files)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeferredPromiseRef(pub String);

/// Configurable options specific to only the REST-like WebsocketAPIClient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSAPIClientConfigurableOptions {
    pub attachEventListeners: bool,
}

// Core infrastructure
pub mod config;
pub mod signing;

// Base client modules and re-exports (hand-written, not generated)
#[path = "BaseRestClient.rs"]
mod base_rest_client;
pub use base_rest_client::BaseRestClient;

#[path = "BaseWebsocketClient.rs"]
mod base_websocket_client;
pub use base_websocket_client::BaseWebsocketClient;

// Generated API clients
pub mod RestClientV5;
pub mod SpotClientV3;
pub mod WebsocketAPIClient;
pub mod WebsocketClient;
