# wickra-compile WASM examples

Browser demos for the `wickra-compile-wasm` binding.

The WASM build carries the whole codegen core: the same templates, the same
canonical hashing and the same manifest bytes the CLI and the other nine
bindings produce. A spec is data, not code, so the `project_hash` on this page
is the same one `examples/node/compile.js` prints. A real build needs `cargo`
and stays with the native bindings; the page stops at the dry-run manifest.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/compile.html`.

## Pages

| Page | What it does |
|------|--------------|
| `compile.html` | Compiles the SMA-crossover spec in dry-run mode, lists the generated files with their hashes, and shows that the manifest does not depend on the key order the spec was built in. The page counterpart of `examples/node/compile.js`. |

## See also

- [examples/README.md](../README.md) — the same compile in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.
