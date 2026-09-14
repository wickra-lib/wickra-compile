# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-09-14

### Fixed

- **The generated `no_std` project could not build.** Its Cargo template
  declared `wickra-backtest`, a std crate (rayon, crossbeam, the loaders), for
  a `thumbv*` target, and the nightly cross-compile smoke had failed on `can't
  find crate for std` since it was first scheduled. The no_std artifact embeds
  the spec text and parses nothing, so its project now depends on nothing at
  all, is allocator-free (`&'static str`, a length and a pointer for a C
  caller) and carries the `#[panic_handler]` a static library for bare metal
  has to provide. A generated project also declares its own `[workspace]`, so
  one written inside another workspace tree is not claimed by it. Both the
  wasm and the binary projects take `wickra-backtest` from crates.io
  (`0.1.5`) instead of git. The golden manifests are re-blessed for the new
  templates; all three targets cross-compile for real.
- **Canonical JSON round-trips exactly.** The fuzz target over the
  canonicaliser -- which the family-level CI runs for the first time -- found a
  number the canonical form printed as `-9.299999999999999e+30` and re-parsed
  as `-9.3e+30`: serde_json's default float parser is fast and may be a ULP
  off, so a spec hash could depend on which neighbour the parser landed on.
  `float_roundtrip` is on; the canonical form is a fixed point again and the
  golden hashes are unchanged.
- **Operating-mode equivalence is tested in the core and in every binding.**
  The manifest a dry run describes must be the manifest a real build builds:
  `operating_modes.rs` compiles the no_std project for real (no dependencies,
  seconds, the `thumbv7em-none-eabihf` target only) and requires the built
  artifact's manifest to equal the dry-run one and the blessed golden; each
  binding that can reach `cargo` checks the same at its own boundary, and the
  WASM build, which carries no toolchain, checks that a spec in any key order
  yields the same manifest. The C suite (`examples/c/golden_test.c`, wired into
  ctest with a CMake-globbed spec list) checks golden parity and the real
  build without a JSON library. The golden tests fail on a missing corpus
  instead of skipping.
- **The binding crates carry the family's names.** `compile-c`, `compile-node`,
  `compile-python` and `compile-wasm` are `wickra-compile-c`,
  `wickra-compile-node`, `wickra-compile-python` and `wickra-compile-wasm`;
  the wasm module is `wickra_compile_wasm.js`.
- **A C++ hull.** `bindings/c/include/wickra_compile.hpp` -- header-only
  C++17, owns the handle, runs the length protocol, returns artifact bytes as
  a vector, turns a negative return into an exception -- ships beside the C
  header and in the release archive; `examples/c/compile.cpp` builds against
  it.
- **Every dependency comes from crates.io, and the engine pin is exact.**
  `wickra-backtest` was a git dependency, which `cargo publish` refuses; it is
  the registry crate at `=0.1.5`, as the released siblings pin it, and
  `deny.toml` no longer allows git sources. `examples/rust` is a workspace
  member and `examples/node` installs the binding from the checkout rather
  than an unpublished registry version.
- **The release front in the family shape.** The tag guard, the version gate,
  idempotent publishing of both crates with CycloneDX SBOMs, the C ABI
  archives with the header and the hull, a Maven Central deploy that skips a
  version already on Central and waits as long as Central takes, the jar
  uploaded for the provenance job to attest, provenance over the nupkg, jar
  and C ABI archives, the Go mirror that builds before it pushes, and a
  `workflow_dispatch` that publishes nothing. The pom carries the release
  profile (sources, javadoc, GPG, the publishing plugin), `<scm>` and
  `<developers>` Central requires.
- **The Python 3.9 CI row runs without pytest.** pytest 9.x requires 3.10, so
  that row could only pin 8.4.2, below the fix for GHSA-6w46-j5rx-g56g with no
  backport. The 3.9 lock carries maturin only, and the row runs the same test
  modules through `bindings/python/tests/run_without_pytest.py`; 3.10 and up
  run them under pytest as before.
- **The R package builds the family way.** `configure` downloads the C ABI
  release archive (or builds it from the tag's source on r-universe's
  WebAssembly image), `configure.win` picks the architecture from
  `R.version$arch`, the exported functions carry generated `man/` pages, a
  shipped smoke test runs inside the tarball, `.Rbuildignore` is regex-safe,
  and `DESCRIPTION` states the R floor.
- CI in the family shape: the binding-surface, links and semver jobs, the
  wheel container smoke, osv-scanner, an Examples job that holds every example
  to the version line, a WASM demo page (`examples/wasm/compile.html`) whose
  module is parse-checked, the four fuzz targets run for a short smoke, the
  bare-metal target installed wherever the operating-mode test builds, CodeQL
  over C#, Java and C/C++ with a config that keeps generated code out,
  timeouts on every job, patch-level pins, Dependabot over every manifest (the
  fuzz crate and the Go and Node examples included), the five
  repository-check scripts, `update-lockfiles.sh`, the detailed issue and PR
  templates, actionlint, CodSpeed with `criterion` aliased to it, zizmor's
  `self-repository` policy, and docs.rs metadata on both crates.
- Licence texts travel with every published package (`LICENSES/`, copies in
  each crate and the Python and npm packages; the binding READMEs link the
  texts absolutely); the README opens with the quickstart, gains the
  ecosystem list and states the toolchain floors the manifests declare;
  `SECURITY.md` names the first release.
- **The core crate carried a name the release could not upload.**
  `compile-core` is outside the org's crates.io token scope, which creates new
  crates under the `wickra-` prefix only; `cargo publish` on it returns 403 at
  upload while `--dry-run` passes, and because the publish jobs run in
  parallel the release would have landed on PyPI, npm, NuGet, Maven Central
  and the Go mirror without ever reaching crates.io. The core is now
  `wickra-compile-core`, the shape of every released sibling. The directory
  keeps its name; only the package and the `wickra_compile_core` path moved.
  The same audit ran across the family (xray paid for this with its first
  tag).

- **`release.yml` copied the CLI's SBOM from a directory that does not
  exist.** It read `crates/synth-cli/wickra-compile.cdx.json`, a path carried
  over from the repository it was templated from; the crate lives in
  `crates/compile-cli/`. The `cp` sits after both uploads, so the job would
  have failed with the crates already published and no `.crate` or SBOM
  attached to the GitHub Release.

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`wickra-compile-core`, `compile-cli`, `compile-bench`) with the language-binding
  crates, and the `wickra-backtest` git dependency (the `StrategySpec` source of
  truth embedded into generated projects).
- `wickra-compile-core`: the codegen library — the `CompileSpec` model, canonical JSON,
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

[Unreleased]: https://github.com/wickra-lib/wickra-compile/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/wickra-lib/wickra-compile/releases/tag/v0.1.0
