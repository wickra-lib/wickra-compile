# Wickra Compile examples

Runnable, self-contained examples — one per supported language. Every example
compiles the **same** strategy spec in dry-run mode and prints the resulting
`project_hash`. Because the manifest is deterministic across languages, that hash
is byte-identical in all of them: the whole point of wickra-compile.

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q --manifest-path examples/rust/Cargo.toml
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: compile a strategy spec (dry run) and print its deterministic manifest. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-compile-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `compile.c` | A runnable C example: compile a strategy spec (dry run) through the |
| `compile.cpp` | A runnable C++ example: compile a strategy spec (dry run) and print the raw JSON manifest -- through the C++ hull. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Compile
```

| Example | What it does |
| --- | --- |
| `Compile/Program.cs` | A runnable C# example: compile a strategy spec (dry run) and print its deterministic manifest. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `compile.go` | A runnable Go example: compile a strategy spec (dry run) and print its deterministic manifest. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/compile.R
```

| Example | What it does |
| --- | --- |
| `compile.R` | A runnable R example: compile a strategy spec (dry run) through the binding. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q package -DskipTests
javac -cp bindings/java/target/classes examples/java/Compile.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir="$PWD/target/release"  -cp "bindings/java/target/classes:examples/java/out" Compile
```

| Example | What it does |
| --- | --- |
| `Compile.java` | A runnable Java example: compile a strategy spec (dry run) through the binding. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-compile
python examples/python/compile.py
```

| Example | What it does |
| --- | --- |
| `compile.py` | A runnable Python example: compile a strategy spec (dry run) and print its |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/compile.js
```

| Example | What it does |
| --- | --- |
| `compile.js` | A runnable Node.js example: compile a strategy spec (dry run) and print its deterministic manifest. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `compile.html` | A runnable example against this binding. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## Run them

| Language | Path | Command |
|----------|------|---------|
| Rust | [`rust/`](rust/src/main.rs) | `cargo run --manifest-path examples/rust/Cargo.toml` |
| Python | [`python/compile.py`](python/compile.py) | `pip install wickra-compile && python examples/python/compile.py` |
| Node.js | [`node/`](node/compile.js) | `cd examples/node && npm install && node compile.js` |
| Go | [`go/`](go/compile.go) | `cd examples/go && go run compile.go` |
| C# | [`csharp/`](csharp/Compile/Program.cs) | `dotnet run --project examples/csharp/Compile` |
| Java | [`java/Compile.java`](java/Compile.java) | see header comment in the file |
| R | [`r/compile.R`](r/compile.R) | `R CMD INSTALL bindings/r && Rscript examples/r/compile.R` |
| C / C++ | [`c/`](c/compile.c) | `cmake -S examples/c -B examples/c/build && cmake --build examples/c/build && ctest --test-dir examples/c/build` |
| WASM | [`wasm/`](wasm/) | `wasm-pack build bindings/wasm --target web`, serve the repository root, open `examples/wasm/compile.html` |

The Go, C#, Java, R, C and C++ examples call through the C ABI or the native
binding, so they need the library built first. The C example calls the five
ABI functions directly; the C++ example goes through
`bindings/c/include/wickra_compile.hpp`, the header-only hull that owns the
handle and runs the length-out protocol. `golden_test.c` beside them asserts
golden parity and the dry-run-versus-build equivalence, and needs the
`thumbv7em-none-eabihf` target for its real build.

```bash
cargo build --release -p wickra-compile-c   # C / C++ / Go / C# / Java / R
```

## Data

[`data/`](data/) holds a few sample inputs you can point the CLI at:

- [`data/specs/`](data/specs/) — ready-to-compile strategy specs
  (`sma_cross`, `ema_trend`, `no_std_blink`).
- [`data/candles/btcusdt.csv`](data/candles/btcusdt.csv) — a small OHLCV series
  for specs that embed a dataset.

```bash
wickra-compile --spec examples/data/specs/sma_cross.json --manifest
```
