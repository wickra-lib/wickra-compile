<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/npm.svg)](https://www.npmjs.com/package/wickra-compile-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — WASM

---

**Part of the [Wickra ecosystem](#ecosystem): — for WASM. `npm install wickra-compile-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WebAssembly bindings for the Wickra strategy compiler, built from Rust with
[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `Compiler` is
driven by command JSONs over a JSON boundary, so a browser front-end runs
against the exact same core — and gets the byte-identical manifest — as every
other Wickra Compile binding.

## Install

```bash
npm install wickra-compile-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Quick start

```js
import init, { Compiler } from "./pkg/wickra_compile_wasm.js";

await init();

const compiler = new Compiler();
const out = JSON.parse(
  compiler.command(
    JSON.stringify({
      cmd: "compile",
      dry_run: true,
      spec: {
        strategy: {
          symbol: "x",
          timeframe: "1h",
          indicators: { f: { type: "Ema", params: [3] } },
          entry: { cross_above: ["f", "f"] },
          exit: { cross_below: ["f", "f"] },
          sizing: { type: "fixed_qty", qty: 1 },
        },
        target: { kind: "wasm" },
        crate_name: "demo",
      },
    }),
  ),
);
console.log(out.project_hash); // the deterministic manifest hash
```

### Surface

- **`new Compiler()`** — construct a compiler handle.
- **`compiler.command(cmdJson) -> string`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`compiler.version() -> string`** and the module-level **`version()`** — the
  crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not thrown.

### No toolchain in the browser

This binding links `wickra-compile-core` with its `build` feature **off** — a browser
sandbox cannot shell out to `cargo`. Pure codegen and the deterministic manifest
(`compile` with `dry_run: true`, plus `targets` and `version`) work fully. A real
build (`compile` with `dry_run: false`) returns an in-band error, because there
is no toolchain to invoke. The generated files and the manifest are identical to
the native run — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/wasm)

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
