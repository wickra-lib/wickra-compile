//! The error type for codegen and (optionally) building.

/// Errors produced while validating a spec, generating a project, or building.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON / TOML document could not be parsed.
    #[error("parse: {0}")]
    Parse(String),
    /// The spec is structurally invalid: the strategy is not a valid
    /// `StrategySpec`, the crate name is not a valid identifier, or referenced
    /// embedded data is missing.
    #[error("bad spec: {0}")]
    BadSpec(String),
    /// A `no_std` target named an MCU triple outside the allowlist.
    #[error("bad target: {0}")]
    BadTarget(String),
    /// A template failed to render.
    #[error("codegen: {0}")]
    Codegen(String),
    /// The `cargo` / `wasm-pack` build exited non-zero (a sanitized stderr tail).
    #[error("build: {0}")]
    Build(String),
    /// A filesystem operation failed.
    #[error("io: {0}")]
    Io(String),
}

/// The crate result alias.
pub type Result<T> = core::result::Result<T, Error>;
