//! The byte-handle reader, `reset`, and the two version answers.
//!
//! `artifact_bytes` and `artifact_handle_bytes` are the C-ABI byte channel: a
//! caller asks for an artifact's bytes, gets a handle, a length and a digest,
//! and reads the bytes back through the handle. That pair is what every
//! non-Rust binding uses to take a compiled binary across the boundary, and no
//! Rust test touched either -- nor `reset`, which is what a long-lived handle
//! is released by, nor the version pair a generated project is pinned to.

use std::fs;

use serde_json::Value;
use wickra_compile_core::{sha256_hex, Compiler};

#[test]
fn the_versions_answer_the_crate_and_its_pinned_engine() {
    assert_eq!(Compiler::version(), wickra_compile_core::version());
    assert_eq!(Compiler::version(), env!("CARGO_PKG_VERSION"));
    let dep = Compiler::backtest_dep();
    assert!(
        dep.split('.').count() >= 2 && dep.split('.').all(|p| p.parse::<u32>().is_ok()),
        "the pinned engine reads like a version: {dep}"
    );
}

#[test]
fn a_new_compiler_is_the_default_one() {
    let mut fresh = Compiler::new();
    let mut default = Compiler::default();
    // Both answer the same to the same command, which is what "same state" means
    // for a handle that carries no public fields.
    let cmd = r#"{"cmd":"version"}"#;
    assert_eq!(fresh.command_json(cmd), default.command_json(cmd));
}

#[test]
fn bytes_are_read_through_a_handle_and_released_by_reset() {
    // The same shape operating_modes.rs uses: a process-scoped directory under
    // the system temp, removed at the end.
    let dir = std::env::temp_dir().join(format!("wickra-compile-handles-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("a temp dir");
    let path = dir.join("artifact.bin");
    let payload: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    fs::write(&path, &payload).expect("write the artifact");

    let mut compiler = Compiler::new();
    let request = serde_json::json!({ "cmd": "artifact_bytes", "path": path }).to_string();
    let answer: Value =
        serde_json::from_str(&compiler.command_json(&request)).expect("the answer is JSON");

    assert_eq!(
        answer["len"].as_u64().expect("a length"),
        payload.len() as u64
    );
    assert_eq!(
        answer["sha256"],
        sha256_hex(&payload),
        "the digest is of the bytes"
    );

    let handle = answer["handle"].as_u64().expect("a handle");
    assert_eq!(
        compiler.artifact_handle_bytes(handle),
        Some(payload.as_slice()),
        "the handle reads back exactly what was written"
    );
    assert_eq!(
        compiler.artifact_handle_bytes(handle + 1),
        None,
        "an unknown handle is None, not a panic"
    );

    // A second request opens a second handle; both stay readable until reset.
    let second: Value =
        serde_json::from_str(&compiler.command_json(&request)).expect("the answer is JSON");
    let handle2 = second["handle"].as_u64().expect("a handle");
    assert_ne!(handle, handle2, "handles are not reused while open");
    assert!(compiler.artifact_handle_bytes(handle).is_some());

    compiler.reset();
    assert_eq!(compiler.artifact_handle_bytes(handle), None);
    assert_eq!(compiler.artifact_handle_bytes(handle2), None);

    fs::remove_dir_all(&dir).expect("the temp dir goes");
}

#[test]
fn a_malformed_bytes_request_is_answered_in_band() {
    let mut compiler = Compiler::new();
    for (request, expected) in [
        (r#"{"cmd":"artifact_bytes"}"#, "bad artifact_bytes request"),
        (
            r#"{"cmd":"artifact_bytes","path":"no/such/file.bin"}"#,
            "reading no/such/file.bin",
        ),
    ] {
        let answer: Value =
            serde_json::from_str(&compiler.command_json(request)).expect("the answer is JSON");
        assert_eq!(answer["ok"], false, "{request}");
        assert!(
            answer["error"]
                .as_str()
                .expect("an error message")
                .contains(expected),
            "{request} -> {answer}"
        );
    }
}

#[test]
fn a_malformed_compile_request_is_answered_in_band() {
    let mut compiler = Compiler::new();
    let answer: Value =
        serde_json::from_str(&compiler.command_json(r#"{"cmd":"compile","spec":7}"#))
            .expect("the answer is JSON");
    assert_eq!(answer["ok"], false);
    assert!(answer["error"]
        .as_str()
        .expect("an error message")
        .contains("bad compile request"));
}
