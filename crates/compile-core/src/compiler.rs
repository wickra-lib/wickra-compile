//! The stateful command handle and the `command_json` FFI boundary.
//!
//! A single entry point dispatches the command table. Every error is reported
//! **in band** as `{"ok":false,"error":...}` — `command_json` never returns an
//! `Err` and never panics, so the C-ABI layer only has to guard against null and
//! non-UTF-8 arguments.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::json;

use crate::canonical::sha256_hex;
use crate::codegen::generate;
use crate::manifest::Artifact;
use crate::spec::CompileSpec;
use crate::targets::{MCU_ALLOWLIST, TARGET_KINDS};
use crate::{version, BACKTEST_DEP};

/// The stateful compiler handle. Holds the last artifact path and any open bytes
/// handles for the C-ABI byte reader.
#[derive(Default)]
pub struct Compiler {
    last_artifact: Option<String>,
    bytes_handles: BTreeMap<u64, Vec<u8>>,
    next_handle: u64,
}

#[derive(Deserialize)]
struct Envelope {
    cmd: String,
}

#[derive(Deserialize)]
struct CompileReq {
    spec: CompileSpec,
    #[serde(default)]
    dry_run: bool,
    #[serde(default)]
    #[allow(dead_code)] // consumed only by the build path (feature = "build")
    out_dir: Option<String>,
}

#[derive(Deserialize)]
struct BytesReq {
    path: String,
}

fn err_json(message: &str) -> String {
    json!({ "ok": false, "error": message }).to_string()
}

impl Compiler {
    /// Create a fresh handle.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The compiler version.
    #[must_use]
    pub fn version() -> &'static str {
        version()
    }

    /// The pinned `wickra-backtest` dependency version.
    #[must_use]
    pub fn backtest_dep() -> &'static str {
        BACKTEST_DEP
    }

    /// Drop open bytes handles and the last artifact.
    pub fn reset(&mut self) {
        self.last_artifact = None;
        self.bytes_handles.clear();
        self.next_handle = 0;
    }

    /// Dispatch a `{"cmd":...}` command, returning a JSON response string. Errors
    /// are in-band `{"ok":false,"error":...}`; this never returns `Err`.
    #[must_use]
    pub fn command_json(&mut self, cmd_json: &str) -> String {
        let Ok(envelope) = serde_json::from_str::<Envelope>(cmd_json) else {
            return err_json("invalid command envelope: expected {\"cmd\":...}");
        };
        match envelope.cmd.as_str() {
            "compile" => self.cmd_compile(cmd_json),
            "targets" => Self::cmd_targets(),
            "version" => Self::cmd_version(),
            "artifact_bytes" => self.cmd_artifact_bytes(cmd_json),
            "reset" => {
                self.reset();
                json!({ "ok": true }).to_string()
            }
            other => err_json(&format!("unknown cmd: {other}")),
        }
    }

    fn cmd_targets() -> String {
        json!({ "targets": TARGET_KINDS, "mcus": MCU_ALLOWLIST }).to_string()
    }

    fn cmd_version() -> String {
        json!({ "version": version(), "backtest_dep": BACKTEST_DEP }).to_string()
    }

    fn cmd_compile(&mut self, cmd_json: &str) -> String {
        let req: CompileReq = match serde_json::from_str(cmd_json) {
            Ok(req) => req,
            Err(e) => return err_json(&format!("bad compile request: {e}")),
        };
        if req.dry_run {
            return match generate(&req.spec) {
                Ok(project) => {
                    let artifact = Artifact {
                        target: req.spec.target.clone(),
                        path: None,
                        manifest: project.manifest,
                        built: false,
                    };
                    serde_json::to_string(&artifact)
                        .unwrap_or_else(|e| err_json(&format!("serialize: {e}")))
                }
                Err(e) => err_json(&e.to_string()),
            };
        }
        self.build(&req)
    }

    #[cfg(feature = "build")]
    fn build(&mut self, req: &CompileReq) -> String {
        use std::path::Path;
        let out_dir = req.out_dir.as_deref().unwrap_or("_out");
        match crate::builder::compile(&req.spec, Path::new(out_dir)) {
            Ok(artifact) => {
                self.last_artifact.clone_from(&artifact.path);
                serde_json::to_string(&artifact)
                    .unwrap_or_else(|e| err_json(&format!("serialize: {e}")))
            }
            Err(e) => err_json(&e.to_string()),
        }
    }

    #[cfg(not(feature = "build"))]
    #[allow(clippy::unused_self)] // signature mirrors the `build`-feature variant
    fn build(&mut self, _req: &CompileReq) -> String {
        err_json("build mode unavailable (the `build` feature is disabled)")
    }

    fn cmd_artifact_bytes(&mut self, cmd_json: &str) -> String {
        let req: BytesReq = match serde_json::from_str(cmd_json) {
            Ok(req) => req,
            Err(e) => return err_json(&format!("bad artifact_bytes request: {e}")),
        };
        let bytes = match std::fs::read(&req.path) {
            Ok(bytes) => bytes,
            Err(e) => return err_json(&format!("reading {}: {e}", req.path)),
        };
        let handle = self.next_handle;
        self.next_handle += 1;
        let response = json!({
            "handle": handle,
            "len": bytes.len(),
            "sha256": sha256_hex(&bytes),
        })
        .to_string();
        self.bytes_handles.insert(handle, bytes);
        response
    }

    /// Read the bytes of an open handle (for the C-ABI byte reader). Returns the
    /// bytes, or `None` if the handle is unknown.
    #[must_use]
    pub fn artifact_handle_bytes(&self, handle: u64) -> Option<&[u8]> {
        self.bytes_handles.get(&handle).map(Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compile_cmd(dry_run: bool) -> String {
        json!({
            "cmd": "compile",
            "dry_run": dry_run,
            "spec": {
                "strategy": {
                    "symbol": "x",
                    "timeframe": "1h",
                    "indicators": { "fast": { "type": "Ema", "params": [3] } },
                    "entry": { "cross_above": ["fast", "fast"] },
                    "exit": { "cross_below": ["fast", "fast"] },
                    "sizing": { "type": "fixed_qty", "qty": 1 }
                },
                "target": { "kind": "wasm" },
                "opt_level": "size",
                "crate_name": "demo"
            }
        })
        .to_string()
    }

    #[test]
    fn version_and_targets_roundtrip() {
        let mut c = Compiler::new();
        let v = c.command_json(r#"{"cmd":"version"}"#);
        assert!(v.contains("\"version\""));
        let t = c.command_json(r#"{"cmd":"targets"}"#);
        assert!(t.contains("wasm") && t.contains("thumbv6m-none-eabi"));
    }

    #[test]
    fn dry_run_compile_returns_manifest_not_built() {
        let mut c = Compiler::new();
        let out = c.command_json(&compile_cmd(true));
        assert!(out.contains("\"built\":false"));
        assert!(out.contains("\"project_hash\""));
        assert!(out.contains("\"crate_name\":\"demo\""));
    }

    #[test]
    fn unknown_cmd_is_error_json_not_panic() {
        let mut c = Compiler::new();
        let out = c.command_json(r#"{"cmd":"nope"}"#);
        assert!(out.contains("\"ok\":false"));
        assert!(out.contains("unknown cmd"));
    }

    #[test]
    fn bad_envelope_is_error_json() {
        let mut c = Compiler::new();
        assert!(c.command_json("not json").contains("\"ok\":false"));
    }

    #[test]
    fn bad_spec_is_error_json() {
        let mut c = Compiler::new();
        let cmd = json!({
            "cmd": "compile", "dry_run": true,
            "spec": { "strategy": { "not": "a strategy" }, "target": { "kind": "binary" } }
        })
        .to_string();
        let out = c.command_json(&cmd);
        assert!(out.contains("\"ok\":false"));
    }

    #[test]
    fn reset_is_ok() {
        let mut c = Compiler::new();
        assert_eq!(c.command_json(r#"{"cmd":"reset"}"#), r#"{"ok":true}"#);
    }
}
