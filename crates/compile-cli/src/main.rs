//! `wickra-compile` — the reference codegen CLI.
//!
//! Loads a strategy spec, optionally overrides the target / optimisation level
//! from flags, and generates (or builds) the standalone project, printing the
//! deterministic manifest.

mod args;
mod run;

use std::process::ExitCode;

use clap::Parser;

use crate::args::Args;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
