# Determinism

The **manifest** is Wickra Compile's golden guarantee: the same `CompileSpec`
plus target produces a byte-identical manifest on every run, on every OS, and
through every one of the ten language bindings. This document explains why.

## What the manifest contains

```json
{
  "compiler_version": "0.1.0",
  "target": { "kind": "wasm" },
  "opt_level": "release",
  "crate_name": "demo",
  "spec_hash": "…",
  "backtest_dep": "0.1",
  "files": [ { "path": "Cargo.toml", "sha256": "…", "bytes_len": 123 }, … ],
  "project_hash": "…"
}
```

Every field is a pure function of the input. There is **no** timestamp, no
absolute path, no RNG, no host detail anywhere in it.

## How reproducibility is enforced

- **Canonical JSON.** Before hashing, the strategy is serialised through a
  canonicaliser that recursively sorts object keys, so two specs that differ
  only in key order hash identically.
- **Stable ordering.** The generated file set is stored in a `BTreeMap`, so the
  `files` array is always in sorted path order, and `project_hash` folds the
  files in that order.
- **No ambient state.** Codegen reads only the spec. There is no clock, no
  random seed, no environment lookup on the golden path.
- **Version-pinned dependency.** The embedded engine dependency is recorded as
  `backtest_dep` so a manifest is tied to the engine contract it was generated
  against.

## Cross-language byte-equality

The whole compiler lives once, in `compile-core`. Every binding calls
`command_json` and returns its result **string verbatim** — no per-language JSON
re-encode, no deep-equal, no hash reformat. Because the response is the same
bytes and the manifest inside it carries no ambient state, `project_hash` is
identical in Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R. The golden
corpus under `golden/` pins this: each blessed `expected/*.json` is the manifest
`--manifest` prints, and the cross-language test asserts every binding reproduces
it.

## What is *not* guaranteed

The compiled **artifact bytes** (the `.wasm`, the ELF, the executable) are only
reproducible with a reproducible-build toolchain — pinned compiler, sorted
inputs, no embedded build paths. That is best-effort and deliberately separate
from the manifest guarantee: the manifest proves *what* was generated;
reproducible builds prove the *compiler* turned it into the same bytes.

## See also

- [ARCHITECTURE.md](ARCHITECTURE.md) · [TEMPLATES.md](TEMPLATES.md) ·
  `golden/README.md`
