# Architecture

Wickra Compile turns a strategy spec (data) into a standalone deployable project
(code) and, optionally, builds it. The moat is a **deterministic manifest**: the
same `CompileSpec` produces byte-identical metadata on every run and in every
language binding.

## Workspace

| Crate / dir | Role |
|-------------|------|
| `crates/compile-core` | The codegen engine: spec validation, canonical hashing, the embedded template set, manifest generation, and (behind the `build` feature) the `cargo` invocation. |
| `crates/compile-cli` | `wickra-compile` — the reference consumer: load a spec, generate or build, print the manifest. |
| `crates/compile-bench` | Criterion benchmarks for `generate` / `spec_hash` / `project_hash`. |
| `bindings/{c,python,node,wasm}` | The language surface (C ABI hub + native bindings); C, C++, C#, Go, Java and R ride the C ABI. Each passes the `command_json` string through verbatim. |

The binding crates depend on `compile-core` by path (with an explicit version so
they are not wildcard-path dependencies). The `fuzz` crate is a detached
workspace built by `cargo-fuzz` on nightly.

## The codegen pipeline

1. **Validate** (`spec.rs`) — the embedded `strategy` round-trips as a
   `wickra_backtest::StrategySpec`; the resolved crate name matches
   `^[a-z][a-z0-9_]*$`; a `no_std` MCU triple is checked against an allowlist.
2. **Canonicalise + hash** (`canonical.rs`) — the spec is serialised with sorted
   keys and hashed with SHA-256; no timestamps, no nonces.
3. **Render** (`templates/`) — a generated Rust project is rendered from embedded
   templates. The spec is embedded as **data** (raw `include_str!`-style content),
   never passed through the template engine, so a spec cannot inject template
   syntax.
4. **Manifest** (`manifest.rs`) — every generated file's SHA-256 and length, the
   spec hash and the project hash, in stable path order (`BTreeMap`).
5. **Build** (`builder.rs`, `build` feature) — `cargo` is invoked by argv (never
   `sh -c`) with the target's args; WASM may use `wasm-pack`.

## Determinism

`BTreeMap` in every output path, stable path-sorted file lists, canonical spec
serialisation, no RNG and no wall-clock. The manifest is the golden artifact; the
compiled *binary* bytes are only reproducible with a reproducible-build toolchain
(best-effort, separate from the manifest guarantee). See
[THREAT_MODEL.md](THREAT_MODEL.md) for the codegen-specific risks.

## See also

- [THREAT_MODEL.md](THREAT_MODEL.md) — template injection, cargo-as-execution,
  MCU-triple injection, reproducible-build drift.
- [ROADMAP.md](ROADMAP.md).
