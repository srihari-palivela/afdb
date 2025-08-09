use axum::{routing::{post, get}, Router, Json, extract::State};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::storage::Engine;
use crate::persona::Persona;
use roaring::RoaringBitmap;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Engine>,
    pub sessions: Arc<parking_lot::RwLock<std::collections::HashMap<String, Persona>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestionManifest {
    pub source_app: String,
    pub org_unit_hint: Option<String>,
    pub taxonomy_path: Option<String>,
    pub owner_role: Option<String>,
    pub raci_override: Option<serde_json::Value>,
    pub legal_basis: Option<String>,
    pub retention_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataContract {
    pub producer: String,
    pub schema_hash: String,
    pub pii_fields: Vec<String>,
}

#[derive(Deserialize)]
pub struct UploadReq { pub manifest: IngestionManifest, pub artifacts: Vec<Artifact> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artifact { pub id: String, pub text: String }

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/ingest", post(upload))
        .route("/contracts", post(register_contract))
        .route("/assume_role", post(assume_role))
        .with_state(state)
}

async fn upload(State(st): State<AppState>, Json(req): Json<UploadReq>) -> Json<serde_json::Value> {
    // TODO: validate against DataContract registry (omitted)
    for a in req.artifacts.iter() {
        let row = crate::types::Row { key: crate::types::RowKey(a.id.clone()), payload: serde_json::json!({"text": a.text}) };
        st.engine.insert(1, row);
    }
    Json(serde_json::json!({"status": "ok", "ingested": req.artifacts.len()}))
}

#[derive(Deserialize)]
struct ContractReq { contract: DataContract }
async fn register_contract(Json(_req): Json<ContractReq>) -> Json<serde_json::Value> {
    // For now just ack
    Json(serde_json::json!({"status": "registered"}))
}

#[derive(Deserialize)]
struct AssumeReq { person_id: String, roles: Vec<String>, scope_ids: Vec<u32> }
#[derive(Serialize)]
struct AssumeResp { session_id: String }

async fn assume_role(State(st): State<AppState>, Json(req): Json<AssumeReq>) -> Json<AssumeResp> {
    let mut scope = RoaringBitmap::new();
    for id in req.scope_ids { scope.insert(id); }
    let persona = Persona { person_id: req.person_id, assumed_roles: req.roles, org_scope: scope, raci_allowed: vec![crate::raci::RaciRole::R] };
    let sid = Uuid::new_v4().to_string();
    st.sessions.write().insert(sid.clone(), persona);
    Json(AssumeResp { session_id: sid })
}
