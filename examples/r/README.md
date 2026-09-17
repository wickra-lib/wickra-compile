# Wickra Compile examples — R

Runnable R examples for the [Wickra Compile R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-compile-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/compile.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `compile.R` | A runnable R example: compile a strategy spec (dry run) through the binding. |
