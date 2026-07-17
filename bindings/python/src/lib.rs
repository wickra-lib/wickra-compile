//! python binding for `wickra-compile`.
//!
//! Scaffold stub; the real binding surface (handle + `command_json` + `version`)
//! lands in P-COMP-3.

/// Returns the codegen-core version string.
#[must_use]
pub fn version() -> &'static str {
    compile_core::version()
}
