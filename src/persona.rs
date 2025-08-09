use crate::raci::{RaciMask, RaciRole};
use roaring::RoaringBitmap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Persona {
    pub person_id: String,
    pub assumed_roles: Vec<String>,
    pub org_scope: RoaringBitmap,
    pub raci_allowed: Vec<RaciRole>,
}

impl Persona {
    pub fn allows_role(&self, r: RaciRole) -> bool { self.raci_allowed.contains(&r) }
}
