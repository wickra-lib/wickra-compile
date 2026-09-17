<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/release.svg)](https://github.com/wickra-lib/wickra-compile/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — C / C++

---

**Part of the [Wickra ecosystem](#ecosystem): — for C / C++. `cargo build -p wickra-compile-c --release` — a prebuilt shared/static library plus a generated `wickra_compile.h`, no system dependencies.**

The C ABI hub for `wickra-compile`: a tiny, JSON-shaped surface that every
C-capable language (C, C++, C#, Go, Java, R) links against. It exposes an opaque
handle plus a `command(json) -> json` entry point mirroring
[`wickra_compile_core::Compiler::command_json`].

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-compile/releases) — each archive
has `wickra_compile.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-compile-c --release
# -> target/release/libwickra_compile.{so,dylib} or wickra_compile.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/compile.c`](https://github.com/wickra-lib/wickra-compile/blob/main/examples/c/compile.c) is the runnable example the CI smoke job executes; in full:

```c
/* A runnable C example: compile a strategy spec (dry run) through the
 * wickra-compile C ABI and print the raw JSON manifest. Every language example
 * uses the same spec and prints the same project_hash. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_compile.h"

static const char *CMD =
    "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":"
    "{\"strategy\":{\"symbol\":\"btcusdt\",\"timeframe\":\"1h\","
    "\"indicators\":{\"fast\":{\"type\":\"Sma\",\"params\":[10]},"
    "\"slow\":{\"type\":\"Sma\",\"params\":[30]}},"
    "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
    "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
    "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}},"
    "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}}";

int main(void) {
    WickraCompiler *compiler = wickra_compile_new();
    if (!compiler) {
        fprintf(stderr, "failed to build compiler\n");
        return 1;
    }

    /* Length-out protocol: learn the length, then read into a caller buffer. */
    int len = wickra_compile_command(compiler, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_compile_free(compiler);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_compile_free(compiler);
        return 1;
    }
    wickra_compile_command(compiler, CMD, buf, (size_t)len + 1);

    printf("wickra-compile %s\n", wickra_compile_version());
    printf("output: %s\n", buf);

    free(buf);
    wickra_compile_free(compiler);
    return 0;
}
```

### Surface

```c
#include "wickra_compile.h"

WickraCompiler *h = wickra_compile_new();

/* Two-call length protocol: measure, then write. */
const char *cmd = "{\"cmd\":\"version\"}";
int len = wickra_compile_command(h, cmd, NULL, 0);
char *buf = malloc(len + 1);
wickra_compile_command(h, cmd, buf, len + 1);   /* buf now holds the JSON */

wickra_compile_free(h);
```

- `wickra_compile_new()` / `wickra_compile_free(h)` — create and destroy a handle.
- `wickra_compile_command(h, cmd_json, out, cap)` — apply a command
  (`compile`, `targets`, `version`, `artifact_bytes`, `reset`); returns the
  response length. `out = NULL, cap = 0` queries the length. A response longer
  than `cap` leaves `out` untouched; re-call with `len + 1`. Negative returns:
  `-1` null argument, `-2` non-UTF-8 command, `-3` caught panic. **Domain errors
  are in-band** `{"ok":false,"error":...}` JSON, not negative codes.
- `wickra_compile_artifact_read(h, handle, out, cap)` — copy the bytes of an
  artifact handle (from a prior `artifact_bytes` response) into `out`.
- `wickra_compile_version()` — a static NUL-terminated version string.

### Once-only execution

A `compile` with `dry_run: false` writes a project and runs `cargo`, so the
two-call length protocol must not execute it twice. The handle caches the
response it has computed but not yet delivered, and a repeated call with the
same command bytes reuses it instead of re-executing. Once the response has
been written to a buffer the cache is cleared, so the next identical command
executes freshly.

### C++

`include/wickra_compile.hpp` is a header-only C++17 hull over the same five
functions: `wickra::Compiler` owns and frees the handle, `command` runs the
length-out protocol for you, `artifact_bytes` returns a vector, and a negative
return becomes a `wickra::CompileError`. In-band refusals (`{"ok":false,...}`)
are returned as strings, not thrown. `examples/c/compile.cpp` builds against it.

### Determinism

A `compile` with `dry_run: true` returns the deterministic manifest (byte-identical
across every binding); it needs no toolchain. `dry_run: false` invokes `cargo` and
is available only in the native (non-WASM) bindings.

The header `include/wickra_compile.h` is generated by `cbindgen` and committed;
regenerate it with `cbindgen --config cbindgen.toml --output include/wickra_compile.h`.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/c)

Wickra Compile ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-compile/blob/main/SECURITY.md>.

## Disclaimer

Wickra Compile is a code-generation tool, provided "as is" without warranty of
any kind. It generates projects and can invoke `cargo` to build them — run it
only on specs you trust. Nothing here is financial advice; compiled strategies
are your responsibility, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-MIT) at your option.
