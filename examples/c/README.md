# Wickra Compile — C / C++ examples

The Wickra Compile C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_compile.h`](../../bindings/c/include/wickra_compile.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_compile.hpp`](../../bindings/c/include/wickra_compile.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-compile-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_compile.so`     | `-lwickra_compile` |
| macOS    | `libwickra_compile.dylib`  | `-lwickra_compile` |
| Windows (MSVC) | `wickra_compile.dll` | `wickra_compile.dll.lib` (import lib) |

A static library (`libwickra_compile.a` / `wickra_compile.lib`) is emitted alongside.

## Build and run the examples

### With CMake (portable, used by CI)

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

### Directly with a compiler

```sh
# Linux / macOS
cc examples/c/compile.c -I bindings/c/include -L target/release -lwickra_compile -lm -o compile
LD_LIBRARY_PATH=target/release ./compile        # macOS: DYLD_LIBRARY_PATH

# Windows (MinGW gcc, linking the DLL directly)
gcc examples/c/compile.c -I bindings/c/include target/release/wickra_compile.dll -lm -o compile.exe
```

## The examples

| Example | What it does |
|---------|--------------|
| `compile.c` | A runnable C example: compile a strategy spec (dry run) through the |
| `compile.cpp` | A runnable C++ example: compile a strategy spec (dry run) and print the raw JSON manifest -- through the C++ hull. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_compile.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
