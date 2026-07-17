# Wickra Compile — R

R bindings for the Wickra strategy compiler over its C ABI hub via R's `.Call`
interface. A compiler handle is driven over a JSON boundary, so the manifest it
produces is byte-identical to every other Wickra Compile binding.

## Requirements

The native C ABI library and its header, provided out-of-tree via two
environment variables read by the `Makevars`:

- `WKCOMPILE_INC` — the directory holding `wickra_compile.h`
  (`bindings/c/include/`).
- `WKCOMPILE_LIB` — the directory holding the built C ABI library
  (`target/release/` after `cargo build -p compile-c --release`).

At run time the loader finds the shared library via `PATH` (Windows) or
`LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` (Linux / macOS).

## Install

```bash
WKCOMPILE_INC=../c/include WKCOMPILE_LIB=../../target/release \
  R CMD INSTALL .
```

## Usage

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

## Surface

- **`wkcompile_new()`** — construct a compiler handle (an external pointer; a
  finalizer frees it).
- **`wkcompile_command(compiler, cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`wkcompile_version()`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as an R error.

## Determinism

The whole compiler lives once in the Rust core; this binding forwards its JSON
verbatim, so a given spec produces the byte-identical manifest here and in every
other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-compile>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
