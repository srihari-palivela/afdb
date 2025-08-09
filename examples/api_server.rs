use axum::{Router, routing::get};
use afdb::{api, Config};
use afdb::semantic::pipeline::{HttpEmbedder, DummyEmbedder, Embedder};
use afdb::storage::Engine;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cfg = Config::default();
    let embedder: Box<dyn Embedder> = match HttpEmbedder::new(cfg.embedding.clone().unwrap(), cfg.vector_dims) {
        Ok(e) => Box::new(e),
        Err(_) => Box::new(DummyEmbedder::new("demo-mini", cfg.vector_dims)),
    };
    let engine = Arc::new(Engine::new(embedder, cfg.vector_dims));
    let state = api::AppState { engine: engine.clone(), sessions: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())) };
    let app = api::router(state).route("/healthz", get(|| async { "ok" }));
    let addr = std::net::SocketAddr::from(([127,0,0,1], 8090));
    println!("API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
