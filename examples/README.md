# Examples

Runnable, self-contained examples — one per supported language. Every example
compiles the **same** strategy spec in dry-run mode and prints the resulting
`project_hash`. Because the manifest is deterministic across languages, that hash
is byte-identical in all of them: the whole point of wickra-compile.

The spec is a simple SMA crossover targeting WebAssembly:

```json
{
  "strategy": {
    "symbol": "btcusdt",
    "timeframe": "1h",
    "indicators": {
      "fast": { "type": "Sma", "params": [10] },
      "slow": { "type": "Sma", "params": [30] }
    },
    "entry": { "cross_above": ["fast", "slow"] },
    "exit": { "cross_below": ["fast", "slow"] },
    "sizing": { "type": "fixed_qty", "qty": 1 }
  },
  "target": { "kind": "wasm" },
  "crate_name": "demo"
}
```

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

The Go, C#, Java, R, C and C++ examples call through the C ABI or the native
binding, so they need the library built first:

```bash
cargo build --release -p compile-c   # C / C++ / Go / C# / Java / R
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
