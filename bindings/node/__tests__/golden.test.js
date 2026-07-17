"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Compiler } = require("../index.js");

const BASE = {
  strategy: {
    symbol: "btc",
    timeframe: "1h",
    indicators: { f: { type: "Sma", params: [10] } },
    entry: { cross_above: ["f", "f"] },
    exit: { cross_below: ["f", "f"] },
    sizing: { type: "fixed_qty", qty: 1 },
  },
  target: { kind: "binary" },
};

function manifest(spec) {
  return JSON.parse(
    new Compiler().command(JSON.stringify({ cmd: "compile", dry_run: true, spec })),
  ).manifest;
}

test("the same spec yields the same project hash", () => {
  assert.strictEqual(manifest(BASE).project_hash, manifest(BASE).project_hash);
});

test("changing the target changes the project hash", () => {
  const binary = manifest(BASE);
  const wasm = manifest({ ...BASE, target: { kind: "wasm" } });
  assert.notStrictEqual(binary.project_hash, wasm.project_hash);
});

// The cross-language golden: every binding must reproduce the exact
// `project_hash` pinned in golden/expected. `binary_daemon` embeds a CSV whose
// path is resolved relative to the working directory, so it is covered by the
// Rust golden (which controls the cwd) rather than here.
const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");
const SPECS = ["sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"];

test("every golden spec reproduces its expected project hash", () => {
  for (const name of SPECS) {
    // Splice the raw spec JSON into the command verbatim: parsing it through
    // JavaScript would collapse a float like `1.0` to `1`, changing the
    // canonical form and thus the hash.
    const specRaw = fs.readFileSync(path.join(GOLDEN, "specs", `${name}.json`), "utf8");
    const expected = JSON.parse(fs.readFileSync(path.join(GOLDEN, "expected", `${name}.json`), "utf8"));
    const out = JSON.parse(
      new Compiler().command(`{"cmd":"compile","dry_run":true,"spec":${specRaw}}`),
    );
    assert.strictEqual(
      out.manifest.project_hash,
      expected.project_hash,
      `project_hash mismatch for ${name}`,
    );
  }
});
