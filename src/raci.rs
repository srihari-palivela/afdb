use serde::{Serialize, Deserialize};
use roaring::RoaringBitmap;
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RaciRole { R, A, C, I }

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RaciMask { pub roles: HashSet<RaciRole> }

impl RaciMask { pub fn allows(&self, r: RaciRole) -> bool { self.roles.contains(&r) } }

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RaciFunction { pub org_unit_id: u32, pub roles: HashMap<String, RaciMask> }

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RaciArtifact { pub artifact_id: String, pub mask: RaciMask }

#[derive(Default)]
pub struct RaciStore {
    pub functions: RwLock<HashMap<u32, RaciFunction>>, // by org unit
    pub artifacts: RwLock<HashMap<String, RaciArtifact>>, // by artifact id
}

impl RaciStore {
    pub fn resolve_for_artifact(&self, artifact_id: &str, org_chain: &[u32]) -> RaciMask {
        if let Some(a) = self.artifacts.read().get(artifact_id) { return a.mask.clone(); }
        for &ou in org_chain { // nearest ancestor first
            if let Some(fun) = self.functions.read().get(&ou) {
                // naive merge: any role present grants
                let mut mask = RaciMask { roles: HashSet::new() };
                for m in fun.roles.values() { mask.roles.extend(m.roles.iter().copied()); }
                return mask;
            }
        }
        RaciMask::default()
    }
}
