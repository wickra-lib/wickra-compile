# Targets

A `CompileSpec` picks one of three artifact shapes with its `target` field.

## `wasm`

```json
{ "kind": "wasm" }
```

A WebAssembly module. The generated project builds with
`--target wasm32-unknown-unknown`. Install the target once:

```bash
rustup target add wasm32-unknown-unknown
```

## `binary`

```json
{ "kind": "binary" }
```

A self-contained native executable for the host platform. No extra target
toolchain is needed beyond a working Rust install.

## `no_std`

```json
{ "kind": "no_std", "mcu": "thumbv7em-none-eabihf" }
```

A bare-metal `no_std` artifact for a microcontroller. The `mcu` triple is
required and must be one of the allowlisted targets below. Install it with
`rustup target add <triple>`.

### MCU allowlist

The compiler accepts exactly these seven MCU triples:

| Triple | Typical cores |
|--------|---------------|
| `thumbv6m-none-eabi` | Cortex-M0 / M0+ |
| `thumbv7m-none-eabi` | Cortex-M3 |
| `thumbv7em-none-eabi` | Cortex-M4 / M7 (soft float) |
| `thumbv7em-none-eabihf` | Cortex-M4F / M7F (hard float) |
| `thumbv8m.main-none-eabi` | Cortex-M33 (soft float) |
| `thumbv8m.main-none-eabihf` | Cortex-M33F (hard float) |
| `riscv32imac-unknown-none-elf` | RV32IMAC |

Any other triple is rejected during validation. The allowlist is the single
source of truth (`MCU_ALLOWLIST` in `compile-core`); the `targets` command
returns it at runtime.

## Overriding the target

The CLI can override the spec's target without editing the file:

```bash
wickra-compile --spec strategy.json --target no-std --mcu thumbv7em-none-eabihf
wickra-compile --spec strategy.json --target wasm --opt size
```

`--mcu` is required with `--target no-std` and rejected otherwise.

## The manifest is target-aware

The chosen target and optimisation level are part of the manifest, so the same
strategy compiled for `wasm` and for `binary` produces two distinct, each
reproducible, manifests.

## See also

- [COMPILESPEC.md](COMPILESPEC.md) · [DETERMINISM.md](DETERMINISM.md)
