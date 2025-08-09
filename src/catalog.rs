
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Default)]
pub struct Catalog {
    // table -> columns (very simplified for MVP)
    pub tables: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl Catalog {
    pub fn new() -> Self { Self::default() }
    pub fn create_table(&self, name: &str, cols: Vec<String>) {
        self.tables.write().insert(name.to_string(), cols);
    }
}
