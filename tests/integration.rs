
use afdb::semantic::pipeline::{DummyEmbedder, Embedder};
use afdb::vector::flat::FlatIndex;
use afdb::vector::hnsw::HnswIndex;

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
