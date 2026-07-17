<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-compile)
[![CI](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-compile)
[![Deterministic manifest](https://img.shields.io/badge/manifest-deterministic-3b82f6)](#determinism)

---

# Wickra Compile

**Compile a strategy spec into a standalone deployable: a WASM module, a
self-contained binary, or a `no_std` artifact for microcontrollers. Write once
as data, deploy anywhere.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** it takes the
> same [`StrategySpec`](https://github.com/wickra-lib/wickra-backtest) that
> `wickra-backtest` runs and emits a self-contained project that embeds the spec
> and calls the Wickra engine — no interpreter, no runtime spec parsing.

Wickra Compile is the **output** side of strategy authoring: instead of wrestling
a Pine-like input language, you write your strategy as a `StrategySpec` (data),
and the compiler generates a standalone Rust project that embeds it verbatim and
targets WASM, a native binary, or bare metal. The generated **manifest** — the
list of files with their hashes plus the canonical spec hash — is
**byte-identical across all ten language bindings** and reproducible across runs.

## Status

Early development (0.1.0, unreleased). Built out in phases; this scaffold pins
the repository, governance and supply-chain configuration ahead of the codegen
core, the CLI, the ten language bindings and the golden harness.

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

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
passes the command string through verbatim, so the manifest they return is
identical.

## Building from source

```bash
cargo build
cargo test
```

## Requirements

- Rust 1.86+ (MSRV). Building generated artifacts additionally needs the target
  toolchain (`wasm32-unknown-unknown` for WASM, a `thumbv*` target for `no_std`).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The
compiler generates code and can invoke `cargo` on it — run it only on trusted
specs.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
