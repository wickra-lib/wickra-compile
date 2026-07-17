//! Command-line arguments for `wickra-compile`.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use compile_core::OptLevel;

/// Compile a Wickra strategy spec into a standalone deployable.
#[derive(Parser, Debug)]
#[command(name = "wickra-compile", version, about)]
pub struct Args {
    /// Path to the spec (`.json` or `.toml`).
    #[arg(long, value_name = "PATH")]
    pub spec: PathBuf,

    /// Override the target artifact (else the spec's `target` is used).
    #[arg(long, value_enum)]
    pub target: Option<TargetArg>,

    /// The MCU target triple (required with `--target no_std`).
    #[arg(long, value_name = "TRIPLE")]
    pub mcu: Option<String>,

    /// Override the optimisation level (else the spec's `opt_level`).
    #[arg(long, value_enum)]
    pub opt: Option<OptArg>,

    /// Output directory for the generated project.
    #[arg(long, value_name = "DIR", default_value = "_out")]
    pub out: PathBuf,

    /// Generate the project and manifest only; do not invoke `cargo`.
    #[arg(long)]
    pub dry_run: bool,

    /// Print the manifest JSON to stdout and nothing else (implies no build).
    #[arg(long)]
    pub manifest: bool,
}

/// The `--target` choices.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum TargetArg {
    /// A WASM module.
    Wasm,
    /// A self-contained binary.
    Binary,
    /// A bare-metal `no_std` artifact (needs `--mcu`).
    NoStd,
}

/// The `--opt` choices.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum OptArg {
    /// Debug (`dev`) profile.
    Debug,
    /// Release profile.
    Release,
    /// Size-optimised release.
    Size,
}

impl From<OptArg> for OptLevel {
    fn from(value: OptArg) -> Self {
        match value {
            OptArg::Debug => OptLevel::Debug,
            OptArg::Release => OptLevel::Release,
            OptArg::Size => OptLevel::Size,
        }
    }
}
