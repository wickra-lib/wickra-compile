"use strict";

// Operating-mode equivalence the WebAssembly core can make without a
// toolchain: a spec arrives either as a file's bytes (the CLI, the golden
// corpus) or as an object a page builds in JavaScript, whose key order is
// whatever the code happened to write. The manifest must not depend on which
// way it came: the canonical form the core hashes is the logical spec, not the
// key order of its bytes. The core pins this in Rust (canonical_determinism.rs); this
// checks the boundary the WASM binding crosses. A missing corpus is a failure,
// not a skip.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const wasm = require(path.resolve(__dirname, "..", "pkg-node", "wickra_compile_wasm.js"));

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");
const SPECS = ["sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"];

// The same object with its keys in reverse insertion order, recursively.
function reversed(value) {
  if (Array.isArray(value)) return value.map(reversed);
  if (value && typeof value === "object") {
    const out = {};
    for (const key of Object.keys(value).reverse()) out[key] = reversed(value[key]);
    return out;
  }
  return value;
}

test("a spec in any key order compiles to the same manifest", () => {
  assert.ok(fs.existsSync(path.join(GOLDEN, "specs")), "golden corpus not found");
  for (const name of SPECS) {
    const specRaw = fs.readFileSync(path.join(GOLDEN, "specs", `${name}.json`), "utf8");
    const expected = JSON.parse(fs.readFileSync(path.join(GOLDEN, "expected", `${name}.json`), "utf8"));
    // The file's own bytes, spliced verbatim.
    const fromFile = JSON.parse(new wasm.Compiler().command(`{"cmd":"compile","dry_run":true,"spec":${specRaw}}`));
    assert.deepStrictEqual(fromFile.manifest, expected, name);
    // The same spec as an object a page would build, in two key orders. Both
    // go through JavaScript's number formatting (a `1.0` in the file becomes
    // `1`, which is a different canonical number and hashes differently -- the
    // reason the golden check above splices the file's own bytes), so the two
    // objects are compared with each other: what must not matter is the order.
    const spec = JSON.parse(specRaw);
    const inOrder = JSON.parse(
      new wasm.Compiler().command(JSON.stringify({ cmd: "compile", dry_run: true, spec })),
    );
    const inReverse = JSON.parse(
      new wasm.Compiler().command(JSON.stringify({ cmd: "compile", dry_run: true, spec: reversed(spec) })),
    );
    assert.deepStrictEqual(inReverse.manifest, inOrder.manifest, name);
  }
});
