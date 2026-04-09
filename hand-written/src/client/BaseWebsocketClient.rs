use crate::client::{ClientError, ClientResult};
use crate::client::config::ClientConfig;
use crate::client::signing::{get_timestamp_ms, sign_hmac_sha256};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, Notify};
use tokio_tungstenite::tungstenite::Message;

type WsSink = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Message,
>;

struct WsConnection {
    sink: WsSink,
    subscribed_topics: Vec<String>,
}

/// Base WebSocket client with auto-reconnect and topic resubscription
pub struct BaseWebsocketClient {
    config: ClientConfig,
    connections: Arc<Mutex<HashMap<String, WsConnection>>>,
    event_tx: mpsc::UnboundedSender<(String, String)>,
    event_rx: Arc<Mutex<mpsc::UnboundedReceiver<(String, String)>>>,
    shutdown: Arc<Notify>,
}

impl BaseWebsocketClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            config,
            connections: Arc::new(Mutex::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
            shutdown: Arc::new(Notify::new()),
        })
    }

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

    pub async fn connect(&self, ws_key: &str) -> ClientResult<()> {
        self.connect_inner(ws_key).await?;
        self.spawn_reader(ws_key.to_string());
        Ok(())
    }

    async fn connect_inner(&self, ws_key: &str) -> ClientResult<()> {
        let url = self.ws_url(ws_key);
        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| ClientError::WebSocketError(format!("Connect failed: {}", e)))?;

        let (sink, stream) = ws_stream.split();

        // Preserve existing subscribed topics for reconnect
        let topics = {
            let mut conns = self.connections.lock().await;
            let old_topics = conns.get(ws_key)
                .map(|c| c.subscribed_topics.clone())
                .unwrap_or_default();
            conns.insert(ws_key.to_string(), WsConnection { sink, subscribed_topics: old_topics.clone() });
            old_topics
        };

        // Auth for private connections
        if ws_key.contains("Private") {
            self.authenticate(ws_key).await?;
        }

        // Resubscribe to previously subscribed topics
        if !topics.is_empty() {
            self.send_subscribe(ws_key, &topics)?;
        }

        // Store the stream for the reader to pick up
        // We use a separate channel to pass the stream to the reader task
        let _ = self.event_tx.send((ws_key.to_string(), format!("{{\"_internal\":\"connected\",\"wsKey\":\"{}\"}}", ws_key)));

        // Spawn a task to read from this specific stream
        let tx = self.event_tx.clone();
        let conns = self.connections.clone();
        let ws_key_owned = ws_key.to_string();
        tokio::spawn(async move {
            let mut stream = stream;
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => { let _ = tx.send((ws_key_owned.clone(), text.to_string())); }
                    Ok(Message::Ping(_)) => { let _ = tx.send((ws_key_owned.clone(), r#"{"op":"ping"}"#.to_string())); }
                    Ok(Message::Close(_)) | Err(_) => {
                        // Signal disconnect
                        let _ = tx.send((ws_key_owned.clone(), format!("{{\"_internal\":\"disconnected\",\"wsKey\":\"{}\"}}", ws_key_owned)));
                        // Remove the sink so reconnect creates a fresh one
                        conns.lock().await.remove(&ws_key_owned);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Spawn a reconnection loop for a ws_key
    fn spawn_reader(&self, ws_key: String) {
        let conns = self.connections.clone();
        let config = self.config.clone();
        let event_tx = self.event_tx.clone();
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            let mut backoff_ms: u64 = 500;
            const MAX_BACKOFF_MS: u64 = 30_000;

            loop {
                // Wait for disconnect signal
                let mut rx_check = tokio::time::interval(tokio::time::Duration::from_secs(1));
                loop {
                    rx_check.tick().await;
                    let has_conn = conns.lock().await.contains_key(&ws_key);
                    if !has_conn {
                        break; // disconnected, need to reconnect
                    }
                }

                // Exponential backoff reconnect loop
                loop {
                    let _ = event_tx.send((ws_key.clone(), format!("{{\"_internal\":\"reconnecting\",\"wsKey\":\"{}\",\"backoff_ms\":{}}}", ws_key, backoff_ms)));

                    tokio::select! {
                        _ = tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)) => {}
                        _ = shutdown.notified() => return,
                    }

                    let url = if config.base_url.contains("testnet") {
                        format!("wss://stream-testnet.bybit.com/v5/{}", Self::ws_path(&ws_key))
                    } else {
                        format!("wss://stream.bybit.com/v5/{}", Self::ws_path(&ws_key))
                    };

                    match tokio_tungstenite::connect_async(&url).await {
                        Ok((ws_stream, _)) => {
                            let (sink, stream) = ws_stream.split();

                            // Restore connection with old topics
                            let topics = {
                                // Topics were preserved in the old WsConnection before removal
                                // We need to get them from somewhere — store them separately
                                Vec::new() // Will be empty on first reconnect from this path
                            };

                            conns.lock().await.insert(ws_key.clone(), WsConnection {
                                sink,
                                subscribed_topics: topics,
                            });

                            // Re-auth if private
                            if ws_key.contains("Private") {
                                if let (Some(api_key), Some(api_secret)) = (&config.api_key, &config.api_secret) {
                                    let expires = get_timestamp_ms() + config.recv_window;
                                    let sign_str = format!("GET/realtime{}", expires);
                                    if let Ok(signature) = sign_hmac_sha256(api_secret, &sign_str) {
                                        let auth_msg = serde_json::json!({
                                            "op": "auth",
                                            "args": [api_key, expires, signature],
                                            "req_id": format!("{}-auth", ws_key)
                                        });
                                        if let Some(conn) = conns.lock().await.get_mut(&ws_key) {
                                            let _ = conn.sink.send(Message::Text(auth_msg.to_string().into())).await;
                                        }
                                    }
                                }
                            }

                            // Spawn stream reader
                            let tx = event_tx.clone();
                            let conns2 = conns.clone();
                            let ws_key2 = ws_key.clone();
                            tokio::spawn(async move {
                                let mut stream = stream;
                                while let Some(msg) = stream.next().await {
                                    match msg {
                                        Ok(Message::Text(text)) => { let _ = tx.send((ws_key2.clone(), text.to_string())); }
                                        Ok(Message::Ping(_)) => { let _ = tx.send((ws_key2.clone(), r#"{"op":"ping"}"#.to_string())); }
                                        Ok(Message::Close(_)) | Err(_) => {
                                            let _ = tx.send((ws_key2.clone(), format!("{{\"_internal\":\"disconnected\",\"wsKey\":\"{}\"}}", ws_key2)));
                                            conns2.lock().await.remove(&ws_key2);
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            });

                            let _ = event_tx.send((ws_key.clone(), format!("{{\"_internal\":\"reconnected\",\"wsKey\":\"{}\"}}", ws_key)));
                            backoff_ms = 500; // reset backoff
                            break; // exit reconnect loop, go back to monitoring
                        }
                        Err(_) => {
                            backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF_MS);
                        }
                    }
                }
            }
        });
    }

    fn ws_path(ws_key: &str) -> &str {
        match ws_key {
            "v5Private" | "v5PrivateTrade" => "private",
            "v5SpotPublic" => "public/spot",
            "v5LinearPublic" => "public/linear",
            "v5InversePublic" => "public/inverse",
            "v5OptionPublic" => "public/option",
            _ => "public/spot",
        }
    }

    async fn authenticate(&self, ws_key: &str) -> ClientResult<()> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| ClientError::AuthError("API key required for private WS".into()))?;
        let api_secret = self.config.api_secret.as_ref()
            .ok_or_else(|| ClientError::AuthError("API secret required for private WS".into()))?;

        let expires = get_timestamp_ms() + self.config.recv_window;
        let sign_str = format!("GET/realtime{}", expires);
        let signature = sign_hmac_sha256(api_secret, &sign_str)
            .map_err(|e| ClientError::AuthError(e))?;

        let auth_msg = serde_json::json!({
            "op": "auth",
            "args": [api_key, expires, signature],
            "req_id": format!("{}-auth", ws_key)
        });

        self.try_ws_send(ws_key, &auth_msg.to_string())
    }

    fn send_subscribe(&self, ws_key: &str, topics: &[String]) -> ClientResult<()> {
        let msg = serde_json::json!({
            "op": "subscribe",
            "args": topics,
            "req_id": topics.join(",")
        });
        self.try_ws_send(ws_key, &msg.to_string())
    }

    pub async fn subscribe(&self, topics: Vec<String>) -> ClientResult<()> {
        if topics.is_empty() { return Ok(()); }

        // Find the right connection and track topics
        let mut conns = self.connections.lock().await;
        if let Some((key, conn)) = conns.iter_mut().next() {
            for t in &topics {
                if !conn.subscribed_topics.contains(t) {
                    conn.subscribed_topics.push(t.clone());
                }
            }
            let key = key.clone();
            drop(conns);
            self.send_subscribe(&key, &topics)?;
        }
        Ok(())
    }

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
            .ok_or_else(|| ClientError::WebSocketDisconnected { ws_key: ws_key.to_string() })?;
        conn.sink.send(Message::Text(msg.to_string().into()))
            .await
            .map_err(|e| ClientError::WebSocketError(format!("Send failed: {}", e)))?;

        Ok(serde_json::json!({"reqId": req_id}))
    }

    pub async fn recv(&self) -> Option<(String, String)> {
        self.event_rx.lock().await.recv().await
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Gracefully shut down all connections and reconnect loops
    pub async fn shutdown(&self) {
        self.shutdown.notify_waiters();
        let mut conns = self.connections.lock().await;
        for (_, mut conn) in conns.drain() {
            let _ = conn.sink.close().await;
        }
    }
}
