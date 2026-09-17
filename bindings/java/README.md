<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-compile)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — Java

---

**Part of the [Wickra ecosystem](#ecosystem): — for Java. `org.wickra:wickra-compile` — prebuilt native library inside the jar, no JNI, no system dependencies.**

JVM bindings for the Wickra strategy compiler over its C ABI hub via the Foreign
Function & Memory API (FFM / Panama). A `Compiler` is driven over a JSON
boundary, so the manifest it produces is byte-identical to every other Wickra
Compile binding.

## Requirements

- JDK 22+ (the FFM API is stable as of JDK 22).
- The native C ABI library, built by `cargo build -p wickra-compile-c` into the
  workspace `target/debug/` directory (the Maven build reads `native.lib.dir`).

Run with native access enabled:

```bash
java --enable-native-access=ALL-UNNAMED ...
```

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-compile</artifactId>
  <version>0.1.2</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-compile:0.1.2")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

## Quick start

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

### Surface

- **`new Compiler()`** — construct a compiler handle (`AutoCloseable`).
- **`command(String cmdJson) -> String`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `compile`,
  `targets`, `version`, `artifact_bytes`, `reset`.
- **`artifactBytes(String path) -> byte[]`** — read the raw bytes of a file
  through the C ABI byte reader.
- **`Compiler.version() -> String`** — the crate version.

A malformed command, an unknown command name, or an invalid spec is reported
in-band as `{"ok":false,"error":...}` (the response JSON), not as an exception.

### Determinism

The whole compiler lives once in the Rust core; this binding forwards its JSON
verbatim, so a given spec produces the byte-identical manifest here and in every
other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/java)

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
