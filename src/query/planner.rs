
use crate::query::operators::{SimilarityOp, SimilarityOpHnsw};
use crate::vector::flat::FlatIndex;
use crate::vector::hnsw::HnswIndex;
use crate::semantic::pipeline::Embedder;

// Extremely simplified planner API for demo/testing
pub struct Planner<'a> {
    pub embedder: &'a dyn Embedder,
}

impl<'a> Planner<'a> {
    pub fn new(embedder: &'a dyn Embedder) -> Self { Self { embedder } }

    pub fn similar_flat(&self, index: &'a FlatIndex, text: &str, k: usize) -> Vec<(u64, f32)> {
        let op = SimilarityOp { index, embedder: self.embedder };
        op.topk(text, k)
    }

    pub fn similar_hnsw(&self, index: &'a HnswIndex, text: &str, k: usize) -> Vec<(u64, f32)> {
        let op = SimilarityOpHnsw { index, embedder: self.embedder };
        op.topk(text, k)
    }
}
