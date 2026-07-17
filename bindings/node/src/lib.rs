//! Node.js bindings for `wickra-compile` via napi-rs.
//!
//! A `Compiler` is driven by JSON commands: `command` takes a request JSON and
//! returns the response JSON, so Node drives the exact same byte-identical
//! surface — and gets the byte-identical manifest — as every other binding.

use napi_derive::napi;

/// A compiler driven by JSON commands.
#[napi]
pub struct Compiler(compile_core::Compiler);

#[napi]
impl Compiler {
    /// Construct a compiler handle.
    #[napi(constructor)]
    pub fn new() -> Self {
        Compiler(compile_core::Compiler::new())
    }

    /// Apply a command envelope (`{"cmd":"...", ...}`) and return the response
    /// JSON. Domain errors are reported in-band as `{"ok":false,"error":...}`.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn command(&mut self, cmd_json: String) -> String {
        self.0.command_json(&cmd_json)
    }

    /// The crate version.
    #[napi]
    pub fn version(&self) -> &'static str {
        compile_core::version()
    }
}
