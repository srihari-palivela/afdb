
pub mod pipeline;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityLink {
    pub surface: String,
    pub entity_id: String,
    pub score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Kpi {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OlspOutput {
    pub entities: Vec<EntityLink>,
    pub kpis: Vec<Kpi>,
    pub summary: Option<Summary>,
    pub drift_flag: bool,
}

pub trait Olsp: Send + Sync {
    fn process(&self, text: &str) -> OlspOutput;
}

pub struct NoopOlsp;
impl Olsp for NoopOlsp {
    fn process(&self, _text: &str) -> OlspOutput { OlspOutput::default() }
}

// Simple OLSP based on heuristics for demo
pub struct HeuristicOlsp;
impl Olsp for HeuristicOlsp {
    fn process(&self, text: &str) -> OlspOutput {
        let mut out = OlspOutput::default();
        // trivial entity: detect words starting with capital letter
        for token in text.split_whitespace() {
            if token.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                out.entities.push(EntityLink { surface: token.to_string(), entity_id: token.to_lowercase(), score: 0.5 });
            }
        }
        out.summary = Some(Summary { text: format!("{}", text.chars().take(120).collect::<String>()) });
        out
    }
}
