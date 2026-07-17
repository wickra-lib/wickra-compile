//! The optional build step: generate the project, then invoke `cargo` on it.
//!
//! Gated behind the `build` feature. `cargo` is invoked by argv (a `Vec<String>`),
//! never through a shell, and the target triple is allowlisted upstream — so no
//! spec value can break out into a command. Run only on trusted specs (see
//! `THREAT_MODEL.md`): building executes `build.rs` and proc-macros from the
//! dependency tree.

use std::path::Path;
use std::process::Command;

use crate::codegen::generate;
use crate::error::{Error, Result};
use crate::manifest::Artifact;
use crate::spec::{CompileSpec, OptLevel};

/// Keep the last portion of a build's stderr, so error messages stay bounded.
fn stderr_tail(bytes: &[u8]) -> String {
    const MAX: usize = 2000;
    let text = String::from_utf8_lossy(bytes);
    if text.len() <= MAX {
        text.into_owned()
    } else {
        text[text.len() - MAX..].to_owned()
    }
}

/// The expected artifact path within the generated crate, relative to it.
fn artifact_rel_path(spec: &CompileSpec, crate_name: &str) -> String {
    let profile = match spec.opt_level {
        OptLevel::Debug => "debug",
        OptLevel::Release | OptLevel::Size => "release",
    };
    match spec.target.triple() {
        // wasm-pack emits pkg/<crate>_bg.wasm.
        Some(triple) if spec.target.needs_wasm_pack() => {
            let _ = triple;
            format!("pkg/{crate_name}_bg.wasm")
        }
        Some(triple) => format!("target/{triple}/{profile}/lib{crate_name}.a"),
        None => format!("target/{profile}/{crate_name}"),
    }
}

/// Generate the project under `out_dir` and build it with `cargo`.
///
/// # Errors
/// - Validation / codegen errors from [`generate`].
/// - [`Error::Build`] if `cargo` cannot be spawned or exits non-zero.
pub fn compile(spec: &CompileSpec, out_dir: &Path) -> Result<Artifact> {
    let project = generate(spec)?;
    let crate_name = project.manifest.crate_name.clone();
    let crate_dir = out_dir.join(&crate_name);
    project.write_to(&crate_dir)?;

    let args = spec.target.cargo_args(spec.opt_level);
    let output = Command::new("cargo")
        .args(&args)
        .current_dir(&crate_dir)
        .output()
        .map_err(|e| Error::Build(format!("spawning cargo: {e}")))?;

    if !output.status.success() {
        return Err(Error::Build(stderr_tail(&output.stderr)));
    }

    let rel = artifact_rel_path(spec, &crate_name);
    let path = crate_dir.join(&rel).to_string_lossy().into_owned();

    Ok(Artifact {
        target: spec.target.clone(),
        path: Some(path),
        manifest: project.manifest,
        built: true,
    })
}
