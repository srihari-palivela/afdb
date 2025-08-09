
use serde::{Serialize, Deserialize};

pub type TxnId = u64;
pub type Timestamp = u64; // logical ts

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowKey(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    pub key: RowKey,
    pub payload: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionedRow {
    pub begin_ts: Timestamp,
    pub end_ts: Option<Timestamp>,
    pub txn_id: TxnId,
    pub row: Row,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vector(pub Vec<f32>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddingMeta {
    pub model_id: String,
    pub dims: usize,
    pub created_ts: Timestamp,
}
