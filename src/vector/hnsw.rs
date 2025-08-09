
use crate::types::Vector;
use rand::{Rng, seq::SliceRandom};
use smallvec::SmallVec;

// A minimal, educational HNSW-like graph for approximate search.
// Not production-ready; good for demos and API shape.

#[derive(Clone)]
struct Node {
    id: u64,
    vec: Vector,
    neighbors: SmallVec<[usize; 16]>, // indices into nodes
    level: i32,
}

pub struct HnswIndex {
    pub dims: usize,
    pub m: usize,          // max neighbors per level
    pub ef: usize,         // search breadth
    entry: Option<usize>,  // entry point index
    nodes: Vec<Node>,
}

impl HnswIndex {
    pub fn new(dims: usize, m: usize, ef: usize) -> Self {
        Self { dims, m, ef, entry: None, nodes: Vec::new() }
    }

    pub fn len(&self) -> usize { self.nodes.len() }

    pub fn add(&mut self, id: u64, v: Vector) {
        let level = self.sample_level();
        let node_idx = self.nodes.len();
        let node = Node { id, vec: v, neighbors: SmallVec::new(), level };
        self.nodes.push(node);

        if self.entry.is_none() {
            self.entry = Some(node_idx);
            return;
        }

        // greedy search from entry
        let mut cur = self.entry.unwrap();
        let mut improved = true;
        while improved {
            improved = false;
            let (best_idx, best_sim) = (cur, cosine(&self.nodes[cur].vec.0, &self.nodes[node_idx].vec.0));
            for &n in &self.nodes[cur].neighbors {
                let s = cosine(&self.nodes[n].vec.0, &self.nodes[node_idx].vec.0);
                if s > best_sim {
                    cur = n;
                    improved = true;
                }
            }
        }

        // connect new node to neighbors of cur
        let mut cand: Vec<(usize, f32)> = self.nodes[cur].neighbors.iter()
            .map(|&n| (n, cosine(&self.nodes[n].vec.0, &self.nodes[node_idx].vec.0)))
            .collect();
        cand.push((cur, cosine(&self.nodes[cur].vec.0, &self.nodes[node_idx].vec.0)));
        cand.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        cand.truncate(self.m);

        for (nidx, _) in cand {
            self.nodes[node_idx].neighbors.push(nidx);
            self.nodes[nidx].neighbors.push(node_idx);
            if self.nodes[nidx].neighbors.len() > self.m {
                // drop worst neighbor to keep degree bounded
                let mut scored: Vec<(usize, f32)> = self.nodes[nidx].neighbors.iter()
                    .map(|&x| (x, cosine(&self.nodes[nidx].vec.0, &self.nodes[x].vec.0))).collect();
                scored.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
                scored.truncate(self.m);
                self.nodes[nidx].neighbors = scored.into_iter().map(|(x,_)| x).collect();
            }
        }
    }

    pub fn topk(&self, q: &Vector, k: usize) -> Vec<(u64, f32)> {
        if self.nodes.is_empty() { return vec![]; }
        let mut rng = rand::thread_rng();
        let start = self.entry.unwrap_or(0);
        // beam search
        let mut frontier = vec![start];
        let mut visited = ahash::AHashSet::<usize>::default();
        let mut best: Vec<(usize, f32)> = Vec::new();

        while !frontier.is_empty() {
            let cur = frontier.pop().unwrap();
            if !visited.insert(cur) { continue; }

            let s = cosine(&self.nodes[cur].vec.0, &q.0);
            best.push((cur, s));
            // push neighbors (shuffle to avoid bias)
            let mut neigh = self.nodes[cur].neighbors.clone().into_vec();
            neigh.shuffle(&mut rng);
            for n in neigh.into_iter().take(self.ef) {
                if !visited.contains(&n) { frontier.push(n); }
            }
        }
        best.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        best.truncate(k);
        best.into_iter().map(|(idx, sc)| (self.nodes[idx].id, sc)).collect()
    }

    fn sample_level(&self) -> i32 {
        // very rough geometric level sampler
        let mut r = rand::thread_rng();
        let mut level = 0;
        while r.gen::<f32>() < 0.5 { level += 1; if level > 5 { break; } }
        level
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
