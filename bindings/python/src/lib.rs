//! Python bindings for `wickra-compile`, exposed under the `wickra_compile`
//! package.
//!
//! Thin glue over the codegen core's command surface: construct a
//! [`Compiler`], drive it with a command JSON and read back the response JSON.
//! The same command protocol crosses every binding, so a Python front-end drives
//! the exact same core — and gets the byte-identical manifest — as the CLI.

// PyO3 protocol methods take `self` by value/ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::prelude::*;

use compile_core::Compiler;

/// A compiler driven by JSON commands.
///
/// `unsendable`: the handle holds a stateful build cache, so it is bound to the
/// thread that created it.
#[pyclass(name = "Compiler", unsendable)]
struct PyCompiler {
    inner: Compiler,
}

#[pymethods]
impl PyCompiler {
    /// Construct a compiler handle.
    #[new]
    fn new() -> Self {
        Self {
            inner: Compiler::new(),
        }
    }

    /// Apply a command JSON and return the response JSON.
    ///
    /// Domain errors are reported in-band as `{"ok": false, "error": ...}` JSON,
    /// not raised.
    fn command(&mut self, cmd_json: &str) -> String {
        self.inner.command_json(cmd_json)
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        compile_core::version()
    }
}

/// The native module (`wickra_compile._wickra_compile`).
#[pymodule]
fn _wickra_compile(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyCompiler>()?;
    Ok(())
}
