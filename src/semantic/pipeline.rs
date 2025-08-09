
use crate::types::{Vector};
use rand::Rng;

pub trait Embedder: Send + Sync {
    fn model_id(&self) -> &str;
    fn dims(&self) -> usize;
    fn embed(&self, text: &str) -> Vector;
}

// Dummy CPU embedder for MVP
pub struct DummyEmbedder {
    model: String,
    dims_: usize,
}

impl DummyEmbedder {
    pub fn new(model: &str, dims: usize) -> Self {
        Self { model: model.to_string(), dims_: dims }
    }
}

impl Embedder for DummyEmbedder {
    fn model_id(&self) -> &str { &self.model }
    fn dims(&self) -> usize { self.dims_ }
    fn embed(&self, text: &str) -> Vector {
        let mut rng = rand::thread_rng();
        let mut v = Vec::with_capacity(self.dims_);
        // deterministic-ish seed by length
        let bias = text.len() as f32;
        for _ in 0..self.dims_ { v.push(rng.gen::<f32>() + bias.fract()); }
        Vector(v)
    }
}
