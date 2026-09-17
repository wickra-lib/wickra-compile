# Wickra Compile examples — Go

Runnable Go examples for the [Wickra Compile Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-compile-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_compile.so bindings/go/lib/linux_amd64/
```

## Run

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `compile.go` | A runnable Go example: compile a strategy spec (dry run) and print its deterministic manifest. |
