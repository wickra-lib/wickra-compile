#![no_main]
//! Fuzz the codegen surface: any spec that parses and validates is generated
//! without panicking, and every generated path is relative and free of parent
//! references or absolute prefixes — the path-traversal guarantee from the
//! threat model.

use compile_core::{generate, CompileSpec};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(spec) = CompileSpec::from_json(text) else {
        return;
    };
    // Only generate specs that pass structural validation (a valid strategy and
    // target); everything else is expected to be rejected earlier.
    if spec.validate().is_err() {
        return;
    }
    let Ok(project) = generate(&spec) else {
        return;
    };
    for path in project.files.keys() {
        assert!(!path.starts_with('/'), "absolute path escaped codegen: {path}");
        assert!(!path.starts_with('\\'), "absolute path escaped codegen: {path}");
        assert!(!path.contains(".."), "parent reference escaped codegen: {path}");
        assert!(!path.contains(':'), "drive-letter path escaped codegen: {path}");
    }
});
