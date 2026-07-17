# Roadmap

Wickra Compile is early (0.1.0, unreleased). Direction, not a promise.

## 0.1.0 — first release

- The codegen core: `CompileSpec` → validated, canonical, deterministic manifest.
- Three targets: `wasm` (module), `binary` (self-contained executable), `no_std`
  (bare-metal artifact for an MCU allowlist).
- The `wickra-compile` CLI (generate / build / manifest).
- Ten language bindings over the C ABI hub, all returning the byte-identical
  manifest.
- The golden harness: one `CompileSpec` → one manifest, replayed across every
  binding.

## Later

- Reproducible **binary** builds (pinned toolchain, `--remap-path-prefix`), so the
  artifact bytes join the manifest in the determinism guarantee.
- More MCU triples in the allowlist as they are verified.
- A `wasm-pack`-free WASM path and size-optimised `no_std` templates.
- Optional signing / provenance attestation of the emitted artifact.
