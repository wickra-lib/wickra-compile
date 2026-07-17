# Templates & injection safety

The compiler renders a generated Rust project from a small set of embedded
templates. This document explains what is generated and — more importantly — why
a hostile spec cannot inject code.

## What is generated

For a given `CompileSpec` the core emits a minimal, buildable crate:

- `Cargo.toml` — the crate name, the target-appropriate profile, and the pinned
  engine dependency.
- `src/spec.json` — the strategy, written as **canonical JSON data**.
- `src/main.rs` (or `src/lib.rs` for `wasm`) — a fixed driver that loads
  `spec.json` at build time and hands it to the Wickra engine.
- target scaffolding as needed (e.g. a `no_std` entry point for the MCU target).

The exact file set is what the manifest enumerates, in path order.

## The spec is data, never code

This is the security invariant: **the strategy never passes through the template
engine.** Field values from the strategy — symbol names, indicator names,
parameters — are written into `src/spec.json` via the JSON serialiser, which
escapes them. The generated Rust sources are fixed templates with **no** spec
substitution; they read the strategy at build time by deserialising the embedded
JSON, exactly as any binding does at run time.

So a strategy whose `symbol` is `"btc\"; std::process::exit(1); //"` produces a
JSON string with that value escaped inside `spec.json`. It never becomes Rust
tokens. The worst a malformed strategy can do is fail validation.

Two more guards back this up:

- **Crate name allowlist.** `crate_name` (the one identifier that *does* reach
  `Cargo.toml` unquoted) must match `^[a-z][a-z0-9_]*$`. Anything else is
  rejected before rendering.
- **MCU allowlist.** The `no_std` MCU triple is checked against a fixed set (see
  [TARGETS.md](TARGETS.md)) before it reaches any build command.

## Building is argv, never a shell

When the optional `build` feature invokes `cargo`, it does so with an explicit
argument vector — never a shell string — so there is no shell to inject into
either. Codegen and the manifest need no toolchain at all; only this last,
optional step runs `cargo`.

## Trust boundary

Even with these guards, the compiler generates code and can build it. Run it only
on specs you trust, and treat generated projects like any other code you are
about to compile. See [SECURITY.md](../SECURITY.md) and
[THREAT_MODEL.md](../THREAT_MODEL.md).

## See also

- [ARCHITECTURE.md](ARCHITECTURE.md) · [DETERMINISM.md](DETERMINISM.md)
