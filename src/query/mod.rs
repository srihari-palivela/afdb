
pub mod operators;
pub mod planner;

use regex::Regex;

// A minimal SemanticQL parser for patterns like:
// FIND SIMILAR "<query>" IN <space>
#[derive(Debug, Clone)]
pub struct SemanticQl {
    pub query: String,
    pub space: String,
    pub k: usize,
}

impl SemanticQl {
    pub fn parse(input: &str) -> Option<Self> {
        // Raw string with escaped quotes around the query capture
        let re = Regex::new(r#"\s*FIND\s+SIMILAR\s+\"(.+?)\"\s+IN\s+([a-zA-Z0-9_]+)(?:\s+TOP\s+(\d+))?\s*"#).ok()?;
        let caps = re.captures(input.trim())?;
        let query = caps.get(1)?.as_str().to_string();
        let space = caps.get(2)?.as_str().to_string();
        let k = caps.get(3).map(|m| m.as_str().parse::<usize>().unwrap_or(10)).unwrap_or(10);
        Some(Self { query, space, k })
    }
}
