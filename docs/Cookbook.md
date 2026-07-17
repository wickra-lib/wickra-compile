# Cookbook

Short, practical recipes. Every one is deterministic: the same spec produces the
same manifest in every language.

## Print the manifest for a spec (CLI)

```bash
wickra-compile --spec strategy.json --manifest
```

`--manifest` prints the manifest JSON and nothing else — no files are written,
no `cargo` is invoked.

## Generate a project without building (dry run)

```bash
wickra-compile --spec strategy.json --dry-run --out ./out
```

Writes the generated crate under `./out/<crate_name>/` and prints the same
manifest as `--manifest`. Inspect the emitted `Cargo.toml`, `src/spec.json` and
`src/main.rs`.

## Compile for a different target than the spec

```bash
wickra-compile --spec strategy.json --target wasm --opt size
wickra-compile --spec strategy.json --target binary
wickra-compile --spec strategy.json --target no-std --mcu thumbv7em-none-eabihf
```

`--target` / `--opt` / `--mcu` override the spec's values without editing it.
`--mcu` is required with `--target no-std`.

## Verify two languages agree

The manifest is byte-identical across bindings. In Node.js:

```js
const { Compiler } = require("wickra-compile");
const spec = require("fs").readFileSync("golden/specs/sma_cross.json", "utf8");
const out = JSON.parse(new Compiler().command(`{"cmd":"compile","dry_run":true,"spec":${spec}}`));
console.log(out.manifest.project_hash);
```

and in Python:

```python
import json
from wickra_compile import Compiler
spec = open("golden/specs/sma_cross.json").read()
out = json.loads(Compiler().command(f'{{"cmd":"compile","dry_run":true,"spec":{spec}}}'))
print(out["manifest"]["project_hash"])
```

Both print the same `project_hash`. (Splice the spec file text in verbatim — do
not re-parse and re-serialise it, or a language's number formatting can change
the canonical form and therefore the hash.)

## Embed a dataset

Point the spec's `embed_data` at a CSV:

```json
{ "embed_data": { "kind": "csv", "path": "candles.csv" } }
```

Run the CLI from the directory that holds `candles.csv` — the path is resolved
relative to the working directory when the manifest is produced.

## List the supported targets

```bash
wickra-compile --spec any.json --manifest   # manifest carries the resolved target
```

or, through any binding, send `{"cmd":"targets"}` to get the MCU allowlist and
target kinds at runtime.

## See also

- [COMPILESPEC.md](COMPILESPEC.md) · [TARGETS.md](TARGETS.md) ·
  [DETERMINISM.md](DETERMINISM.md)
