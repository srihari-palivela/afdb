
use crate::types::Vector;
use rand::Rng;
use crate::config::ModelEndpointConfig;

#[derive(thiserror::Error, Debug)]
pub enum EmbedError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("invalid response")]
    InvalidResponse,
}

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

// Blocking HTTP embedder using a simple JSON API
// Expected request: { "model": string, "input": string }
// Expected response: { "embedding": [f32, ...] }
pub struct HttpEmbedder {
    cfg: ModelEndpointConfig,
    client: reqwest::blocking::Client,
    dims_: usize,
}

impl HttpEmbedder {
    pub fn new(cfg: ModelEndpointConfig, dims: usize) -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(cfg.timeout_ms))
            .build()?;
        Ok(Self { cfg, client, dims_: dims })
    }

    fn url(&self) -> String {
        format!("{}{}", self.cfg.base_url.trim_end_matches('/'), self.cfg.path.as_str())
    }
}

impl Embedder for HttpEmbedder {
    fn model_id(&self) -> &str { &self.cfg.model }
    fn dims(&self) -> usize { self.dims_ }
    fn embed(&self, text: &str) -> Vector {
        let mut req = self.client.post(self.url())
            .json(&serde_json::json!({
                "model": self.cfg.model,
                "input": text,
            }));
        if let Some(h) = &self.cfg.auth_header {
            if let Some(k) = &self.cfg.api_key { req = req.header(h, k); }
        }
        for (k,v) in &self.cfg.headers { req = req.header(k, v); }
        let resp = req.send();
        match resp.and_then(|r| r.error_for_status())
                  .and_then(|mut r| r.json::<serde_json::Value>()) {
            Ok(val) => {
                if let Some(arr) = val.get("embedding").and_then(|e| e.as_array()) {
                    let mut v = Vec::with_capacity(self.dims_);
                    for i in 0..self.dims_ { v.push(arr.get(i).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32); }
                    return Vector(v);
                }
                // fallback if server returns { data: { embedding: [...] } }
                if let Some(arr) = val.pointer("/data/embedding").and_then(|e| e.as_array()) {
                    let mut v = Vec::with_capacity(self.dims_);
                    for i in 0..self.dims_ { v.push(arr.get(i).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32); }
                    return Vector(v);
                }
                Vector(vec![0.0; self.dims_])
            }
            Err(_) => Vector(vec![0.0; self.dims_]),
        }
    }
}

// Reasoning client to call an LLM-like endpoint
// Expected request: { "model": string, "prompt": string, "context": any }
// Expected response: { "output": string }
pub struct ReasoningClient {
    cfg: ModelEndpointConfig,
    client: reqwest::blocking::Client,
}

impl ReasoningClient {
    pub fn new(cfg: ModelEndpointConfig) -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(cfg.timeout_ms))
            .build()?;
        Ok(Self { cfg, client })
    }

    fn url(&self) -> String {
        format!("{}{}", self.cfg.base_url.trim_end_matches('/'), self.cfg.path.as_str())
    }

    pub fn complete(&self, prompt: &str, context: serde_json::Value) -> anyhow::Result<String> {
        let mut req = self.client.post(self.url())
            .json(&serde_json::json!({
                "model": self.cfg.model,
                "prompt": prompt,
                "context": context,
            }));
        if let Some(h) = &self.cfg.auth_header {
            if let Some(k) = &self.cfg.api_key { req = req.header(h, k); }
        }
        for (k,v) in &self.cfg.headers { req = req.header(k, v); }

        let resp = req.send()?.error_for_status()?;
        let val: serde_json::Value = resp.json()?;
        let out = val.get("output").and_then(|x| x.as_str()).unwrap_or("").to_string();
        Ok(out)
    }
}
