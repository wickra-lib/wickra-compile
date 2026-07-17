//! The CLI flow: load a spec, apply flag overrides, then generate or build.

use std::fs;
use std::path::Path;

use compile_core::{compile, generate, manifest_of, CompileSpec, Manifest, Target};

use crate::args::{Args, TargetArg};

/// Load a spec from a `.json` or `.toml` file.
fn load(path: &Path) -> Result<CompileSpec, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let is_toml = path.extension().and_then(|e| e.to_str()) == Some("toml");
    if is_toml {
        CompileSpec::from_toml(&text)
    } else {
        CompileSpec::from_json(&text)
    }
    .map_err(|e| e.to_string())
}

/// Apply the `--target` / `--mcu` / `--opt` overrides onto the loaded spec.
fn apply_overrides(spec: &mut CompileSpec, args: &Args) -> Result<(), String> {
    if let Some(target) = args.target {
        spec.target = match target {
            TargetArg::Wasm => Target::Wasm,
            TargetArg::Binary => Target::Binary,
            TargetArg::NoStd => {
                let mcu = args
                    .mcu
                    .clone()
                    .ok_or_else(|| "--mcu is required with --target no_std".to_owned())?;
                Target::NoStd { mcu }
            }
        };
    }
    if let Some(opt) = args.opt {
        spec.opt_level = opt.into();
    }
    Ok(())
}

/// Print the manifest as compact JSON — the machine-readable output that the
/// golden harness compares byte-for-byte against `manifest_of`.
fn print_manifest(manifest: &Manifest) -> Result<(), String> {
    let json = serde_json::to_string(manifest).map_err(|e| e.to_string())?;
    println!("{json}");
    Ok(())
}

/// Run the CLI.
///
/// # Errors
/// Returns a human-readable message on any failure; the caller maps it to a
/// non-zero exit code.
pub fn run(args: &Args) -> Result<(), String> {
    let mut spec = load(&args.spec)?;
    apply_overrides(&mut spec, args)?;

    // `--manifest`: print the manifest JSON and nothing else (no filesystem).
    if args.manifest {
        let manifest = manifest_of(&spec).map_err(|e| e.to_string())?;
        return print_manifest(&manifest);
    }

    // `--dry-run`: generate the project and manifest, write the files, no cargo.
    if args.dry_run {
        let project = generate(&spec).map_err(|e| e.to_string())?;
        let dir = args.out.join(&project.manifest.crate_name);
        project.write_to(&dir).map_err(|e| e.to_string())?;
        eprintln!(
            "generated {} ({} files)",
            dir.display(),
            project.files.len()
        );
        return print_manifest(&project.manifest);
    }

    // Full build: generate, write, then invoke cargo.
    let artifact = compile(&spec, &args.out).map_err(|e| e.to_string())?;
    if let Some(path) = &artifact.path {
        eprintln!("built {path}");
    }
    print_manifest(&artifact.manifest)
}
