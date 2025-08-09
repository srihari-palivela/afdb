
use afdb::semantic::pipeline::{DummyEmbedder, Embedder};
use afdb::vector::flat::FlatIndex;
use afdb::vector::hnsw::HnswIndex;
use afdb::storage::Engine;
use afdb::types::{Row, RowKey};
use afdb::persona::Persona;
use afdb::raci::RaciRole;
use afdb::query::planner::Planner;

#[test]
fn flat_index_topk_basic() {
    let emb = DummyEmbedder::new("demo-mini", 32);
    let mut idx = FlatIndex::new(32);
    let items = vec![
        (1u64, "payment failed"),
        (2u64, "checkout declined"),
        (3u64, "refund issued"),
        (4u64, "plan renewed"),
    ];
    for (id, t) in &items {
        idx.add(*id, emb.embed(t));
    }
    let hits = idx.cosine_topk(&emb.embed("credit card failed"), 2);
    assert_eq!(hits.len(), 2);
    assert!(hits[0].1 >= hits[1].1);
}

#[test]
fn hnsw_index_returns_results() {
    let emb = DummyEmbedder::new("demo-mini", 32);
    let mut idx = HnswIndex::new(32, 8, 4);
    let items = vec![
        (1u64, "payment failed"),
        (2u64, "checkout declined"),
        (3u64, "refund issued"),
        (4u64, "plan renewed"),
    ];
    for (id, t) in &items {
        idx.add(*id, emb.embed(t));
    }
    let hits = idx.topk(&emb.embed("credit card failed"), 3);
    assert!(hits.len() > 0);
    // Scores should be within [-1,1]
    for (_, s) in hits {
        assert!(s >= -1.0 && s <= 1.0);
    }
}

#[test]
fn engine_insert_embeds_and_indexes() {
    let emb = DummyEmbedder::new("demo-mini", 32);
    let eng = Engine::new(Box::new(emb), 32);
    let row = Row { key: RowKey("r1".to_string()), payload: serde_json::json!({"text": "payment failed"}) };
    eng.insert(1, row);
    // query via flat index directly
    let hits = eng.flat_index.read().cosine_topk(&eng.embedder.embed("credit card failed"), 1);
    assert_eq!(hits.len(), 1);
}

#[test]
fn persona_shaping_blocks_without_r_or_a() {
    let emb = DummyEmbedder::new("demo-mini", 32);
    let mut idx = FlatIndex::new(32);
    idx.add(1, emb.embed("payment failed"));
    let persona = Persona { person_id: "u1".into(), assumed_roles: vec![], org_scope: roaring::RoaringBitmap::new(), raci_allowed: vec![] };
    let planner = Planner::new(&emb).with_persona(&persona);
    let hits = planner.similar_flat(&idx, "credit card failed", 3);
    assert_eq!(hits.len(), 0);
    // Allow with R role
    let persona_r = Persona { person_id: "u1".into(), assumed_roles: vec![], org_scope: roaring::RoaringBitmap::new(), raci_allowed: vec![RaciRole::R] };
    let planner_r = Planner::new(&emb).with_persona(&persona_r);
    let hits_r = planner_r.similar_flat(&idx, "credit card failed", 3);
    assert!(hits_r.len() > 0);
}
