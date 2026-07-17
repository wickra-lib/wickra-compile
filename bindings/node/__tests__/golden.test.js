"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
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
