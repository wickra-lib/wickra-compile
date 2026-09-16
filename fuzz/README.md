# Fuzzing Wickra Compile

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra Compile. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `spec_parse` | The spec-parsing surface: arbitrary bytes are parsed as a `CompileSpec` (JSON). |
| `canonical_hash` | The canonicalisation + hashing surface: arbitrary JSON is canonicalised and hashed without panicking, and canonicalisation is idempotent — feeding a canonicalised value back through must produce the identical string, which is the property the deterministic hashing relies on. |
| `codegen` | The codegen surface: any spec that parses and validates is generated without panicking, and every generated path is relative and free of parent references or absolute prefixes — the path-traversal guarantee from the threat model. |
| `target_parse` | The target-parsing + MCU-validation surface: arbitrary JSON parsed as a `Target` never panics, round-trips when it parses, and `validate_mcu` on a `no_std` triple is total (only allowlisted triples are accepted). |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu canonical_hash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu codegen
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu target_parse
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
