//! Target triples, the MCU allowlist, and cargo argument construction.
//!
//! The MCU allowlist is the injection guard: a `no_std` target's `mcu` string is
//! passed to `cargo` as a `--target` argv element (never through a shell), and
//! only allowlisted triples are accepted.

use crate::error::{Error, Result};
use crate::spec::{OptLevel, Target};

/// The bare-metal target triples accepted for [`Target::NoStd`].
pub const MCU_ALLOWLIST: &[&str] = &[
    "thumbv6m-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
    "riscv32imac-unknown-none-elf",
];

/// The high-level target kinds, for the `targets` command.
pub const TARGET_KINDS: &[&str] = &["wasm", "binary", "no_std"];

/// Validate that `mcu` is an allowlisted target triple.
///
/// # Errors
/// Returns [`Error::BadTarget`] for any triple outside [`MCU_ALLOWLIST`].
pub fn validate_mcu(mcu: &str) -> Result<()> {
    if MCU_ALLOWLIST.contains(&mcu) {
        Ok(())
    } else {
        Err(Error::BadTarget(format!(
            "mcu {mcu:?} is not in the allowlist"
        )))
    }
}

impl Target {
    /// The Rust target triple, or `None` for a host binary (no `--target`).
    #[must_use]
    pub fn triple(&self) -> Option<&str> {
        match self {
            Target::Wasm => Some("wasm32-unknown-unknown"),
            Target::Binary => None,
            Target::NoStd { mcu } => Some(mcu.as_str()),
        }
    }

    /// Whether the artifact is produced with `wasm-pack` rather than `cargo`.
    #[must_use]
    pub fn needs_wasm_pack(&self) -> bool {
        matches!(self, Target::Wasm)
    }

    /// The `cargo build` arguments (argv, never a shell string) for this target
    /// at the given optimisation level.
    #[must_use]
    pub fn cargo_args(&self, opt: OptLevel) -> Vec<String> {
        let mut args = vec!["build".to_owned()];
        if let Some(triple) = self.triple() {
            args.push("--target".to_owned());
            args.push(triple.to_owned());
        }
        match opt {
            OptLevel::Debug => {}
            OptLevel::Release | OptLevel::Size => args.push("--release".to_owned()),
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_accepts_known_and_rejects_unknown() {
        assert!(validate_mcu("thumbv7em-none-eabihf").is_ok());
        assert!(matches!(
            validate_mcu("x86_64-unknown-linux-gnu; rm -rf /"),
            Err(Error::BadTarget(_))
        ));
    }

    #[test]
    fn wasm_cargo_args_include_target() {
        let args = Target::Wasm.cargo_args(OptLevel::Size);
        assert_eq!(
            args,
            vec![
                "build".to_owned(),
                "--target".to_owned(),
                "wasm32-unknown-unknown".to_owned(),
                "--release".to_owned()
            ]
        );
    }

    #[test]
    fn binary_debug_has_no_target_or_release() {
        assert_eq!(
            Target::Binary.cargo_args(OptLevel::Debug),
            vec!["build".to_owned()]
        );
    }
}
