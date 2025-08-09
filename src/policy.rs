use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PolicyEffect { Allow, Deny, Mask, AggregateOnly }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyDecision { pub effect: PolicyEffect }

pub trait PolicyEngine: Send + Sync {
    fn evaluate(&self, input: serde_json::Value) -> PolicyDecision;
}

pub struct AllowAll;
impl PolicyEngine for AllowAll {
    fn evaluate(&self, _input: serde_json::Value) -> PolicyDecision {
        PolicyDecision { effect: PolicyEffect::Allow }
    }
}
