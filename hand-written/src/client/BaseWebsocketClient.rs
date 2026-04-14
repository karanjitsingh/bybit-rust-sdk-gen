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

/// Base WebSocket client with auto-reconnect and topic resubscription
pub struct BaseWebsocketClient {
    config: ClientConfig,
    /// Active WS sinks (removed on disconnect, re-inserted on reconnect)
    sinks: Arc<Mutex<HashMap<String, WsSink>>>,
    /// Topic state survives disconnects — never cleared except by unsubscribe
    topics: Arc<Mutex<HashMap<String, Vec<String>>>>,
    event_tx: mpsc::UnboundedSender<(String, String)>,
    event_rx: Arc<Mutex<mpsc::UnboundedReceiver<(String, String)>>>,
    shutdown: Arc<Notify>,
}

impl BaseWebsocketClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            config,
            sinks: Arc::new(Mutex::new(HashMap::new())),
            topics: Arc::new(Mutex::new(HashMap::new())),
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
        format!("{}/v5/{}", base, Self::ws_path(ws_key))
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

    /// Connect and start the reconnect monitor for this ws_key
    pub async fn connect(&self, ws_key: &str) -> ClientResult<()> {
        self.connect_and_setup(ws_key).await?;
        self.spawn_reconnect_monitor(ws_key.to_string());
        Ok(())
    }

    /// Establish WS connection, auth if private, resubscribe tracked topics
    async fn connect_and_setup(&self, ws_key: &str) -> ClientResult<()> {
        let url = self.ws_url(ws_key);
        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| ClientError::WebSocketError(format!("Connect failed: {}", e)))?;

        let (sink, stream) = ws_stream.split();
        self.sinks.lock().await.insert(ws_key.to_string(), sink);

        if ws_key.contains("Private") {
            self.authenticate(ws_key).await?;
        }

        // Resubscribe to tracked topics
        let tracked = self.topics.lock().await.get(ws_key).cloned().unwrap_or_default();
        if !tracked.is_empty() {
            self.send_subscribe(ws_key, &tracked)?;
        }

        self.spawn_stream_reader(ws_key.to_string(), stream);
        let _ = self.event_tx.send((ws_key.to_string(), format!("{{\"_internal\":\"connected\",\"wsKey\":\"{}\"}}", ws_key)));
        Ok(())
    }

    /// Read from a WS stream, emit events, signal disconnect on close/error
    fn spawn_stream_reader(
        &self,
        ws_key: String,
        stream: futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        >,
    ) {
        let tx = self.event_tx.clone();
        let sinks = self.sinks.clone();
        tokio::spawn(async move {
            let mut stream = stream;
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => { let _ = tx.send((ws_key.clone(), text.to_string())); }
                    Ok(Message::Ping(_)) => { let _ = tx.send((ws_key.clone(), r#"{"op":"ping"}"#.to_string())); }
                    Ok(Message::Close(_)) | Err(_) => {
                        let _ = tx.send((ws_key.clone(), format!("{{\"_internal\":\"disconnected\",\"wsKey\":\"{}\"}}", ws_key)));
                        sinks.lock().await.remove(&ws_key);
                        break;
                    }
                    _ => {}
                }
            }
        });
    }

    /// Monitor for disconnects and reconnect with exponential backoff
    fn spawn_reconnect_monitor(&self, ws_key: String) {
        let sinks = self.sinks.clone();
        let topics = self.topics.clone();
        let config = self.config.clone();
        let event_tx = self.event_tx.clone();
        let shutdown = self.shutdown.clone();

        tokio::spawn(async move {
            let mut backoff_ms: u64 = 500;
            const MAX_BACKOFF: u64 = 30_000;

            loop {
                // Poll until disconnected
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    if !sinks.lock().await.contains_key(&ws_key) { break; }
                }

                // Reconnect loop with exponential backoff
                loop {
                    let _ = event_tx.send((ws_key.clone(), format!(
                        "{{\"_internal\":\"reconnecting\",\"wsKey\":\"{}\",\"backoff_ms\":{}}}", ws_key, backoff_ms
                    )));

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
                            sinks.lock().await.insert(ws_key.clone(), sink);

                            // Re-auth private
                            if ws_key.contains("Private") {
                                if let (Some(api_key), Some(api_secret)) = (&config.api_key, &config.api_secret) {
                                    let expires = get_timestamp_ms() + config.recv_window;
                                    if let Ok(sig) = sign_hmac_sha256(api_secret, &format!("GET/realtime{}", expires)) {
                                        let auth = serde_json::json!({"op":"auth","args":[api_key,expires,sig],"req_id":format!("{}-auth",ws_key)});
                                        if let Some(s) = sinks.lock().await.get_mut(&ws_key) {
                                            let _ = s.send(Message::Text(auth.to_string().into())).await;
                                        }
                                    }
                                }
                            }

                            // Resubscribe from topic state (survives disconnect)
                            let tracked = topics.lock().await.get(&ws_key).cloned().unwrap_or_default();
                            if !tracked.is_empty() {
                                let sub = serde_json::json!({"op":"subscribe","args":tracked,"req_id":tracked.join(",")});
                                if let Some(s) = sinks.lock().await.get_mut(&ws_key) {
                                    let _ = s.send(Message::Text(sub.to_string().into())).await;
                                }
                            }

                            // Spawn reader for new stream
                            let tx = event_tx.clone();
                            let sinks2 = sinks.clone();
                            let wk = ws_key.clone();
                            tokio::spawn(async move {
                                let mut stream = stream;
                                while let Some(msg) = stream.next().await {
                                    match msg {
                                        Ok(Message::Text(text)) => { let _ = tx.send((wk.clone(), text.to_string())); }
                                        Ok(Message::Ping(_)) => { let _ = tx.send((wk.clone(), r#"{"op":"ping"}"#.to_string())); }
                                        Ok(Message::Close(_)) | Err(_) => {
                                            let _ = tx.send((wk.clone(), format!("{{\"_internal\":\"disconnected\",\"wsKey\":\"{}\"}}", wk)));
                                            sinks2.lock().await.remove(&wk);
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            });

                            let _ = event_tx.send((ws_key.clone(), format!("{{\"_internal\":\"reconnected\",\"wsKey\":\"{}\"}}", ws_key)));
                            backoff_ms = 500;
                            break;
                        }
                        Err(_) => { backoff_ms = (backoff_ms * 2).min(MAX_BACKOFF); }
                    }
                }
            }
        });
    }

    async fn authenticate(&self, ws_key: &str) -> ClientResult<()> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| ClientError::AuthError("API key required for private WS".into()))?;
        let api_secret = self.config.api_secret.as_ref()
            .ok_or_else(|| ClientError::AuthError("API secret required for private WS".into()))?;
        let expires = get_timestamp_ms() + self.config.recv_window;
        let signature = sign_hmac_sha256(api_secret, &format!("GET/realtime{}", expires))
            .map_err(|e| ClientError::AuthError(e))?;
        let msg = serde_json::json!({"op":"auth","args":[api_key,expires,signature],"req_id":format!("{}-auth",ws_key)});
        self.try_ws_send(ws_key, &msg.to_string())
    }

    fn send_subscribe(&self, ws_key: &str, sub_topics: &[String]) -> ClientResult<()> {
        let msg = serde_json::json!({"op":"subscribe","args":sub_topics,"req_id":sub_topics.join(",")});
        self.try_ws_send(ws_key, &msg.to_string())
    }

    /// Subscribe to topics — tracked for resubscription on reconnect
    pub async fn subscribe(&self, new_topics: Vec<String>) -> ClientResult<()> {
        if new_topics.is_empty() { return Ok(()); }

        // Find the connection to subscribe on
        let ws_key = {
            let s = self.sinks.lock().await;
            match s.keys().next() { Some(k) => k.clone(), None => return Ok(()) }
        };

        // Track topics (survives disconnect)
        {
            let mut t = self.topics.lock().await;
            let entry = t.entry(ws_key.clone()).or_default();
            for topic in &new_topics {
                if !entry.contains(topic) { entry.push(topic.clone()); }
            }
        }

        self.send_subscribe(&ws_key, &new_topics)
    }

    pub fn try_ws_send(&self, ws_key: &str, msg: &str) -> ClientResult<()> {
        let sinks = self.sinks.clone();
        let key = ws_key.to_string();
        let msg = msg.to_string();
        tokio::spawn(async move {
            if let Some(sink) = sinks.lock().await.get_mut(&key) {
                let _ = sink.send(Message::Text(msg.into())).await;
            }
        });
        Ok(())
    }

    pub async fn send_ws_api_request(&self, ws_key: &str, operation: &str, params: serde_json::Value) -> ClientResult<serde_json::Value> {
        let req_id = format!("{}_{}", operation, get_timestamp_ms());
        let msg = serde_json::json!({"reqId":req_id,"op":operation,"header":{},"args":[params]});
        let mut sinks = self.sinks.lock().await;
        let sink = sinks.get_mut(ws_key)
            .ok_or_else(|| ClientError::WebSocketDisconnected { ws_key: ws_key.to_string() })?;
        sink.send(Message::Text(msg.to_string().into())).await
            .map_err(|e| ClientError::WebSocketError(format!("Send failed: {}", e)))?;
        Ok(serde_json::json!({"reqId": req_id}))
    }

    pub async fn recv(&self) -> Option<(String, String)> {
        self.event_rx.lock().await.recv().await
    }

    pub fn config(&self) -> &ClientConfig { &self.config }

    pub async fn shutdown(&self) {
        self.shutdown.notify_waiters();
        for (_, mut sink) in self.sinks.lock().await.drain() {
            let _ = sink.close().await;
        }
    }
}
