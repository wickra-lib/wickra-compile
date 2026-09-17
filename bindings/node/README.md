<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/npm.svg)](https://www.npmjs.com/package/wickra-compile)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — Node.js

---

**Part of the [Wickra ecosystem](#ecosystem): — for Node.js. `npm install wickra-compile` — prebuilt native binary, no system dependencies.**

Node.js bindings for [`wickra-compile`](https://github.com/wickra-lib/wickra-compile),
powered by Rust via [napi-rs](https://napi.rs/): compile a Wickra strategy spec
into a standalone deployable and get a **deterministic manifest** — byte-identical
to every other language binding.

## Install

```bash
npm install wickra-compile
```

The native addon ships as a prebuilt binary per platform (Linux, macOS,
Windows — x64 and arm64), selected automatically through optional
dependencies. There is nothing to compile.

Requires Node.js >= 22. The correct native binary is installed automatically as
an optional dependency for your platform.

## Quick start

```js
const { Compiler } = require("wickra-compile");

const spec = {
  strategy: { symbol: "BTCUSDT", timeframe: "1h",
    indicators: { fast: { type: "Sma", params: [10] }, slow: { type: "Sma", params: [30] } },
    entry: { cross_above: ["fast", "slow"] },
    exit:  { cross_below: ["fast", "slow"] },
    sizing: { type: "fixed_qty", qty: 1 } },
  target: { kind: "wasm" }, opt_level: "size",
};

const c = new Compiler();
const out = JSON.parse(c.command(JSON.stringify({ cmd: "compile", dry_run: true, spec })));
console.log(out.manifest.project_hash);  // deterministic across runs and languages
```

`command` mirrors `Compiler::command_json`: the commands are `compile`,
`targets`, `version`, `artifact_bytes` and `reset`. Domain errors come back
in-band as `{ ok: false, error: ... }`. A `compile` with `dry_run: true` needs no
toolchain; `dry_run: false` invokes `cargo`.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of napi-rs, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/node/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/node)

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
