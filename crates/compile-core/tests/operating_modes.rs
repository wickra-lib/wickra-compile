//! Operating-mode equivalence: the manifest a dry run describes is the manifest
//! a real build builds.
//!
//! The command protocol has two ways to run `compile`: `dry_run: true` stops
//! after codegen and returns the manifest, `dry_run: false` writes the project
//! and invokes `cargo` on it. The `Artifact` both return must carry the same
//! manifest bytes -- the same files, the same hashes, the same `project_hash`
//! -- with `built` and `path` the only difference. The blessed golden is the
//! dry-run manifest, so this pins that a real build produces what the golden
//! promises.
//!
//! The no_std spec is the one built for real: its generated project has no
//! dependencies, so it compiles in seconds, and it needs only the
//! `thumbv7em-none-eabihf` target the CI jobs install. The bindings repeat this
//! check at their own boundary; this is the in-core anchor.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use wickra_compile_core::Compiler;

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

#[test]
fn a_real_build_carries_the_dry_run_manifest() {
    let golden = golden_dir();
    let spec = fs::read_to_string(golden.join("specs/no_std_blink.json")).unwrap();
    let expected: Value = serde_json::from_str(
        &fs::read_to_string(golden.join("expected/no_std_blink.json")).unwrap(),
    )
    .unwrap();

    let mut compiler = Compiler::new();
    let dry: Value = serde_json::from_str(&compiler.command_json(&format!(
        r#"{{"cmd":"compile","dry_run":true,"spec":{spec}}}"#
    )))
    .unwrap();
    assert_eq!(dry["built"], Value::Bool(false));
    assert!(dry.get("path").is_none());
    assert_eq!(dry["manifest"], expected);

    let out_dir = std::env::temp_dir().join(format!("wickra-compile-modes-{}", std::process::id()));
    let out = out_dir.to_string_lossy().replace('\\', "/");
    let built: Value = serde_json::from_str(&compiler.command_json(&format!(
        r#"{{"cmd":"compile","dry_run":false,"out_dir":"{out}","spec":{spec}}}"#
    )))
    .unwrap();
    assert_eq!(built["built"], Value::Bool(true), "{built}");
    let path = built["path"].as_str().expect("a built artifact has a path");
    assert!(fs::metadata(path).is_ok(), "artifact missing at {path}");
    assert_eq!(built["manifest"], dry["manifest"]);

    fs::remove_dir_all(&out_dir).unwrap();
}
