
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelEndpointConfig {
    pub base_url: String,
    #[serde(default)]
    pub path: String,
    pub model: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub auth_header: Option<String>, // e.g., "Authorization"
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_timeout_ms() -> u64 { 30_000 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub data_dir: String,
    pub wal_dir: String,
    pub segment_size_mb: usize,
    pub vector_dims: usize,
    #[serde(default)]
    pub embedding: Option<ModelEndpointConfig>,
    #[serde(default)]
    pub reasoning: Option<ModelEndpointConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: "data".to_string(),
            wal_dir: "wal".to_string(),
            segment_size_mb: 128,
            vector_dims: 384,
            embedding: Some(ModelEndpointConfig {
                base_url: "http://localhost:8080".to_string(),
                path: "/embed".to_string(),
                model: "demo-embedding".to_string(),
                api_key: None,
                auth_header: Some("Authorization".to_string()),
                headers: Vec::new(),
                timeout_ms: default_timeout_ms(),
            }),
            reasoning: Some(ModelEndpointConfig {
                base_url: "http://localhost:8080".to_string(),
                path: "/reason".to_string(),
                model: "demo-reasoner".to_string(),
                api_key: None,
                auth_header: Some("Authorization".to_string()),
                headers: Vec::new(),
                timeout_ms: default_timeout_ms(),
            }),
        }
    }
}
