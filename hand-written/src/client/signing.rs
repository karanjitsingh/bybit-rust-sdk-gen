use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Sign a message using HMAC-SHA256
pub fn sign_hmac_sha256(secret: &str, message: &str) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Invalid secret key: {}", e))?;
    
    mac.update(message.as_bytes());
    
    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());
    
    Ok(signature)
}

/// Get current timestamp in milliseconds
pub fn get_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Serialize parameters for signing according to Bybit V5 API spec
/// Parameters are NOT sorted — order is preserved as-is (matching upstream TS SDK)
pub fn serialize_params_for_signing(params: &serde_json::Value) -> String {
    if params.is_null() {
        return String::new();
    }
    let obj = match params.as_object() {
        Some(obj) => obj,
        None => return String::new(),
    };
    obj.iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| format!("{}={}", k, serialize_value(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Serialize a JSON value to string for signing
fn serialize_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => {
            // Arrays are serialized as JSON
            serde_json::to_string(arr).unwrap_or_default()
        }
        serde_json::Value::Object(_) => {
            // Objects are serialized as JSON
            serde_json::to_string(value).unwrap_or_default()
        }
    }
}

/// Build signature string for Bybit V5 API
/// Format: timestamp + api_key + recv_window + query_string (for GET)
/// Format: timestamp + api_key + recv_window + body (for POST)
pub fn build_sign_string(
    timestamp: u64,
    api_key: &str,
    recv_window: u64,
    params_str: &str,
) -> String {
    format!("{}{}{}{}", timestamp, api_key, recv_window, params_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sign_hmac_sha256() {
        let secret = "test_secret";
        let message = "test_message";
        let signature = sign_hmac_sha256(secret, message).unwrap();
        
        // Verify it's a valid hex string
        assert_eq!(signature.len(), 64); // SHA256 produces 32 bytes = 64 hex chars
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_serialize_params_simple() {
        let params = serde_json::json!({
            "symbol": "BTCUSDT",
            "side": "Buy",
            "qty": "0.001"
        });
        let serialized = serialize_params_for_signing(&params);
        // Order is insertion order (serde_json preserves it)
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("side=Buy"));
        assert!(serialized.contains("qty=0.001"));
    }
    
    #[test]
    fn test_serialize_params_with_array() {
        let params = serde_json::json!({
            "symbols": ["BTCUSDT", "ETHUSDT"],
            "category": "spot"
        });
        
        let serialized = serialize_params_for_signing(&params);
        assert!(serialized.contains("category=spot"));
        assert!(serialized.contains("symbols="));
    }
    
    #[test]
    fn test_build_sign_string() {
        let timestamp = 1234567890;
        let api_key = "test_key";
        let recv_window = 5000;
        let params = "symbol=BTCUSDT";
        
        let sign_string = build_sign_string(timestamp, api_key, recv_window, params);
        assert_eq!(sign_string, "1234567890test_key5000symbol=BTCUSDT");
    }
}


