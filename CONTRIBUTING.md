# Contributing to wickra-compile

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-compile>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the `FeatureSpec`, the per-symbol `SymbolState` fold, the
  `FeatureMatrix` and the `build` / `build_batch` entry points — lives in
  `crates/compile-core`. The spec is **data, not code**: a serde struct, so
  the same feature build crosses the C ABI and WASM unchanged.
- The reference consumer is `crates/compile-cli` (the `wickra-compile` binary).
- Every language binding lives under `bindings/<lang>/` and exposes the same
  data-driven surface: a `Compile` handle plus `command(json) -> json` and
  `version`. Bindings must preserve the **golden-parity invariant**: given the
  spec + universe in `golden/{specs,data}/`, the same command produces the
  byte-identical matrix in `golden/expected/`.

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo test --workspace --all-features
cargo test --workspace --no-default-features   # sequential path == parallel path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI on three
operating systems, across both the default (rayon `parallel`) and
`--no-default-features` (sequential / WASM) feature sets — a build must produce a
byte-identical matrix either way.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever** — not in code, tests, fixtures, logs, issues or PRs. The
  compiler reads only local specs and candle data and never uses real keys in tests.
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage).

## Adding a feature or a label

The spec is a serde struct, so extending it means adding a variant, not a
closure. A new feature kind (`indicator` / `price` / `microstructure`) or label
kind (`forward_return` / `triple_barrier`) is added to
`crates/compile-core/src/spec.rs` and handled in the per-symbol fold, with
a serde round-trip test and a golden fixture. Indicators themselves come from the
[Wickra](https://github.com/wickra-lib/wickra) core registry by name and
parameters — no indicator code lives here. See
[docs/FEATURES.md](docs/FEATURES.md) and [docs/LABELS.md](docs/LABELS.md).

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
