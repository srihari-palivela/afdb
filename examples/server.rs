use axum::{routing::post, Router, Json};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use afdb::query::{SemanticQl};
use afdb::semantic::pipeline::{HttpEmbedder, ReasoningClient, Embedder};
use afdb::storage::Engine;
use afdb::Config;

#[derive(Deserialize)]
struct SimilarReq { ql: String }

#[derive(Serialize)]
struct SimilarResp { hits: Vec<(u64, f32)> }

#[tokio::main]
async fn main() {
    let cfg = Config::default();
    // For demo server, construct HttpEmbedder if possible, else dummy
    let embedder: Box<dyn Embedder> = match HttpEmbedder::new(cfg.embedding.clone().unwrap(), cfg.vector_dims) {
        Ok(e) => Box::new(e),
        Err(_) => Box::new(afdb::semantic::pipeline::DummyEmbedder::new("demo-mini", cfg.vector_dims)),
    };
    let engine = std::sync::Arc::new(Engine::new(embedder, cfg.vector_dims));

    // Preload minimal data for demo
    {
        let row = afdb::types::Row { key: afdb::types::RowKey("1".into()), payload: serde_json::json!({"text": "payment failed on renewal"}) };
        engine.insert(1, row);
        let row2 = afdb::types::Row { key: afdb::types::RowKey("2".into()), payload: serde_json::json!({"text": "refund processed successfully"}) };
        engine.insert(1, row2);
    }

    let app = Router::new().route("/semanticql", post({
        let engine = engine.clone();
        move |Json(req): Json<SimilarReq>| {
            let engine = engine.clone();
            async move {
                if let Some(parsed) = SemanticQl::parse(&req.ql) {
                    let hits = engine.flat_index.read().cosine_topk(&engine.embedder.embed(&parsed.query), parsed.k);
                    return Json(SimilarResp { hits });
                }
                Json(SimilarResp { hits: vec![] })
            }
        }
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
