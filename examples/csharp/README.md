# Wickra Compile examples — C#

Runnable C# examples for the [Wickra Compile C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-compile-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Compile
```

## The examples

| Example | What it does |
|---------|--------------|
| `Compile/Program.cs` | A runnable C# example: compile a strategy spec (dry run) and print its deterministic manifest. |
