//! Canonical JSON serialisation and SHA-256 hashing — the determinism core.
//!
//! A spec is hashed after being brought into a canonical form: object keys
//! recursively sorted, compact (no whitespace). Because only this Rust core ever
//! computes a hash — every language binding passes the command string through
//! verbatim — the manifest is byte-identical across languages by construction.

use std::fmt::Write as _;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::error::{Error, Result};
use crate::spec::CompileSpec;

/// Recursively sort every object's keys, leaving arrays in order and scalars
/// untouched.
fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_unstable();
            let mut sorted = Map::with_capacity(map.len());
            for key in keys {
                sorted.insert(key.clone(), canonicalize(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.iter().map(canonicalize).collect()),
        scalar => scalar.clone(),
    }
}

/// Render a value as canonical JSON: keys sorted recursively, compact.
#[must_use]
pub fn canonical_json(value: &Value) -> String {
    // `to_string` on a canonicalised value cannot fail (no non-string map keys,
    // no invalid floats — those never reach here from validated specs).
    serde_json::to_string(&canonicalize(value)).unwrap_or_default()
}

/// Lowercase hex of the SHA-256 of `bytes`.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

/// The SHA-256 (hex) of the canonical `CompileSpec` — the `spec_hash`.
///
/// # Errors
/// Returns [`Error::Parse`] if the spec cannot be serialised to JSON.
pub fn spec_hash(spec: &CompileSpec) -> Result<String> {
    let value = serde_json::to_value(spec).map_err(|e| Error::Parse(e.to_string()))?;
    Ok(sha256_hex(canonical_json(&value).as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_sorted_recursively() {
        let unsorted: Value = serde_json::json!({ "b": 1, "a": { "y": 2, "x": 3 } });
        assert_eq!(canonical_json(&unsorted), r#"{"a":{"x":3,"y":2},"b":1}"#);
    }

    #[test]
    fn arrays_keep_order() {
        let value: Value = serde_json::json!({ "xs": [3, 1, 2] });
        assert_eq!(canonical_json(&value), r#"{"xs":[3,1,2]}"#);
    }

    #[test]
    fn sha256_is_stable_and_lowercase_hex() {
        // Known SHA-256 of the empty input.
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
