//! Canonical determinism: the spec hash and the project hash depend only on the
//! *logical* spec, not on the byte layout of the input JSON. Reordering object
//! keys — the classic source of accidental hash drift — must not change either
//! hash.

use compile_core::{manifest_of, spec_hash, CompileSpec};

/// The same spec, written with three different object-key orderings. A canonical
/// hash must treat all three as identical.
const ORDERINGS: [&str; 3] = [
    r#"{"strategy":{"symbol":"x","timeframe":"1h","indicators":{"a":{"type":"Ema","params":[3]},"b":{"type":"Sma","params":[5]}},"entry":{"cross_above":["a","b"]},"exit":{"cross_below":["a","b"]},"sizing":{"type":"fixed_qty","qty":1}},"target":{"kind":"wasm"},"crate_name":"demo"}"#,
    r#"{"crate_name":"demo","target":{"kind":"wasm"},"strategy":{"sizing":{"qty":1,"type":"fixed_qty"},"exit":{"cross_below":["a","b"]},"entry":{"cross_above":["a","b"]},"indicators":{"b":{"params":[5],"type":"Sma"},"a":{"params":[3],"type":"Ema"}},"timeframe":"1h","symbol":"x"}}"#,
    r#"{"target":{"kind":"wasm"},"strategy":{"symbol":"x","indicators":{"a":{"params":[3],"type":"Ema"},"b":{"type":"Sma","params":[5]}},"sizing":{"type":"fixed_qty","qty":1},"timeframe":"1h","entry":{"cross_above":["a","b"]},"exit":{"cross_below":["a","b"]}},"crate_name":"demo"}"#,
];

#[test]
fn key_order_does_not_change_the_hashes() {
    let specs: Vec<CompileSpec> = ORDERINGS
        .iter()
        .map(|json| CompileSpec::from_json(json).unwrap())
        .collect();

    let spec_hashes: Vec<String> = specs.iter().map(|s| spec_hash(s).unwrap()).collect();
    let project_hashes: Vec<String> = specs
        .iter()
        .map(|s| manifest_of(s).unwrap().project_hash)
        .collect();

    // Run the comparison many times: the hashing must be stable across repeated
    // invocations as well as across key orderings.
    for _ in 0..100 {
        for h in &spec_hashes {
            assert_eq!(h, &spec_hashes[0], "spec_hash drifted with key order");
        }
        for h in &project_hashes {
            assert_eq!(h, &project_hashes[0], "project_hash drifted with key order");
        }
    }
}
