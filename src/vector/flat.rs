
use crate::types::{Vector};
use std::cmp::Ordering;

pub struct FlatIndex {
    pub dims: usize,
    items: Vec<(u64, Vector)>, // id -> vector
}

impl FlatIndex {
    pub fn new(dims: usize) -> Self { Self { dims, items: Vec::new() } }

    pub fn add(&mut self, id: u64, v: Vector) {
        self.items.push((id, v));
    }

    pub fn cosine_topk(&self, q: &Vector, k: usize) -> Vec<(u64, f32)> {
        let mut scores: Vec<(u64, f32)> = self.items.iter()
            .map(|(id, v)| (*id, cosine(&q.0, &v.0))).collect();
        scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        scores.truncate(k);
        scores
    }
}

fn cosine(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for i in 0..a.len() {
        dot += a[i]*b[i];
        na += a[i]*a[i];
        nb += b[i]*b[i];
    }
    if na == 0.0 || nb == 0.0 { return 0.0; }
    dot / (na.sqrt()*nb.sqrt())
}
