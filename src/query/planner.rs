
use crate::query::operators::{SimilarityOp, SimilarityOpHnsw};
use crate::vector::flat::FlatIndex;
use crate::vector::hnsw::HnswIndex;
use crate::semantic::pipeline::Embedder;
use crate::persona::Persona;
use crate::raci::RaciRole;

// Extremely simplified planner API for demo/testing
pub struct Planner<'a> {
    pub embedder: &'a dyn Embedder,
    pub persona: Option<&'a Persona>,
}

impl<'a> Planner<'a> {
    pub fn new(embedder: &'a dyn Embedder) -> Self { Self { embedder, persona: None } }
    pub fn with_persona(mut self, p: &'a Persona) -> Self { self.persona = Some(p); self }

    pub fn similar_flat(&self, index: &'a FlatIndex, text: &str, k: usize) -> Vec<(u64, f32)> {
        let op = SimilarityOp { index, embedder: self.embedder };
        let mut hits = op.topk(text, k * 2); // overfetch
        // persona shaping demo: if persona lacks R/A, drop results
        if let Some(p) = self.persona {
            if !(p.allows_role(RaciRole::R) || p.allows_role(RaciRole::A)) {
                hits.clear();
            }
        }
        hits.truncate(k);
        hits
    }

    pub fn similar_hnsw(&self, index: &'a HnswIndex, text: &str, k: usize) -> Vec<(u64, f32)> {
        let op = SimilarityOpHnsw { index, embedder: self.embedder };
        let mut hits = op.topk(text, k * 2);
        if let Some(p) = self.persona {
            if !(p.allows_role(RaciRole::R) || p.allows_role(RaciRole::A)) {
                hits.clear();
            }
        }
        hits.truncate(k);
        hits
    }
}
