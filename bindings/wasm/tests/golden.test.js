"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// generates the same deterministic manifest as every other binding. Skips
// cleanly when `pkg/` has not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "compile_wasm.js"));
} catch {
  wasm = null;
}

// A dry-run compile: pure codegen + manifest, no toolchain. The strategy is a
// valid `wickra_backtest::StrategySpec` so the compiler accepts it.
const COMPILE_CMD = JSON.stringify({
  cmd: "compile",
  dry_run: true,
  spec: {
    strategy: {
      symbol: "x",
      timeframe: "1h",
      indicators: { f: { type: "Ema", params: [3] } },
      entry: { cross_above: ["f", "f"] },
      exit: { cross_below: ["f", "f"] },
      sizing: { type: "fixed_qty", qty: 1 },
    },
    target: { kind: "wasm" },
    crate_name: "demo",
  },
});

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm dry-run compile returns a manifest", () => {
    const out = JSON.parse(new wasm.Compiler().command(COMPILE_CMD));
    assert.strictEqual(typeof out.manifest.project_hash, "string");
    assert.strictEqual(out.built, false);
  });

  test("wasm compile is byte-identical across calls", () => {
    const a = new wasm.Compiler().command(COMPILE_CMD);
    const b = new wasm.Compiler().command(COMPILE_CMD);
    assert.strictEqual(a, b);
  });

  test("wasm reports an in-band error on a real build (no toolchain)", () => {
    const cmd = COMPILE_CMD.replace('"dry_run":true', '"dry_run":false');
    const out = JSON.parse(new wasm.Compiler().command(cmd));
    assert.strictEqual(out.ok, false);
  });

  test("wasm returns an in-band error on an unknown command", () => {
    const out = JSON.parse(new wasm.Compiler().command('{"cmd":"nope"}'));
    assert.strictEqual(out.ok, false);
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Compiler().version(), wasm.version());
  });
}
