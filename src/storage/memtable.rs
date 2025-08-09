
use std::collections::BTreeMap;
use parking_lot::RwLock;
use crate::types::{RowKey, VersionedRow, Timestamp};
use crate::mvcc::visible_at;

#[derive(Default)]
pub struct MemTable {
    // key -> versions (newest last)
    inner: RwLock<BTreeMap<String, Vec<VersionedRow>>>,
}

impl MemTable {
    pub fn new() -> Self { Self { inner: RwLock::new(BTreeMap::new()) } }

    pub fn upsert(&self, v: VersionedRow) {
        let mut g = self.inner.write();
        let e = g.entry(v.row.key.0.clone()).or_default();
        e.push(v);
    }

    pub fn get_visible(&self, key: &RowKey, ts: Timestamp) -> Option<VersionedRow> {
        let g = self.inner.read();
        g.get(&key.0).and_then(|versions| {
            versions.iter().rev().find(|v| visible_at(v, ts)).cloned()
        })
    }

    pub fn scan_visible(&self, ts: Timestamp) -> Vec<VersionedRow> {
        let g = self.inner.read();
        g.values().flat_map(|vv| {
            vv.iter().rev().find(|v| visible_at(v, ts)).cloned()
        }).collect()
    }
}
