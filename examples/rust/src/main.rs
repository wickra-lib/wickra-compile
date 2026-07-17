//! A runnable Rust example: compile a strategy spec (dry run) and print its
//! deterministic manifest. Every language example uses the same spec and prints
//! the same `project_hash` — that is the cross-language guarantee.
//!
//! ```bash
//! cargo run --manifest-path examples/rust/Cargo.toml
//! ```

use compile_core::{manifest_of, CompileSpec};

const SPEC: &str = r#"{
    "strategy": {
        "symbol": "btcusdt",
        "timeframe": "1h",
        "indicators": {
            "fast": { "type": "Sma", "params": [10] },
            "slow": { "type": "Sma", "params": [30] }
        },
        "entry": { "cross_above": ["fast", "slow"] },
        "exit": { "cross_below": ["fast", "slow"] },
        "sizing": { "type": "fixed_qty", "qty": 1 }
    },
    "target": { "kind": "wasm" },
    "crate_name": "demo"
}"#;

fn main() {
    let spec = CompileSpec::from_json(SPEC).expect("valid spec");
    let manifest = manifest_of(&spec).expect("manifest");

    println!("wickra-compile {}", compile_core::version());
    println!("crate: {}", manifest.crate_name);
    println!("files: {}", manifest.files.len());
    println!("project_hash: {}", manifest.project_hash);
}
