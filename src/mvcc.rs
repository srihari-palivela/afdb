
use crate::types::{VersionedRow, Timestamp};

pub fn visible_at(v: &VersionedRow, read_ts: Timestamp) -> bool {
    v.begin_ts <= read_ts && v.end_ts.map(|e| read_ts < e).unwrap_or(true)
}
