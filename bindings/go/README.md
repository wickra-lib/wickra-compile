<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-compile-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — Go

---

**Part of the [Wickra ecosystem](#ecosystem): — for Go. `go get github.com/wickra-lib/wickra-compile-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

Go bindings for the Wickra strategy compiler over its C ABI hub via cgo. A
`Compiler` is driven over a JSON boundary, so the manifest it produces is
byte-identical to every other Wickra Compile binding.

## Install

Use the published **`wickra-compile-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-compile-go
```

`wickra-compile-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_compile.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-compile-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-compile-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_compile.so bindings/go/lib/linux_amd64/
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-compile-go"
)

func main() {
	c := wickra.New()
	defer c.Close()

	resp, _ := c.Command(`{"cmd":"compile","dry_run":true,"spec":{
		"strategy":{"symbol":"x","timeframe":"1h",
			"indicators":{"f":{"type":"Ema","params":[3]}},
			"entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},
			"sizing":{"type":"fixed_qty","qty":1}},
		"target":{"kind":"wasm"},"crate_name":"demo"}}`)
	fmt.Println(resp) // response JSON, including manifest.project_hash
}
```

### Surface

- **`New() *Compiler`** — construct a compiler handle; `Close()` frees it.
- **`(*Compiler).Command(cmdJSON string) (string, error)`** — apply a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `compile`, `targets`, `version`, `artifact_bytes`, `reset`.
- **`(*Compiler).ArtifactBytes(path string) ([]byte, error)`** — read the raw
  bytes of a file through the C ABI byte reader.
- **`Version() string`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as a Go error.

### Determinism

The whole compiler lives once in the Rust core; this binding forwards its JSON
verbatim, so a given spec produces the byte-identical manifest here and in every
other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/go)

- The main project: <https://github.com/wickra-lib/wickra-compile>
- Documentation: <https://wickra.org>

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
