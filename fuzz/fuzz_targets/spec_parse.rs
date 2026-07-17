#![no_main]
//! Fuzz the spec-parsing surface: arbitrary bytes are parsed as a `CompileSpec`
//! (JSON). Malformed input must surface as a clean `Err`, never a panic. A
//! successfully parsed spec re-serializes and re-parses to an equal value, and
//! `validate` never panics.

use compile_core::CompileSpec;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(spec) = CompileSpec::from_json(text) else {
        return;
    };
    // A parsed spec round-trips: serialize -> parse -> equal.
    let serialized = serde_json::to_string(&spec).expect("serialize a parsed spec");
    let reparsed = CompileSpec::from_json(&serialized).expect("re-parse a serialized spec");
    assert_eq!(reparsed, spec, "CompileSpec serde round-trip is not stable");
    // Validation must never panic, whatever the (already-parseable) spec.
    let _ = spec.validate();
});
