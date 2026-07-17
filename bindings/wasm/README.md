# Wickra Compile — WASM

WebAssembly bindings for the Wickra strategy compiler, built from Rust with
[wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A `Compiler` is
driven by command JSONs over a JSON boundary, so a browser front-end runs
against the exact same core — and gets the byte-identical manifest — as every
other Wickra Compile binding.

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

```js
import init, { Compiler } from "./pkg/compile_wasm.js";

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

## Surface

- **`new Compiler()`** — construct a compiler handle.
- **`compiler.command(cmdJson) -> string`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`compiler.version() -> string`** and the module-level **`version()`** — the
  crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not thrown.

## No toolchain in the browser

This binding links `compile-core` with its `build` feature **off** — a browser
sandbox cannot shell out to `cargo`. Pure codegen and the deterministic manifest
(`compile` with `dry_run: true`, plus `targets` and `version`) work fully. A real
build (`compile` with `dry_run: false`) returns an in-band error, because there
is no toolchain to invoke. The generated files and the manifest are identical to
the native run — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-compile>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
