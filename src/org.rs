use serde::{Serialize, Deserialize};
use roaring::RoaringBitmap;
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrgUnit { pub id: u32, pub name: String, pub parent_ids: Vec<u32> }

#[derive(Default)]
pub struct OrgGraph {
    pub units: RwLock<HashMap<u32, OrgUnit>>,
    pub closure: RwLock<HashSet<(u32,u32,u32)>>, // (ancestor, descendant, depth)
    pub scopes: RwLock<HashMap<u32, RoaringBitmap>>, // unit -> descendants bitmap
}

impl OrgGraph {
    pub fn new() -> Self { Self::default() }

    pub fn upsert_unit(&self, unit: OrgUnit) {
        self.units.write().insert(unit.id, unit);
    }

    pub fn rebuild_closure(&self) {
        let units = self.units.read().clone();
        let mut closure = HashSet::new();
        for (&id, u) in &units {
            // self
            closure.insert((id, id, 0));
            // BFS ancestors
            let mut stack = u.parent_ids.clone();
            let mut depth = 1u32;
            let mut visited = HashSet::new();
            while let Some(p) = stack.pop() {
                if !visited.insert((p, depth)) { continue; }
                closure.insert((p, id, depth));
                if let Some(pu) = units.get(&p) {
                    for pp in &pu.parent_ids { stack.push(*pp); }
                }
                depth = depth.saturating_add(1);
            }
        }
        *self.closure.write() = closure;
        self.rebuild_scopes();
    }

    fn rebuild_scopes(&self) {
        let closure = self.closure.read();
        let mut scopes: HashMap<u32, RoaringBitmap> = HashMap::new();
        for (anc, desc, _d) in closure.iter().cloned() {
            scopes.entry(anc).or_default().insert(desc);
        }
        *self.scopes.write() = scopes;
    }

    pub fn scope_bitmap(&self, unit_id: u32) -> RoaringBitmap {
        self.scopes.read().get(&unit_id).cloned().unwrap_or_default()
    }
}
