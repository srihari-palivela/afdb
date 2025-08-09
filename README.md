
# AFDB Engine (Agent-First DB) — Storage & Processing Engine MVP

## Run demo
```bash
cargo run --example demo
```

## Run tests
```bash
cargo test -q
```

## Configuring embedding and reasoning endpoints

Use `Config` (see `src/config.rs`) to point to your embedding and reasoning APIs.

Example JSON config snippet equivalent:

```json
{
  "data_dir": "data",
  "wal_dir": "wal",
  "segment_size_mb": 128,
  "vector_dims": 384,
  "embedding": {
    "base_url": "https://api.example.com",
    "path": "/v1/embed",
    "model": "text-embedding-3-small",
    "api_key": "${EMBED_API_KEY}",
    "auth_header": "Authorization",
    "headers": [["x-tenant", "acme"]],
    "timeout_ms": 30000
  },
  "reasoning": {
    "base_url": "https://api.example.com",
    "path": "/v1/complete",
    "model": "gpt-4o-mini",
    "api_key": "${LLM_API_KEY}",
    "auth_header": "Authorization",
    "headers": [],
    "timeout_ms": 60000
  }
}
```

Construct clients:

```rust
use afdb::config::Config;
use afdb::semantic::pipeline::{HttpEmbedder, ReasoningClient, Embedder};
use afdb::storage::Engine;

let cfg = Config::default();
let http_emb = HttpEmbedder::new(cfg.embedding.clone().unwrap(), cfg.vector_dims)?;
let llm = ReasoningClient::new(cfg.reasoning.clone().unwrap())?;
let v = http_emb.embed("payment failed on renewal");
let answer = llm.complete("Why did ARR dip last week?", serde_json::json!({"week": "2025-W27"}))?;

// In-kernel embedding on insert
let engine = Engine::new(Box::new(http_emb), cfg.vector_dims);
engine.insert(1, afdb::types::Row { key: afdb::types::RowKey("1".into()), payload: serde_json::json!({"text": "card declined"}) });
let hits = engine.flat_index.read().cosine_topk(&engine.embedder.embed("credit card failed"), 3);
```
