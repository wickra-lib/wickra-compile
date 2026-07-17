#![no_main]
//! Fuzz the canonicalisation + hashing surface: arbitrary JSON is canonicalised
//! and hashed without panicking, and canonicalisation is idempotent — feeding a
//! canonicalised value back through must produce the identical string, which is
//! the property the deterministic hashing relies on.

use compile_core::{canonical_json, sha256_hex};
use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        return;
    };
    let canonical = canonical_json(&value);
    // Hashing the canonical form never panics.
    let _ = sha256_hex(canonical.as_bytes());
    // Canonicalisation is idempotent: re-parsing and re-canonicalising is a
    // fixed point.
    let reparsed: Value = serde_json::from_str(&canonical).expect("canonical json re-parses");
    assert_eq!(
        canonical_json(&reparsed),
        canonical,
        "canonical_json is not idempotent"
    );
});
