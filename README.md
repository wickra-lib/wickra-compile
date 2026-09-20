<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![Built on Wickra](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/built-on.svg)](https://github.com/wickra-lib/wickra)
[![Status](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/status.svg)](https://github.com/wickra-lib/wickra-compile)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codeql.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/release.svg)](https://github.com/wickra-lib/wickra-compile/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/crates.svg)](https://crates.io/crates/wickra-compile)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/pypi.svg)](https://pypi.org/project/wickra-compile/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/npm.svg)](https://www.npmjs.com/package/wickra-compile)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/nuget.svg)](https://www.nuget.org/packages/Wickra.Compile)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-compile)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-compile-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-compile)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/provenance.svg)](https://github.com/wickra-lib/wickra-compile/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/docs.svg)](https://compile.wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/verified.svg)](golden/)
[![Deterministic manifest](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/manifest.svg)](#determinism)

---

**Compile a strategy spec into a standalone deployable: a WASM module, a
self-contained binary, or a `no_std` artifact for microcontrollers. Write once
as data, deploy anywhere.**

> **▶ Live demos:** the backtester compiled to WebAssembly, an equity curve building bar by bar — **[backtest-live.wickra.org](https://backtest-live.wickra.org)**;
> one StrategySpec side by side in Python, Rust, JS and Go — **[playground.wickra.org](https://playground.wickra.org)**;
> all 514 indicators of the core over a real Binance feed — **[live.wickra.org](https://live.wickra.org)**. Zero backend, all of them.

**Part of the [Wickra ecosystem](#ecosystem):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).
same [`StrategySpec`](https://github.com/wickra-lib/wickra-backtest) that
`wickra-backtest` runs and emits a self-contained project that embeds the spec
and calls the Wickra engine — no interpreter, no runtime spec parsing.

Wickra Compile is the **output** side of strategy authoring: instead of wrestling
a Pine-like input language, you write your strategy as a `StrategySpec` (data),
and the compiler generates a standalone Rust project that embeds it verbatim and
targets WASM, a native binary, or bare metal. The generated **manifest** — the
list of files with their hashes plus the canonical spec hash — is
**byte-identical across all ten language bindings** and reproducible across runs.

```bash
# Print the deterministic manifest for a strategy spec — no toolchain needed.
cargo run -p wickra-compile -- --spec golden/specs/sma_cross.json --manifest

# Generate the project without building it, then inspect it.
cargo run -p wickra-compile -- --spec golden/specs/sma_cross.json --dry-run --out ./out
```

## Status

**0.1.2 — the current release.** The codegen core, the reference CLI, the
ten-language binding surface, the golden corpus and the full CI matrix are in
place.

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — the crates and the codegen pipeline.
- [COMPILESPEC.md](docs/COMPILESPEC.md) — the input schema.
- [TARGETS.md](docs/TARGETS.md) — WASM / binary / `no_std` and the MCU allowlist.
- [DETERMINISM.md](docs/DETERMINISM.md) — why the manifest is reproducible.
- [TEMPLATES.md](docs/TEMPLATES.md) — codegen and injection safety.
- [Cookbook.md](docs/Cookbook.md) — practical recipes.

## How it works

A `CompileSpec` names a `StrategySpec`, a `target` (`wasm` / `binary` /
`no_std`), an optimisation level and optional embedded data. The codegen core:

1. validates the spec (the strategy round-trips as a `wickra_backtest::StrategySpec`;
   the crate name and MCU triple are checked against an allowlist);
2. renders a generated Rust project from embedded templates (the spec is embedded
   as **data**, never run through the template engine);
3. emits a deterministic **manifest** — every file's SHA-256 and length, the
   canonical spec hash, and the project hash — in stable path order;
4. optionally invokes `cargo` (argv, never a shell) to build the artifact.

## Determinism

The manifest is the golden moat: `BTreeMap` everywhere in the output path, stable
path-sorted file lists, no RNG, no timestamps, canonical spec serialisation
before hashing. The same `CompileSpec` + `target` produces a byte-identical
manifest on every run and in every language binding. (The compiled *binary*
bytes are only reproducible with a reproducible-build toolchain — that is
best-effort and separate from the manifest guarantee.)

## Quickstart

```bash
# Print the deterministic manifest for a strategy spec — no toolchain needed.
wickra-compile --spec golden/specs/sma_cross.json --manifest

# Generate the project without building it, then inspect it.
wickra-compile --spec golden/specs/sma_cross.json --dry-run --out ./out

# Retarget the same spec without editing it.
wickra-compile --spec golden/specs/sma_cross.json --target wasm --opt size
```

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
passes the command string through verbatim, so the manifest they return is
identical.

```python
import json
from wickra_compile import Compiler

spec = open("golden/specs/sma_cross.json").read()
out = json.loads(Compiler().command(f'{{"cmd":"compile","dry_run":true,"spec":{spec}}}'))
print(out["manifest"]["project_hash"])  # identical in every binding
```

See [`examples/`](examples/) for the same program in all ten languages.

## Project layout

```
crates/compile-core   the library: spec, canonical JSON, codegen, manifest
crates/compile-cli    the wickra-compile CLI
crates/compile-bench  criterion micro-benchmarks
bindings/*            ten language surfaces (c, python, node, wasm, csharp, go, java, r)
golden/               specs + blessed manifests (the cross-language corpus)
examples/             one runnable example per language
docs/                 architecture, spec, targets, determinism, templates, cookbook
```

## Building everything from source

```bash
cargo build --workspace
cargo test  --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-compile -- --spec golden/specs/sma_cross.json --manifest
```

Each binding builds from its own directory — see the per-binding READMEs under
`bindings/`. The operating-mode test builds the generated `no_std` project for
real, so `rustup target add thumbv7em-none-eabihf` before `cargo test`.

## Testing

Run the suites with the commands in
[Building everything from source](#building-everything-from-source).

- **`wickra-compile-core`** — unit tests per template and target, canonical
  determinism (a spec in any key order hashes the same), property tests over
  specs and the command envelope, and the operating-mode check: the manifest a
  dry run describes is the manifest a real build builds (the `no_std` project,
  which has no dependencies and compiles in seconds). The golden fixtures in
  `golden/` are the anchor: every spec must produce the same manifest bytes
  here as in every binding.
- **Every binding** asserts the *same* golden manifest and the same
  operating-mode equivalence (the WASM build, which carries no toolchain,
  checks key-order independence instead of a real build). That is the whole
  cross-language claim, so it is checked the same way in each one rather than
  approximated per language: Python with pytest (and a plain runner on 3.9),
  Node with `node --test`, WASM through the nodejs build, C and C++ through
  `ctest`, C# with `dotnet test`, Go with `go test`, Java with JUnit, and R
  with the shipped `tests/smoke.R` plus the repository's `run_tests.R`.
- **Examples** — every example under `examples/` runs in CI and is held to the
  version and the `project_hash` it prints.
- **Real builds** — the nightly build-targets job cross-compiles a generated
  project per target family (wasm32, thumbv7em) with the CLI.
- **Fuzz** — `fuzz/` holds libFuzzer targets over spec and target parsing, the
  canonical hash and the codegen; CI runs each for a short smoke.

## Requirements

- **Rust 1.86+** — the workspace MSRV; the Node binding needs **Rust 1.88**.
  Building a generated artifact additionally needs its target toolchain
  (`wasm32-unknown-unknown` for WASM, a `thumbv*` target for `no_std`).
- **Python 3.9+** — the Python binding.
- **Node 22+** — the Node binding.
- **Go 1.23+** — the Go binding.
- **Java 22+** — the Java binding.
- **R 4.1+** — the R package.
- **.NET 8+** — the C# binding.
- A **C11 / C++17** compiler with CMake 3.15+ for the C and C++ examples.

See each `bindings/<lang>/README.md` for the per-language build and install.

## Benchmarks

Codegen is pure data templating and hashing — tens of microseconds, no
compilation. See [BENCHMARKS.md](BENCHMARKS.md); reproduce with
`cargo bench -p compile-bench`.

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — a liquidation-cascade early-warning radar over 514 streaming indicators
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The
compiler generates code and can invoke `cargo` on it — run it only on trusted
specs.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option. Use it, fork it, modify it, redistribute it — commercially or
not — file issues, send pull requests; all welcome.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer

Wickra Compile is a code-generation tool, provided "as is" without warranty of
any kind. It generates projects and can invoke `cargo` to build them — run it
only on specs you trust. Nothing here is financial advice; compiled strategies
are your responsibility, and trading carries risk of loss.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-compile">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-compile/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-compile/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-compile star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/star-history.svg">
</p>
