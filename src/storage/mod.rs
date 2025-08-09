
pub mod wal;
pub mod memtable;
pub mod rowsegment;
pub mod columnsegment;
pub mod compactor;

use crate::types::{Row, VersionedRow, Timestamp, TxnId, Vector};
use crate::semantic::pipeline::Embedder;
use crate::vector::flat::FlatIndex;
use parking_lot::RwLock;

pub struct Engine {
    pub mem: memtable::MemTable,
    pub flat_index: RwLock<FlatIndex>,
    pub embedder: Box<dyn Embedder>,
    pub now: RwLock<Timestamp>,
}

impl Engine {
    pub fn new(embedder: Box<dyn Embedder>, dims: usize) -> Self {
        Self {
            mem: memtable::MemTable::new(),
            flat_index: RwLock::new(FlatIndex::new(dims)),
            embedder,
            now: RwLock::new(1),
        }
    }

    fn next_ts(&self) -> Timestamp {
        let mut g = self.now.write();
        *g += 1;
        *g
    }

    pub fn insert(&self, txn: TxnId, row: Row) {
        let ts = self.next_ts();
        let vrow = VersionedRow { begin_ts: ts, end_ts: None, txn_id: txn, row: row.clone() };
        self.mem.upsert(vrow);
        // in-kernel embedding + vector insert for a demo column: payload["text"]
        if let Some(text) = row.payload.get("text").and_then(|x| x.as_str()) {
            let vec: Vector = self.embedder.embed(text);
            self.flat_index.write().add(self.hash_key(&row.key.0), vec);
        }
    }

    fn hash_key(&self, k: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = ahash::AHasher::default();
        k.hash(&mut h);
        h.finish()
    }
}
