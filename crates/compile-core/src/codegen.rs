//! The codegen entry point: `CompileSpec` → generated project + manifest.
//!
//! This is the golden path: pure code generation and hashing, no `cargo`. The
//! same spec always produces the same files and the same manifest.

use std::collections::BTreeMap;
use std::path::Path;

use crate::canonical;
use crate::error::{Error, Result};
use crate::manifest::{self, Manifest};
use crate::spec::{CompileSpec, DatasetRef};
use crate::templates::{self, TemplateCtx};
use crate::{version, BACKTEST_DEP};

/// A generated project: the file set and its deterministic manifest.
pub struct GeneratedProject {
    /// Path (project-relative) to file bytes, in stable path order.
    pub files: BTreeMap<String, Vec<u8>>,
    /// The deterministic manifest.
    pub manifest: Manifest,
}

impl GeneratedProject {
    /// Write the generated files under `dir`, creating parent directories.
    ///
    /// # Errors
    /// Returns [`Error::Io`] if a directory or file cannot be created.
    pub fn write_to(&self, dir: &Path) -> Result<()> {
        for (rel, bytes) in &self.files {
            let path = dir.join(rel);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| Error::Io(format!("creating {}: {e}", parent.display())))?;
            }
            std::fs::write(&path, bytes)
                .map_err(|e| Error::Io(format!("writing {}: {e}", path.display())))?;
        }
        Ok(())
    }
}

/// Generate the project (files + manifest) for a validated spec — no `cargo`.
///
/// # Errors
/// - [`Error::BadSpec`] / [`Error::BadTarget`] from validation.
/// - [`Error::Io`] if a referenced CSV cannot be read.
/// - [`Error::Parse`] if the spec cannot be hashed.
pub fn generate(spec: &CompileSpec) -> Result<GeneratedProject> {
    spec.validate()?;
    let crate_name = spec.resolved_crate_name()?;

    // The embedded spec.json is the canonical StrategySpec (raw data).
    let spec_json = canonical::canonical_json(&spec.strategy);

    // CSV embedded data is read here (generate needs filesystem access).
    let embedded_csv = match &spec.embed_data {
        Some(DatasetRef::Csv { path }) => Some(
            std::fs::read_to_string(path).map_err(|e| Error::Io(format!("reading {path}: {e}")))?,
        ),
        _ => None,
    };

    let ctx = TemplateCtx {
        crate_name: crate_name.clone(),
        backtest_dep: BACKTEST_DEP.to_owned(),
        opt_level: spec.opt_level,
        spec_json,
        embedded_csv,
    };

    let mut files: BTreeMap<String, Vec<u8>> = templates::render(&spec.target, &ctx)
        .into_iter()
        .map(|(path, content)| (path, content.into_bytes()))
        .collect();

    // Inline candle data is embedded as canonical JSON.
    if let Some(DatasetRef::Inline { candles }) = &spec.embed_data {
        let array = serde_json::Value::Array(candles.clone());
        files.insert(
            "src/candles.json".to_owned(),
            canonical::canonical_json(&array).into_bytes(),
        );
    }

    let manifest = Manifest {
        compiler_version: version().to_owned(),
        target: spec.target.clone(),
        opt_level: spec.opt_level,
        crate_name,
        spec_hash: canonical::spec_hash(spec)?,
        backtest_dep: BACKTEST_DEP.to_owned(),
        files: manifest::generated_files(&files),
        project_hash: manifest::project_hash(&files),
    };

    Ok(GeneratedProject { files, manifest })
}

/// The manifest for a spec — `generate(spec)?.manifest`.
///
/// # Errors
/// Same as [`generate`].
pub fn manifest_of(spec: &CompileSpec) -> Result<Manifest> {
    Ok(generate(spec)?.manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{OptLevel, Target};
    use serde_json::json;

    fn spec(target: Target) -> CompileSpec {
        CompileSpec {
            strategy: json!({
                "symbol": "x",
                "timeframe": "1h",
                "indicators": { "fast": { "type": "Ema", "params": [3] } },
                "entry": { "cross_above": ["fast", "fast"] },
                "exit": { "cross_below": ["fast", "fast"] },
                "sizing": { "type": "fixed_qty", "qty": 1 }
            }),
            target,
            opt_level: OptLevel::Size,
            embed_data: None,
            crate_name: Some("demo".to_owned()),
        }
    }

    #[test]
    fn generate_is_deterministic() {
        let a = generate(&spec(Target::Wasm)).unwrap();
        let b = generate(&spec(Target::Wasm)).unwrap();
        assert_eq!(a.manifest, b.manifest);
        assert_eq!(a.manifest.project_hash, b.manifest.project_hash);
    }

    #[test]
    fn manifest_lists_expected_wasm_files() {
        let m = manifest_of(&spec(Target::Wasm)).unwrap();
        let paths: Vec<&str> = m.files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![
                ".cargo/config.toml",
                "Cargo.toml",
                "src/lib.rs",
                "src/spec.json"
            ]
        );
        assert_eq!(m.crate_name, "demo");
        assert_eq!(m.backtest_dep, BACKTEST_DEP);
    }

    #[test]
    fn different_target_changes_project_hash() {
        let wasm = manifest_of(&spec(Target::Wasm)).unwrap();
        let bin = manifest_of(&spec(Target::Binary)).unwrap();
        assert_ne!(wasm.project_hash, bin.project_hash);
        // ...but the spec hash is target-independent only if target is not in the
        // spec; here target IS part of CompileSpec, so spec_hash differs too.
        assert_ne!(wasm.spec_hash, bin.spec_hash);
    }
}
