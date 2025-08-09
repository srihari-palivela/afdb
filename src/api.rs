use axum::{routing::{post, get}, Router, Json, extract::State};
use tower_http::cors::{CorsLayer, Any};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::storage::Engine;
use crate::persona::Persona;
use crate::org::{OrgGraph, OrgUnit};
use roaring::RoaringBitmap;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Engine>,
    pub sessions: Arc<parking_lot::RwLock<std::collections::HashMap<String, Persona>>>,
    pub org: Arc<OrgGraph>,
    pub company: Arc<parking_lot::RwLock<Option<String>>>,
    pub contracts: Arc<parking_lot::RwLock<Vec<DataContract>>>,
    pub taxonomy: Arc<parking_lot::RwLock<Vec<String>>>,
    pub policies: Arc<parking_lot::RwLock<Vec<Policy>>>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Policy { pub name: String, pub effect: String, pub priority: i32 }

#[derive(Deserialize)]
pub struct UploadReq { pub manifest: IngestionManifest, pub artifacts: Vec<Artifact> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Artifact { pub id: String, pub text: String }

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    Router::new()
        .route("/ingest", post(upload))
        .route("/contracts", post(register_contract))
        .route("/contracts", get(list_contracts))
        .route("/assume_role", post(assume_role))
        .route("/semanticql", post(semanticql))
        .route("/onboarding", post(onboard))
        .route("/org/units", get(list_units))
        .route("/org/units/upsert", post(upsert_unit))
        .route("/taxonomy/paths", get(list_taxonomy))
        .route("/taxonomy/paths", post(add_taxonomy))
        .route("/policies", get(list_policies))
        .route("/policies", post(add_policy))
        .with_state(state)
        .layer(cors)
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

async fn list_contracts(State(st): State<AppState>) -> Json<Vec<DataContract>> {
    Json(st.contracts.read().clone())
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

#[derive(Deserialize)]
struct SemanticQlReq { ql: String }
#[derive(Serialize)]
struct SemanticQlResp { hits: Vec<(u64, f32)> }
async fn semanticql(State(st): State<AppState>, Json(req): Json<SemanticQlReq>) -> Json<SemanticQlResp> {
    if let Some(parsed) = crate::query::SemanticQl::parse(&req.ql) {
        let hits = st.engine.flat_index.read().cosine_topk(&st.engine.embedder.embed(&parsed.query), parsed.k);
        return Json(SemanticQlResp { hits });
    }
    Json(SemanticQlResp { hits: vec![] })
}

#[derive(Deserialize)]
struct OnboardReq { company: String }
async fn onboard(State(st): State<AppState>, Json(req): Json<OnboardReq>) -> Json<serde_json::Value> {
    *st.company.write() = Some(req.company);
    Json(serde_json::json!({"status":"ok"}))
}

async fn list_units(State(st): State<AppState>) -> Json<Vec<OrgUnit>> {
    let m = st.org.units.read();
    Json(m.values().cloned().collect())
}

#[derive(Deserialize)]
struct UpsertUnitReq { id: u32, name: String, parents: Vec<u32> }
async fn upsert_unit(State(st): State<AppState>, Json(req): Json<UpsertUnitReq>) -> Json<serde_json::Value> {
    st.org.upsert_unit(OrgUnit { id: req.id, name: req.name, parent_ids: req.parents });
    st.org.rebuild_closure();
    Json(serde_json::json!({"status":"ok"}))
}

async fn list_taxonomy(State(st): State<AppState>) -> Json<Vec<String>> {
    Json(st.taxonomy.read().clone())
}

#[derive(Deserialize)]
struct AddTaxReq { path: String }
async fn add_taxonomy(State(st): State<AppState>, Json(req): Json<AddTaxReq>) -> Json<serde_json::Value> {
    st.taxonomy.write().push(req.path);
    Json(serde_json::json!({"status":"ok"}))
}

async fn list_policies(State(st): State<AppState>) -> Json<Vec<Policy>> {
    Json(st.policies.read().clone())
}

#[derive(Deserialize)]
struct AddPolicyReq { name: String, effect: String, priority: i32 }
async fn add_policy(State(st): State<AppState>, Json(req): Json<AddPolicyReq>) -> Json<serde_json::Value> {
    st.policies.write().push(Policy { name: req.name, effect: req.effect, priority: req.priority });
    Json(serde_json::json!({"status":"ok"}))
}
