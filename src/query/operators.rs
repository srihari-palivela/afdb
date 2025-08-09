
use crate::vector::flat::FlatIndex;
use crate::semantic::pipeline::Embedder;
use crate::types::{Vector};

pub struct SimilarityOp<'a> {
    pub index: &'a FlatIndex,
    pub embedder: &'a dyn Embedder,
}

impl<'a> SimilarityOp<'a> {
    pub fn topk(&self, text: &str, k: usize) -> Vec<(u64, f32)> {
        let v: Vector = self.embedder.embed(text);
        self.index.cosine_topk(&v, k)
    }
}
