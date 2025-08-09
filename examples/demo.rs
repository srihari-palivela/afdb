
use afdb::semantic::pipeline::{DummyEmbedder, Embedder};
use afdb::vector::flat::FlatIndex;
use afdb::vector::hnsw::HnswIndex;
use afdb::types::{Vector};

fn main() {
    // 1) Prepare an embedder
    let emb = DummyEmbedder::new("demo-mini", 64);

    // 2) Build a flat index and insert a few vectors
    let mut flat = FlatIndex::new(64);
    let texts = vec![
        (1u64, "payment failed on renewal"),
        (2u64, "card declined at checkout"),
        (3u64, "refund processed successfully"),
        (4u64, "subscription renewed for annual plan"),
    ];
    for (id, t) in &texts {
        flat.add(*id, emb.embed(t));
    }

    // 3) Run a top-K query with the flat index
    let q = "credit card failed during payment";
    let flat_hits = flat.cosine_topk(&emb.embed(q), 3);
    println!("FlatIndex hits: {:?}", flat_hits);

    // 4) Same corpus in a tiny HNSW index
    let mut hnsw = HnswIndex::new(64, 8, 4);
    for (id, t) in &texts {
        hnsw.add(*id, emb.embed(t));
    }
    let h_hits = hnsw.topk(&emb.embed(q), 3);
    println!("HNSW hits: {:?}", h_hits);
}
