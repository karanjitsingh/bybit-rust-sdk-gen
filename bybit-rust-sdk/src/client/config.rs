use std::time::Duration;

/// Configuration for the Bybit HTTP client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authenticated requests
    pub api_key: Option<String>,
    
    /// API secret for signing requests
    pub api_secret: Option<String>,
    
    /// Base URL for API requests (mainnet or testnet)
    pub base_url: String,
    
    /// Receive window for request timestamp validation (milliseconds)
    pub recv_window: u64,
    
    /// Request timeout
    pub timeout: Duration,
    
    /// Enable time synchronization with server
    pub enable_time_sync: bool,
    
    /// How often to sync time drift (if enabled)
    pub sync_interval: Duration,
    
    /// User agent string
    pub user_agent: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            base_url: "https://api.bybit.com".to_string(),
            recv_window: 5000,
            timeout: Duration::from_secs(30),
            enable_time_sync: false,
            sync_interval: Duration::from_secs(3600),
            user_agent: "bybit-rust-sdk/0.1.0".to_string(),
        }
    }
}

impl ClientConfig {
    /// Create a new config builder
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
    
    /// Create a config for testnet
    pub fn testnet() -> Self {
        Self {
            base_url: "https://api-testnet.bybit.com".to_string(),
            ..Default::default()
        }
    }
    
    /// Create a config for mainnet
    pub fn mainnet() -> Self {
        Self::default()
    }
}

/// Builder for ClientConfig
#[derive(Debug, Clone, Default)]
pub struct ClientConfigBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
    base_url: Option<String>,
    recv_window: Option<u64>,
    timeout: Option<Duration>,
    enable_time_sync: Option<bool>,
    sync_interval: Option<Duration>,
    user_agent: Option<String>,
}

impl ClientConfigBuilder {
    /// Set the API key
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    /// Set the API secret
    pub fn api_secret<S: Into<String>>(mut self, api_secret: S) -> Self {
        self.api_secret = Some(api_secret.into());
        self
    }
    
    /// Set the base URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    
    /// Use testnet
    pub fn testnet(mut self) -> Self {
        self.base_url = Some("https://api-testnet.bybit.com".to_string());
        self
    }
    
    /// Use mainnet (default)
    pub fn mainnet(mut self) -> Self {
        self.base_url = Some("https://api.bybit.com".to_string());
        self
    }
    
    /// Set the receive window (milliseconds)
    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
    
    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Enable time synchronization
    pub fn enable_time_sync(mut self, enable: bool) -> Self {
        self.enable_time_sync = Some(enable);
        self
    }
    
    /// Set sync interval
    pub fn sync_interval(mut self, interval: Duration) -> Self {
        self.sync_interval = Some(interval);
        self
    }
    
    /// Set user agent
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    /// Build the config
    pub fn build(self) -> ClientConfig {
        let default = ClientConfig::default();
        
        ClientConfig {
            api_key: self.api_key,
            api_secret: self.api_secret,
            base_url: self.base_url.unwrap_or(default.base_url),
            recv_window: self.recv_window.unwrap_or(default.recv_window),
            timeout: self.timeout.unwrap_or(default.timeout),
            enable_time_sync: self.enable_time_sync.unwrap_or(default.enable_time_sync),
            sync_interval: self.sync_interval.unwrap_or(default.sync_interval),
            user_agent: self.user_agent.unwrap_or(default.user_agent),
        }
    }
}


