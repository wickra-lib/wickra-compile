//! Serde and validation conformance: every `Target`, `OptLevel` and `DatasetRef`
//! variant round-trips through JSON unchanged, `spec_hash` is stable, and the
//! documented error paths (bad MCU, bad crate name) actually reject.

use compile_core::{spec_hash, CompileSpec, DatasetRef, Error, OptLevel, Target};
use serde_json::{json, Value};

fn sample_strategy() -> Value {
    json!({
        "symbol": "x",
        "timeframe": "1h",
        "indicators": { "fast": { "type": "Ema", "params": [3] } },
        "entry": { "cross_above": ["fast", "fast"] },
        "exit": { "cross_below": ["fast", "fast"] },
        "sizing": { "type": "fixed_qty", "qty": 1 }
    })
}

fn roundtrip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(&back, value, "serde round-trip changed the value: {json}");
}

#[test]
fn target_variants_round_trip() {
    roundtrip(&Target::Wasm);
    roundtrip(&Target::Binary);
    roundtrip(&Target::NoStd {
        mcu: "thumbv7em-none-eabihf".to_owned(),
    });
}

#[test]
fn opt_level_variants_round_trip() {
    roundtrip(&OptLevel::Debug);
    roundtrip(&OptLevel::Release);
    roundtrip(&OptLevel::Size);
}

#[test]
fn dataset_ref_variants_round_trip() {
    roundtrip(&DatasetRef::Csv {
        path: "data/candles.csv".to_owned(),
    });
    roundtrip(&DatasetRef::Inline {
        candles: vec![json!({ "close": 1.0 })],
    });
}

#[test]
fn spec_hash_is_stable_and_hex() {
    let spec = CompileSpec {
        strategy: sample_strategy(),
        target: Target::Wasm,
        opt_level: OptLevel::default(),
        embed_data: None,
        crate_name: Some("demo".to_owned()),
    };
    let a = spec_hash(&spec).unwrap();
    let b = spec_hash(&spec).unwrap();
    assert_eq!(a, b, "spec_hash is not deterministic");
    assert_eq!(a.len(), 64, "spec_hash is not a sha-256 hex digest");
    assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn unknown_mcu_is_rejected() {
    let spec = CompileSpec {
        strategy: sample_strategy(),
        target: Target::NoStd {
            mcu: "x86_64-unknown-linux-gnu".to_owned(),
        },
        opt_level: OptLevel::default(),
        embed_data: None,
        crate_name: Some("demo".to_owned()),
    };
    assert!(matches!(spec.validate(), Err(Error::BadTarget(_))));
}

#[test]
fn bad_crate_name_is_rejected() {
    let spec = CompileSpec {
        strategy: sample_strategy(),
        target: Target::Wasm,
        opt_level: OptLevel::default(),
        embed_data: None,
        crate_name: Some("1nope".to_owned()),
    };
    assert!(matches!(spec.resolved_crate_name(), Err(Error::BadSpec(_))));
}
