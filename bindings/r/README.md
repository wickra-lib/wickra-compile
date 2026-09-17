<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — R

---

**Part of the [Wickra ecosystem](#ecosystem): — for R. `install.packages("wickracompile", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the Wickra strategy compiler over its C ABI hub via R's `.Call`
interface. A compiler handle is driven over a JSON boundary, so the manifest it
produces is byte-identical to every other Wickra Compile binding.

## Install

From r-universe:

```r
install.packages("wickracompile", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

```bash
WKCOMPILE_INC=../c/include WKCOMPILE_LIB=../../target/release \
  R CMD INSTALL .
```

### Requirements

The native C ABI library and its header, provided out-of-tree via two
environment variables read by the `Makevars`:

- `WKCOMPILE_INC` — the directory holding `wickra_compile.h`
  (`bindings/c/include/`).
- `WKCOMPILE_LIB` — the directory holding the built C ABI library
  (`target/release/` after `cargo build -p wickra-compile-c --release`).

At run time the loader finds the shared library via `PATH` (Windows) or
`LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` (Linux / macOS).

## Quick start

```r
library(wickracompile)

compiler <- wkcompile_new()
response <- wkcompile_command(compiler, paste0(
  '{"cmd":"compile","dry_run":true,"spec":{',
  '"strategy":{"symbol":"x","timeframe":"1h",',
  '"indicators":{"f":{"type":"Ema","params":[3]}},',
  '"entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},',
  '"sizing":{"type":"fixed_qty","qty":1}},',
  '"target":{"kind":"wasm"},"crate_name":"demo"}}'
))
cat(response) # response JSON, including manifest.project_hash
```

### Surface

- **`wkcompile_new()`** — construct a compiler handle (an external pointer; a
  finalizer frees it).
- **`wkcompile_command(compiler, cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`wkcompile_version()`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as an R error.

### Determinism

The whole compiler lives once in the Rust core; this binding forwards its JSON
verbatim, so a given spec produces the byte-identical manifest here and in every
other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/r)

- The main project: <https://github.com/wickra-lib/wickra-compile>
- Documentation: <https://wickra.org>

Wickra Compile ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-compile/blob/main/SECURITY.md>.

## Disclaimer

Wickra Compile is a code-generation tool, provided "as is" without warranty of
any kind. It generates projects and can invoke `cargo` to build them — run it
only on specs you trust. Nothing here is financial advice; compiled strategies
are your responsibility, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-MIT) at your option.
