"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// generates the same deterministic manifest as every other binding. Skips
// cleanly when `pkg/` has not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
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

  // The cross-language golden: the wasm build reproduces the exact project_hash
  // pinned in golden/expected. binary_daemon embeds a CSV resolved relative to
  // the working directory, so it is covered by the Rust golden, not here.
  const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");
  const SPECS = ["sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"];

  test("wasm reproduces every golden spec's expected project hash", () => {
    for (const name of SPECS) {
      // Splice the raw spec JSON verbatim: parsing it through JavaScript would
      // collapse a float like `1.0` to `1`, changing the canonical form.
      const specRaw = fs.readFileSync(path.join(GOLDEN, "specs", `${name}.json`), "utf8");
      const expected = JSON.parse(
        fs.readFileSync(path.join(GOLDEN, "expected", `${name}.json`), "utf8"),
      );
      const out = JSON.parse(
        new wasm.Compiler().command(`{"cmd":"compile","dry_run":true,"spec":${specRaw}}`),
      );
      assert.strictEqual(
        out.manifest.project_hash,
        expected.project_hash,
        `project_hash mismatch for ${name}`,
      );
    }
  });
}
