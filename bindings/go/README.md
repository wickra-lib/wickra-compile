# Wickra Compile — Go

Go bindings for the Wickra strategy compiler over its C ABI hub via cgo. A
`Compiler` is driven over a JSON boundary, so the manifest it produces is
byte-identical to every other Wickra Compile binding.

## Install

```bash
go get github.com/wickra-lib/wickra-compile/bindings/go
```

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p compile-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

## Usage

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-compile/bindings/go"
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

## Surface

- **`New() *Compiler`** — construct a compiler handle; `Close()` frees it.
- **`(*Compiler).Command(cmdJSON string) (string, error)`** — apply a command
  envelope (`{"cmd":"...", ...}`) and return the response JSON. Commands:
  `compile`, `targets`, `version`, `artifact_bytes`, `reset`.
- **`(*Compiler).ArtifactBytes(path string) ([]byte, error)`** — read the raw
  bytes of a file through the C ABI byte reader.
- **`Version() string`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as a Go error.

## Determinism

The whole compiler lives once in the Rust core; this binding forwards its JSON
verbatim, so a given spec produces the byte-identical manifest here and in every
other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-compile>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.
