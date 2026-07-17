# Threat model

Wickra Compile is a **code generator** that can invoke a compiler on the code it
generates. That makes its threat model thicker than a pure calculation library:
the input is a spec that becomes source, and an optional build step runs a
toolchain. This document is deliberately explicit about that.

## Assets

- **The user's `CompileSpec`** (the strategy, target, embedded data).
- **The embedded templates** (the generated project skeleton).
- **The build toolchain** (`cargo`, target compilers) when the `build` feature
  is used.
- **The deterministic manifest** — its integrity is the product's core promise.

## Actors

- A user compiling their own trusted spec (the normal case).
- A user compiling a **third-party spec** they did not write (elevated risk).
- A downstream consumer trusting a published manifest.

## Threats and mitigations

- **Template injection.** A malicious spec value could try to inject Rust or
  template syntax into the generated project. *Mitigation:* the spec is embedded
  as **data**, never rendered through the template engine; only scalar,
  **validated** values (crate name, version, opt flags, target triple) reach the
  templates, and HTML/template escaping is not relied on for safety.
- **`cargo` invocation as code execution.** Generating a project and running
  `cargo build` on it executes `build.rs` and proc-macros from the dependency
  tree. *Mitigation:* codegen itself does **no** network or build; the build step
  is opt-in (`build` feature / CLI without `--dry-run`), documented as
  "trusted specs only", and invokes `cargo` by **argv, never `sh -c`**, so spec
  values cannot break out into a shell.
- **MCU-triple injection.** A `no_std` target triple flows into `cargo` args.
  *Mitigation:* the triple must be a member of a fixed allowlist; anything else
  is rejected before any process is spawned.
- **Space / path argv injection.** Paths and names flow into process arguments.
  *Mitigation:* argv (a `Vec<String>`), never a concatenated shell command;
  crate names are regex-constrained.
- **Reproducible-build drift.** Two builds of the same generated project may
  differ in binary bytes across toolchains. *Mitigation:* the **manifest** is the
  determinism guarantee (byte-identical across runs and languages); binary-byte
  reproducibility is documented as best-effort and out of the golden scope.

## Out of scope

- The security of `wickra-backtest` and the wider dependency tree (covered by
  `cargo-deny` / `osv-scanner` and their own advisories).
- Sandboxing the build step's file-system or network access — the operator is
  responsible for running untrusted-spec builds in a sandbox.

## Reporting

See [SECURITY.md](SECURITY.md).
