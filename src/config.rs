
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub data_dir: String,
    pub wal_dir: String,
    pub segment_size_mb: usize,
    pub vector_dims: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: "data".to_string(),
            wal_dir: "wal".to_string(),
            segment_size_mb: 128,
            vector_dims: 384,
        }
    }
}
