//! WebAssembly bindings for `wickra-compile` (wasm-bindgen).
//!
//! Drive a `Compiler` with JSON commands in the browser: construct a handle,
//! call `command` with a request JSON and read back the response JSON. The
//! command protocol crosses every binding, so a browser front-end gets the
//! byte-identical manifest as the native CLI.
//!
//! `compile-core` is pulled in with `default-features = false`, so its `build`
//! feature — which shells out to `cargo` — is off: a browser cannot run a
//! toolchain. Pure codegen and the deterministic manifest (`compile` with
//! `dry_run: true`, plus `targets` and `version`) work fully; a real build
//! (`compile` with `dry_run: false`) comes back as an in-band
//! `{"ok":false,"error":...}` instead.

// `Compiler::new` takes no arguments, so clippy asks for a `Default` impl; a
// wasm-bindgen constructor is the idiomatic entry point here, not `Default`.
#![allow(clippy::new_without_default)]

use wasm_bindgen::prelude::*;

use compile_core::Compiler as CoreCompiler;

/// A compiler driven by JSON commands.
#[wasm_bindgen]
pub struct Compiler {
    inner: CoreCompiler,
}

#[wasm_bindgen]
impl Compiler {
    /// Construct a compiler handle.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Compiler {
        Self {
            inner: CoreCompiler::new(),
        }
    }

    /// Apply a command envelope (`{"cmd":"...", ...}`) and return the response
    /// JSON. Domain errors are reported in-band as `{"ok":false,"error":...}`,
    /// not thrown.
    #[must_use]
    pub fn command(&mut self, cmd_json: &str) -> String {
        self.inner.command_json(cmd_json)
    }

    /// The crate version.
    #[wasm_bindgen(js_name = version)]
    #[must_use]
    pub fn instance_version(&self) -> String {
        compile_core::version().to_owned()
    }
}

/// The library version.
#[wasm_bindgen]
#[must_use]
pub fn version() -> String {
    compile_core::version().to_owned()
}
