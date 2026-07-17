# Architecture

Wickra Compile turns a strategy **spec** (data) into a standalone deployable
project, and reports a deterministic **manifest** describing exactly what it
generated. Everything that matters for reproducibility lives in one Rust crate,
`compile-core`; every other component is a thin shell around it.

## Crates

| Crate | Role |
|-------|------|
| `compile-core` | The library: spec model, canonical JSON, codegen templates, hashing, the manifest, and the optional `cargo` build driver. |
| `compile-cli` (`wickra-compile`) | The reference command-line tool: load a spec, override the target, print the manifest, dry-run, or build. |
| `compile-bench` | Criterion micro-benchmarks for the hot paths (`spec_hash`, `generate`, `project_hash`). |
| `bindings/*` | The ten language surfaces (Rust, Python, Node.js, WASM natively; C, C++, C#, Go, Java, R over the C ABI hub). |

## The pipeline

A `CompileSpec` names a `StrategySpec` (treated as opaque data), a `target`, an
optimisation level, an optional embedded dataset and an optional crate name. The
core walks it in four steps:

1. **Validate.** The strategy round-trips as a `wickra_backtest::StrategySpec`;
   the crate name matches `^[a-z][a-z0-9_]*$`; a `no_std` target's MCU triple is
   checked against the [allowlist](TARGETS.md).
2. **Render.** A generated Rust project is produced from embedded templates. The
   spec is written into the project as **data** (`src/spec.json`, canonicalised),
   never fed through the template engine — see [TEMPLATES.md](TEMPLATES.md).
3. **Manifest.** Every emitted file's SHA-256 and byte length, the canonical
   spec hash, and the overall project hash are collected in stable path order —
   see [DETERMINISM.md](DETERMINISM.md).
4. **Build (optional).** With the `build` feature, the core invokes `cargo`
   (argv, never a shell) on the generated project for the requested target.

Steps 1–3 need no toolchain and are the golden path; step 4 is the only part
that touches the outside world.

## The command boundary

Bindings never re-implement any of this. `Compiler::command_json(&str) -> String`
takes a command envelope (`{"cmd":"compile","dry_run":true,"spec":{…}}`) and
returns a response JSON string. Each binding forwards that string **verbatim**,
which is why the manifest is byte-identical in every language. Commands:
`compile`, `targets`, `version`, `artifact_bytes`, `reset`.

## See also

- [COMPILESPEC.md](COMPILESPEC.md) — the input schema.
- [TARGETS.md](TARGETS.md) — WASM / binary / `no_std` and the MCU allowlist.
- [DETERMINISM.md](DETERMINISM.md) — why the manifest is reproducible.
- [TEMPLATES.md](TEMPLATES.md) — codegen and injection safety.
