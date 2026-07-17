//! The golden invariant, from Rust: every `golden/specs/*.json` regenerates its
//! `golden/expected/*.json` manifest byte-for-byte. `serde_json::to_string`
//! of `manifest_of` is exactly what the CLI's `--manifest` and every language
//! binding produce, so this file is the reference the cross-language corpus is
//! pinned to.

use std::fs;
use std::path::PathBuf;

use compile_core::{manifest_of, CompileSpec};

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

#[test]
fn every_spec_matches_its_expected_manifest() {
    let dir = golden_dir();
    // `embed_data` CSV paths are resolved relative to the working directory, so
    // run the codegen from the repository root.
    std::env::set_current_dir(dir.join("..")).unwrap();

    let mut specs: Vec<_> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();

    let mut checked = 0;
    for spec_path in specs {
        let stem = spec_path.file_stem().unwrap().to_str().unwrap();
        let spec = CompileSpec::from_json(&fs::read_to_string(&spec_path).unwrap()).unwrap();
        let got = serde_json::to_string(&manifest_of(&spec).unwrap()).unwrap();

        let expected_path = dir.join("expected").join(format!("{stem}.json"));
        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", expected_path.display()));

        assert_eq!(
            got,
            expected.trim_end(),
            "golden mismatch for {stem}: the manifest no longer matches the \
             committed fixture (re-bless if the change is intended)"
        );
        checked += 1;
    }
    assert_eq!(checked, 5, "expected 5 golden specs, checked {checked}");
}
