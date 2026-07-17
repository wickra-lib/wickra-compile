//! Codegen core for `wickra-compile`.
//!
//! Scaffold stub; the spec model, canonical hashing, template set, manifest and
//! `cargo` invocation land in P-COMP-1.

/// Returns the crate version string.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_non_empty() {
        assert!(!super::version().is_empty());
    }
}
