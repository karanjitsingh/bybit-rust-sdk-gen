use crate::client::{BybitApiError, ClientError, ClientResult};
use crate::client::config::ClientConfig;
use crate::client::signing::{build_sign_string, get_timestamp_ms, serialize_params_for_signing, sign_hmac_sha256};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Number {
    F64(f64),
    U64(u64),
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::F64(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::U64(value)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::F64(value) => write!(f, "{}", value),
            Number::U64(value) => write!(f, "{}", value),
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::U64(0)
    }
}

pub struct BaseRestClient {
    config: ClientConfig,
    http_client: Client,
    time_offset: Arc<Mutex<i64>>,
}

impl BaseRestClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("x-referer", reqwest::header::HeaderValue::from_static("bybitapirust"));
                h
            })
            .build()
            .map_err(|e| ClientError::HttpError(e.to_string()))?;

        let client = Self {
            config,
            http_client,
            time_offset: Arc::new(Mutex::new(0)),
        };

        Ok(client)
    }

    /// Sync time with server. Call this before first private request or periodically.
    pub async fn sync_time(&self) -> ClientResult<i64> {
        let offset = self.fetch_time_offset().await?;
        *self.time_offset.lock().await = offset;
        Ok(offset)
    }

    /// Start periodic time sync in background (returns join handle)
    pub fn start_time_sync(&self) -> tokio::task::JoinHandle<()> {
        let offset = self.time_offset.clone();
        let client = self.http_client.clone();
        let base_url = self.config.base_url.clone();
        let interval = self.config.sync_interval;
        tokio::spawn(async move {
            loop {
                if let Ok(server_offset) = Self::fetch_time_offset_inner(&client, &base_url).await {
                    *offset.lock().await = server_offset;
                }
                tokio::time::sleep(interval).await;
            }
        })
    }

    /// Estimate clock drift: server_time_ms - local_time_ms (adjusted for latency)
    async fn fetch_time_offset(&self) -> ClientResult<i64> {
        Self::fetch_time_offset_inner(&self.http_client, &self.config.base_url).await
    }

    async fn fetch_time_offset_inner(client: &Client, base_url: &str) -> ClientResult<i64> {
        let start = get_timestamp_ms();
        let url = format!("{}/v5/market/time", base_url);
        let resp = client.get(&url).send().await
            .map_err(|e| ClientError::TimeSyncError(e.to_string()))?;
        let json: Value = resp.json().await
            .map_err(|e| ClientError::TimeSyncError(e.to_string()))?;
        let end = get_timestamp_ms();

        let server_time_str = json.get("result")
            .and_then(|r| r.get("timeSecond"))
            .and_then(|v| v.as_str())
            .or_else(|| json.get("result").and_then(|r| r.get("timeNano")).and_then(|v| v.as_str()));

        let server_time_ms = match server_time_str {
            Some(s) => {
                let f: f64 = s.parse().map_err(|_| ClientError::TimeSyncError("Bad server time".into()))?;
                if f > 1e15 { (f / 1e6) as i64 } // nanos
                else { (f * 1000.0) as i64 } // seconds
            }
            None => return Err(ClientError::TimeSyncError("No time in response".into())),
        };

        let avg_latency = ((end - start) / 2) as i64;
        Ok(server_time_ms - end as i64 + avg_latency)
    }

    fn get_adjusted_timestamp(&self, offset: i64) -> u64 {
        let ts = get_timestamp_ms() as i64 + offset;
        ts.max(0) as u64
    }

    fn sign_request(&self, method: &str, params: &Option<Value>, timestamp: u64) -> Result<String, ClientError> {
        let api_secret = self.config.api_secret.as_ref()
            .ok_or_else(|| ClientError::AuthError("API secret not configured".into()))?;
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| ClientError::AuthError("API key not configured".into()))?;

        let params_str = match params {
            Some(p) if method == "GET" => serialize_params_for_signing(p),
            Some(p) => serde_json::to_string(p).map_err(|e| ClientError::SerializationError(e.to_string()))?,
            None => String::new(),
        };

        let sign_string = build_sign_string(timestamp, api_key, self.config.recv_window, &params_str);
        sign_hmac_sha256(api_secret, &sign_string).map_err(|e| ClientError::AuthError(e))
    }

    async fn _call(&self, method: &str, endpoint: &str, params: Option<Value>, is_public: bool) -> ClientResult<Value> {
        // Auto-sync time before first private request if offset is 0 and time sync enabled
        if !is_public && self.config.enable_time_sync {
            let offset = *self.time_offset.lock().await;
            if offset == 0 {
                let _ = self.sync_time().await;
            }
        }

        let url = format!("{}{}", self.config.base_url, endpoint);
        let http_method = Method::from_bytes(method.as_bytes())
            .map_err(|e| ClientError::HttpError(e.to_string()))?;

        let mut request = self.http_client.request(http_method, &url);

        if !is_public {
            let api_key = self.config.api_key.as_ref()
                .ok_or_else(|| ClientError::AuthError("API key not configured".into()))?;
            let offset = *self.time_offset.lock().await;
            let timestamp = self.get_adjusted_timestamp(offset);
            let signature = self.sign_request(method, &params, timestamp)?;

            request = request
                .header("X-BAPI-API-KEY", api_key)
                .header("X-BAPI-TIMESTAMP", timestamp.to_string())
                .header("X-BAPI-SIGN", signature)
                .header("X-BAPI-RECV-WINDOW", self.config.recv_window.to_string());
        }

        if method == "GET" {
            if let Some(ref p) = params {
                if let Some(obj) = p.as_object() {
                    let pairs: Vec<(String, String)> = obj.iter()
                        .filter(|(_, v)| !v.is_null())
                        .map(|(k, v)| (k.clone(), match v {
                            Value::String(s) => s.clone(),
                            Value::Null => String::new(),
                            other => other.to_string().trim_matches('"').to_string(),
                        }))
                        .collect();
                    request = request.query(&pairs);
                }
            }
        } else {
            request = request.header("Content-Type", "application/json");
            if let Some(ref p) = params {
                request = request.json(p);
            }
        }

        let response = request.send().await
            .map_err(|e| ClientError::HttpError(e.to_string()))?;

        let status = response.status();
        let body_text = response.text().await
            .map_err(|e| ClientError::HttpError(e.to_string()))?;

        let json: Value = serde_json::from_str(&body_text)
            .map_err(|_| ClientError::SerializationError(body_text.clone()))?;

        if !status.is_success() {
            return Err(ClientError::HttpError(format!("HTTP {}: {}", status, body_text)));
        }

        // Check Bybit retCode
        if let Some(ret_code) = json.get("retCode").and_then(|v| v.as_i64()) {
            if ret_code != 0 {
                let ret_msg = json.get("retMsg").and_then(|v| v.as_str()).unwrap_or("Unknown error").to_string();
                return Err(ClientError::Api(BybitApiError {
                    ret_code,
                    ret_msg,
                    raw_response: Some(json),
                }));
            }
        }

        Ok(json.get("result").cloned().unwrap_or(json))
    }

    pub async fn get_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("GET", endpoint, params, false).await
    }

    pub async fn post_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("POST", endpoint, params, false).await
    }

    pub async fn delete_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("DELETE", endpoint, params, false).await
    }

    pub async fn get(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("GET", endpoint, params, true).await
    }

    pub async fn post(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("POST", endpoint, params, true).await
    }
}
