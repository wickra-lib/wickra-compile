# wickra-compile (Node.js)

Node.js bindings for [`wickra-compile`](https://github.com/wickra-lib/wickra-compile),
powered by Rust via [napi-rs](https://napi.rs/): compile a Wickra strategy spec
into a standalone deployable and get a **deterministic manifest** — byte-identical
to every other language binding.

```js
const { Compiler } = require("wickra-compile");

const spec = {
  strategy: { symbol: "BTCUSDT", timeframe: "1h",
    indicators: { fast: { type: "Sma", params: [10] }, slow: { type: "Sma", params: [30] } },
    entry: { cross_above: ["fast", "slow"] },
    exit:  { cross_below: ["fast", "slow"] },
    sizing: { type: "fixed_qty", qty: 1 } },
  target: { kind: "wasm" }, opt_level: "size",
};

const c = new Compiler();
const out = JSON.parse(c.command(JSON.stringify({ cmd: "compile", dry_run: true, spec })));
console.log(out.manifest.project_hash);  // deterministic across runs and languages
```

`command` mirrors `Compiler::command_json`: the commands are `compile`,
`targets`, `version`, `artifact_bytes` and `reset`. Domain errors come back
in-band as `{ ok: false, error: ... }`. A `compile` with `dry_run: true` needs no
toolchain; `dry_run: false` invokes `cargo`.

## Install

```bash
npm install wickra-compile
```

Requires Node.js >= 22. The correct native binary is installed automatically as
an optional dependency for your platform.
