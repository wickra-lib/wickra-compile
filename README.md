<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-compile)
[![CI](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![CodeQL](https://github.com/wickra-lib/wickra-compile/actions/workflows/codeql.yml/badge.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/codeql.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-compile)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Deterministic manifest](https://img.shields.io/badge/manifest-deterministic-3b82f6)](#determinism)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

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

Early development (0.1.0, unreleased). The codegen core, the reference CLI, the
ten-language binding surface, the golden corpus and the full CI matrix are in
place; the first published release is still pending.

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

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — the crates and the codegen pipeline.
- [COMPILESPEC.md](docs/COMPILESPEC.md) — the input schema.
- [TARGETS.md](docs/TARGETS.md) — WASM / binary / `no_std` and the MCU allowlist.
- [DETERMINISM.md](docs/DETERMINISM.md) — why the manifest is reproducible.
- [TEMPLATES.md](docs/TEMPLATES.md) — codegen and injection safety.
- [Cookbook.md](docs/Cookbook.md) — practical recipes.

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

## Building from source

```bash
cargo build
cargo test
```

## Benchmarks

Codegen is pure data templating and hashing — tens of microseconds, no
compilation. See [BENCHMARKS.md](BENCHMARKS.md); reproduce with
`cargo bench -p compile-bench`.

## Requirements

- Rust 1.86+ (MSRV). Building generated artifacts additionally needs the target
  toolchain (`wasm32-unknown-unknown` for WASM, a `thumbv*` target for `no_std`).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). The
compiler generates code and can invoke `cargo` on it — run it only on trusted
specs.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Disclaimer

Wickra Compile is a code-generation tool, provided "as is" without warranty of
any kind. It generates projects and can invoke `cargo` to build them — run it
only on specs you trust. Nothing here is financial advice; compiled strategies
are your responsibility, and trading carries risk of loss.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
