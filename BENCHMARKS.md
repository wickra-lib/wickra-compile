# Benchmarks

Numbers land here once the Criterion suite (`crates/compile-bench`) is wired up
in P-COMP-6.5. The tracked operations:

| Operation | What it measures |
|-----------|------------------|
| `spec_hash` | Canonical serialisation + SHA-256 of a `StrategySpec`. |
| `generate` | Full codegen: validate → render → manifest (no `cargo`). |
| `project_hash` | Hashing the generated file set in path order. |

Code generation is cheap (no compilation); the optional `cargo build` step
dominates end-to-end time and is measured separately, not in the microbenchmarks.

_Placeholder — replaced with measured figures during P-COMP-6._
