# Wickra Compile — Java

JVM bindings for the Wickra strategy compiler over its C ABI hub via the Foreign
Function & Memory API (FFM / Panama). A `Compiler` is driven over a JSON
boundary, so the manifest it produces is byte-identical to every other Wickra
Compile binding.

## Requirements

- JDK 22+ (the FFM API is stable as of JDK 22).
- The native C ABI library, built by `cargo build -p compile-c` into the
  workspace `target/debug/` directory (the Maven build reads `native.lib.dir`).

Run with native access enabled:

```bash
java --enable-native-access=ALL-UNNAMED ...
```

## Usage

```java
import org.wickra.compile.Compiler;

try (Compiler compiler = new Compiler()) {
    String response = compiler.command("""
        {"cmd":"compile","dry_run":true,"spec":{
          "strategy":{"symbol":"x","timeframe":"1h",
            "indicators":{"f":{"type":"Ema","params":[3]}},
            "entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},
            "sizing":{"type":"fixed_qty","qty":1}},
          "target":{"kind":"wasm"},"crate_name":"demo"}}
        """);
    System.out.println(response); // response JSON, including manifest.project_hash
}
```

## Surface

- **`new Compiler()`** — construct a compiler handle (`AutoCloseable`).
- **`command(String cmdJson) -> String`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`artifactBytes(String path) -> byte[]`** — read the raw bytes of a file
  through the C ABI byte reader.
- **`Compiler.version() -> String`** — the crate version.

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
