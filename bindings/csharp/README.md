# Wickra Compile — C#

C# / .NET bindings for the Wickra strategy compiler over its C ABI hub. A
`Compiler` is driven over a JSON boundary, so the manifest it produces is
byte-identical to every other Wickra Compile binding.

## Install

```bash
dotnet add package Wickra.Compile
```

The native C ABI library is loaded through a `DllImportResolver`. For a local
build, run `cargo build -p compile-c --release` and the test project copies the
library from `target/release` next to the assembly (see the `<Content>` items in
`WickraCompile.Tests`).

## Usage

```csharp
using Wickra.Compile;

using var compiler = new Compiler();

var response = compiler.Command("""
    {"cmd":"compile","dry_run":true,"spec":{
      "strategy":{"symbol":"x","timeframe":"1h",
        "indicators":{"f":{"type":"Ema","params":[3]}},
        "entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},
        "sizing":{"type":"fixed_qty","qty":1}},
      "target":{"kind":"wasm"},"crate_name":"demo"}}
    """);

Console.WriteLine(response); // response JSON, including manifest.project_hash
```

## Surface

- **`new Compiler()`** — construct a compiler handle; it is `IDisposable`, so
  wrap it in `using`.
- **`string Command(string cmdJson)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`static string Version()`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as an exception.

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
