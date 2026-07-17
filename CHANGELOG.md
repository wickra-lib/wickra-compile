# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`compile-core`, `compile-cli`, `compile-bench`) with the language-binding
  crates, and the `wickra-backtest` git dependency (the `StrategySpec` source of
  truth embedded into generated projects).
- `compile-core`: the codegen library — the `CompileSpec` model, canonical JSON,
  the codegen templates, spec/project hashing, the deterministic `Manifest`, and
  the optional `cargo` build driver behind the `build` feature.
- `wickra-compile` CLI: load a spec, override `--target` / `--opt` / `--mcu`,
  and `--manifest` / `--dry-run` / build.
- Ten language bindings — Rust, Python, Node.js and WASM natively, plus C, C++,
  C#, Go, Java and R over the C ABI hub — each forwarding `command_json`
  verbatim so the manifest is byte-identical everywhere.
- Golden corpus: candle data, canonical `CompileSpec`s and blessed `Manifest`
  JSON under `golden/`, with a cross-language conformance test.
- Tests: conformance, golden, canonical-determinism and proptest suites; a
  detached `fuzz` workspace (spec parse, canonical hash, codegen, target parse);
  Criterion micro-benchmarks in `compile-bench`.
- Runnable examples for all ten languages under `examples/`.
- CI/CD: `ci.yml` (fmt, clippy on both feature sets, the 3-OS × 2-feature test
  matrix, MSRV, cargo-deny, CLI smoke, C ABI, the ten bindings, coverage,
  codegen-golden, and a schedule-gated build-targets smoke), plus CodeQL,
  Scorecard, zizmor, link-check, nightly benchmark, metadata audit, and a
  release workflow.
- Documentation: `docs/ARCHITECTURE.md`, `COMPILESPEC.md`, `TARGETS.md`,
  `DETERMINISM.md`, `TEMPLATES.md`, `Cookbook.md`, per-binding READMEs, and
  measured `BENCHMARKS.md` figures.

[Unreleased]: https://github.com/wickra-lib/wickra-compile/commits/main
