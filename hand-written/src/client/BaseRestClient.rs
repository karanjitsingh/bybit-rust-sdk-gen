use crate::client::{ClientError, ClientResult};
use crate::client::config::ClientConfig;
use crate::client::signing::{build_sign_string, get_timestamp_ms, serialize_params_for_signing, sign_hmac_sha256};
use reqwest::{Client, Method};
use serde_json::Value;
use std::sync::{Arc, Mutex};

/// Base REST client implementation for Bybit API
/// Handles HTTP requests, signing, and error handling
pub struct BaseRestClient {
    config: ClientConfig,
    http_client: Client,
    time_offset: Arc<Mutex<i64>>,
}

impl BaseRestClient {
    /// Create a new Base REST client
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| ClientError::HttpError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config,
            http_client,
            time_offset: Arc::new(Mutex::new(0)),
        })
    }
    
    /// Get adjusted timestamp accounting for time drift
    fn get_adjusted_timestamp(&self) -> u64 {
        let offset = *self.time_offset.lock().unwrap();
        let timestamp = get_timestamp_ms();
        
        if offset >= 0 {
            timestamp + (offset as u64)
        } else {
            timestamp.saturating_sub((-offset) as u64)
        }
    }
    
    /// Sign a request with API credentials
    fn sign_request(
        &self,
        params: &Option<Value>,
        timestamp: u64,
    ) -> Result<String, ClientError> {
        let api_secret = self.config.api_secret.as_ref()
            .ok_or_else(|| ClientError::ApiError("API secret not configured".to_string()))?;
        
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| ClientError::ApiError("API key not configured".to_string()))?;
        
        // Serialize parameters
        let params_str = if let Some(p) = params {
            serialize_params_for_signing(p)
        } else {
            String::new()
        };
        
        // Build sign string: timestamp + api_key + recv_window + params
        let sign_string = build_sign_string(
            timestamp,
            api_key,
            self.config.recv_window,
            &params_str,
        );
        
        // Sign with HMAC-SHA256
        sign_hmac_sha256(api_secret, &sign_string)
            .map_err(|e| ClientError::ApiError(format!("Failed to sign request: {}", e)))
    }
    
    /// Make an HTTP request
    async fn _call(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<Value>,
        is_public: bool,
    ) -> ClientResult<Value> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        let http_method = Method::from_bytes(method.as_bytes())
            .map_err(|e| ClientError::HttpError(format!("Invalid HTTP method: {}", e)))?;
        
        let mut request = self.http_client.request(http_method.clone(), &url);
        
        // For private endpoints, add authentication headers
        if !is_public {
            let api_key = self.config.api_key.as_ref()
                .ok_or_else(|| ClientError::ApiError("API key not configured for private endpoint".to_string()))?;
            
            let timestamp = self.get_adjusted_timestamp();
            let signature = self.sign_request(&params, timestamp)?;
            
            request = request
                .header("X-BAPI-API-KEY", api_key)
                .header("X-BAPI-TIMESTAMP", timestamp.to_string())
                .header("X-BAPI-SIGN", signature)
                .header("X-BAPI-RECV-WINDOW", self.config.recv_window.to_string());
        }
        
        // Add parameters based on HTTP method
        if method == "GET" {
            if let Some(p) = params {
                // Convert JSON params to query parameters
                if let Some(obj) = p.as_object() {
                    let mut query_pairs: Vec<(String, String)> = Vec::new();
                    for (key, value) in obj {
                        let value_str = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            Value::Null => continue,
                            _ => serde_json::to_string(value)
                                .map_err(|e| ClientError::SerializationError(e.to_string()))?,
                        };
                        query_pairs.push((key.clone(), value_str));
                    }
                    request = request.query(&query_pairs);
                }
            }
        } else {
            // POST, PUT, DELETE use JSON body
            request = request.header("Content-Type", "application/json");
            if let Some(p) = params {
                request = request.json(&p);
            }
        }
        
        // Execute request
        let response = request
            .send()
            .await
            .map_err(|e| ClientError::HttpError(format!("Request failed: {}", e)))?;
        
        let status = response.status();
        let body_text = response
            .text()
            .await
            .map_err(|e| ClientError::HttpError(format!("Failed to read response body: {}", e)))?;
        
        // Parse JSON response
        let json: Value = serde_json::from_str(&body_text)
            .map_err(|e| ClientError::SerializationError(format!("Failed to parse JSON response: {}", e)))?;
        
        // Check for API errors
        if !status.is_success() {
            let error_msg = json.get("retMsg")
                .and_then(|v| v.as_str())
                .unwrap_or(&body_text);
            return Err(ClientError::ApiError(format!("API error ({}): {}", status, error_msg)));
        }
        
        // Check Bybit's retCode field
        if let Some(ret_code) = json.get("retCode").and_then(|v| v.as_i64()) {
            if ret_code != 0 {
                let error_msg = json.get("retMsg")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                return Err(ClientError::ApiError(format!("API error {}: {}", ret_code, error_msg)));
            }
        }
        
        // Return the result field if present, otherwise return the whole response
        Ok(json.get("result")
            .cloned()
            .unwrap_or(json))
    }

    /// Make a private GET request
    pub async fn get_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("GET", endpoint, params, false).await
    }
    
    /// Make a private POST request
    pub async fn post_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("POST", endpoint, params, false).await
    }
    
    /// Make a private DELETE request
    pub async fn delete_private(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("DELETE", endpoint, params, false).await
    }
    
    /// Make a public GET request
    pub async fn get(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("GET", endpoint, params, true).await
    }
    
    /// Make a public POST request
    pub async fn post(&self, endpoint: &str, params: Option<Value>) -> ClientResult<Value> {
        self._call("POST", endpoint, params, true).await
    }
}
