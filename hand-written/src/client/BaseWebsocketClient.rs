use crate::client::{ClientError, ClientResult};
use crate::client::config::ClientConfig;
use crate::client::signing::{get_timestamp_ms, sign_hmac_sha256};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::tungstenite::Message;

type WsSink = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Message,
>;

/// Active WS connection handle
struct WsConnection {
    sink: WsSink,
}

/// Base WebSocket client implementation for Bybit API
pub struct BaseWebsocketClient {
    config: ClientConfig,
    connections: Arc<Mutex<HashMap<String, WsConnection>>>,
    /// Channel for incoming messages — consumers read from the receiver
    event_tx: mpsc::UnboundedSender<(String, String)>,
    event_rx: Arc<Mutex<mpsc::UnboundedReceiver<(String, String)>>>,
}

impl BaseWebsocketClient {
    /// Create a new Base WebSocket client
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            config,
            connections: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
        })
    }

    /// Get the WS base URL for a given key
    fn ws_url(&self, ws_key: &str) -> String {
        let base = if self.config.base_url.contains("testnet") {
            "wss://stream-testnet.bybit.com"
        } else {
            "wss://stream.bybit.com"
        };
        match ws_key {
            "v5Private" | "v5PrivateTrade" => format!("{}/v5/private", base),
            "v5SpotPublic" => format!("{}/v5/public/spot", base),
            "v5LinearPublic" => format!("{}/v5/public/linear", base),
            "v5InversePublic" => format!("{}/v5/public/inverse", base),
            "v5OptionPublic" => format!("{}/v5/public/option", base),
            _ => format!("{}/v5/public/spot", base),
        }
    }

    /// Connect to a WebSocket endpoint by key
    pub async fn connect(&self, ws_key: &str) -> ClientResult<()> {
        let url = self.ws_url(ws_key);
        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| ClientError::WebSocketError(format!("Connect failed: {}", e)))?;

        let (sink, mut stream) = ws_stream.split();
        let key = ws_key.to_string();

        // Store connection
        {
            let mut conns = self.connections.lock().await;
            conns.insert(key.clone(), WsConnection { sink });
        }

        // If private, authenticate
        if ws_key.contains("Private") {
            self.authenticate(ws_key).await?;
        }

        // Spawn reader task
        let tx = self.event_tx.clone();
        let ws_key_owned = ws_key.to_string();
        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => { let _ = tx.send((ws_key_owned.clone(), text.to_string())); }
                    Ok(Message::Ping(data)) => { let _ = tx.send((ws_key_owned.clone(), format!("{{\"op\":\"ping\",\"data\":\"{}\"}}", String::from_utf8_lossy(&data)))); }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Authenticate a private WS connection
    async fn authenticate(&self, ws_key: &str) -> ClientResult<()> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| ClientError::ApiError("API key required for private WS".to_string()))?;
        let api_secret = self.config.api_secret.as_ref()
            .ok_or_else(|| ClientError::ApiError("API secret required for private WS".to_string()))?;

        let expires = get_timestamp_ms() + self.config.recv_window;
        let sign_str = format!("GET/realtime{}", expires);
        let signature = sign_hmac_sha256(api_secret, &sign_str)
            .map_err(|e| ClientError::ApiError(e))?;

        let auth_msg = serde_json::json!({
            "op": "auth",
            "args": [api_key, expires, signature],
            "req_id": format!("{}-auth", ws_key)
        });

        self.try_ws_send(ws_key, &auth_msg.to_string())
    }

    /// Subscribe to topics on a WS connection
    pub async fn subscribe(&self, topics: Vec<String>) -> ClientResult<()> {
        if topics.is_empty() {
            return Ok(());
        }
        let msg = serde_json::json!({
            "op": "subscribe",
            "args": topics,
            "req_id": topics.join(",")
        });
        // Route to first available connection (simplified — full impl would route per topic)
        let conns = self.connections.lock().await;
        if let Some((key, _)) = conns.iter().next() {
            let key = key.clone();
            drop(conns);
            self.try_ws_send(&key, &msg.to_string())?;
        }
        Ok(())
    }

    /// Send a raw message on a WS connection
    pub fn try_ws_send(&self, ws_key: &str, msg: &str) -> ClientResult<()> {
        let conns = self.connections.clone();
        let key = ws_key.to_string();
        let msg = msg.to_string();
        tokio::spawn(async move {
            if let Some(conn) = conns.lock().await.get_mut(&key) {
                let _ = conn.sink.send(Message::Text(msg.into())).await;
            }
        });
        Ok(())
    }

    /// Send a WS API request (order placement, etc.)
    pub async fn send_ws_api_request(
        &self,
        ws_key: &str,
        operation: &str,
        params: serde_json::Value,
    ) -> ClientResult<serde_json::Value> {
        let req_id = format!("{}_{}", operation, get_timestamp_ms());
        let msg = serde_json::json!({
            "reqId": req_id,
            "op": operation,
            "header": {},
            "args": [params]
        });

        let mut conns = self.connections.lock().await;
        let conn = conns.get_mut(ws_key)
            .ok_or_else(|| ClientError::WebSocketError(format!("No connection for key: {}", ws_key)))?;
        conn.sink.send(Message::Text(msg.to_string().into()))
            .await
            .map_err(|e| ClientError::WebSocketError(format!("Send failed: {}", e)))?;

        // Return the request ID so callers can match responses
        Ok(serde_json::json!({"reqId": req_id}))
    }

    /// Receive the next WS event (ws_key, message_json)
    pub async fn recv(&self) -> Option<(String, String)> {
        self.event_rx.lock().await.recv().await
    }

    /// Access the client config
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
}
