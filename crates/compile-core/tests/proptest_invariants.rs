//! Property-based invariants: over a wide range of specs, codegen never panics
//! and never emits an unsafe path, MCUs outside the allowlist are always
//! rejected, and a resolved crate name is always a valid Rust identifier.

use compile_core::{generate, CompileSpec, Error, OptLevel, Target, MCU_ALLOWLIST};
use proptest::prelude::*;
use serde_json::{json, Value};

fn strategy() -> Value {
    json!({
        "symbol": "x",
        "timeframe": "1h",
        "indicators": { "fast": { "type": "Ema", "params": [3] } },
        "entry": { "cross_above": ["fast", "fast"] },
        "exit": { "cross_below": ["fast", "fast"] },
        "sizing": { "type": "fixed_qty", "qty": 1 }
    })
}

fn spec(target: Target, crate_name: &str) -> CompileSpec {
    CompileSpec {
        strategy: strategy(),
        target,
        opt_level: OptLevel::Release,
        embed_data: None,
        crate_name: Some(crate_name.to_owned()),
    }
}

proptest! {
    /// Codegen never panics, and every generated path is relative and free of
    /// parent references or drive prefixes — no path traversal (see the threat
    /// model).
    #[test]
    fn generate_emits_only_safe_relative_paths(
        name in "[a-z][a-z0-9_]{0,12}",
        kind in 0u8..3,
        mcu in "[a-z0-9._-]{1,20}",
    ) {
        let target = match kind {
            0 => Target::Wasm,
            1 => Target::Binary,
            _ => Target::NoStd { mcu },
        };
        if let Ok(project) = generate(&spec(target, &name)) {
            for path in project.files.keys() {
                prop_assert!(!path.starts_with('/'), "absolute path: {path}");
                prop_assert!(!path.contains(".."), "parent reference in path: {path}");
                prop_assert!(!path.contains(':'), "drive-letter path: {path}");
                prop_assert!(!path.starts_with('\\'), "absolute path: {path}");
            }
        }
    }

    /// An MCU triple outside the allowlist is always a `BadTarget`.
    #[test]
    fn mcu_outside_allowlist_is_bad_target(mcu in "[a-z0-9._-]{1,20}") {
        prop_assume!(!MCU_ALLOWLIST.contains(&mcu.as_str()));
        let spec = spec(Target::NoStd { mcu }, "demo");
        prop_assert!(matches!(spec.validate(), Err(Error::BadTarget(_))));
    }

    /// A resolved crate name is always a valid identifier `^[a-z][a-z0-9_]*$`.
    #[test]
    fn resolved_crate_name_is_a_valid_identifier(name in ".{0,20}") {
        let spec = spec(Target::Wasm, &name);
        if let Ok(resolved) = spec.resolved_crate_name() {
            let mut chars = resolved.chars();
            prop_assert!(chars.next().is_some_and(|c| c.is_ascii_lowercase()));
            prop_assert!(chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'));
        }
    }
}
