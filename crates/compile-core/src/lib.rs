//! Codegen core for `wickra-compile`.
//!
//! Turn a [`CompileSpec`] — a strategy spec plus a target — into a standalone,
//! deployable Rust project, and produce a **deterministic manifest** describing
//! it. The manifest (file hashes + spec hash + project hash) is byte-identical
//! across every language binding and across repeated runs of the same spec,
//! because only this Rust core ever hashes anything; the bindings pass the
//! command string through verbatim.
//!
//! ```
//! use compile_core::{generate, CompileSpec};
//! let spec: CompileSpec = serde_json::from_str(r#"{
//!     "strategy": { "symbol": "x", "timeframe": "1h",
//!         "indicators": { "f": { "type": "Ema", "params": [3] } },
//!         "entry": { "cross_above": ["f", "f"] },
//!         "exit": { "cross_below": ["f", "f"] },
//!         "sizing": { "type": "fixed_qty", "qty": 1 } },
//!     "target": { "kind": "binary" }, "crate_name": "demo"
//! }"#).unwrap();
//! let project = generate(&spec).unwrap();
//! assert_eq!(project.manifest.crate_name, "demo");
//! ```

#[cfg(feature = "build")]
mod builder;
mod canonical;
mod codegen;
mod compiler;
mod config;
mod error;
mod manifest;
mod spec;
mod targets;
mod templates;

pub use canonical::{canonical_json, sha256_hex, spec_hash};
pub use codegen::{generate, manifest_of, GeneratedProject};
pub use compiler::Compiler;
pub use config::Config;
pub use error::{Error, Result};
pub use manifest::{project_hash, Artifact, GeneratedFile, Manifest};
pub use spec::{CompileSpec, DatasetRef, OptLevel, Target};
pub use targets::{validate_mcu, MCU_ALLOWLIST, TARGET_KINDS};

#[cfg(feature = "build")]
pub use builder::compile;

/// The pinned `wickra-backtest` version written into generated projects and
/// recorded in the manifest.
pub const BACKTEST_DEP: &str = "0.1";

/// The crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
