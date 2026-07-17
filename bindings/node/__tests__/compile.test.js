"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Compiler } = require("../index.js");

const SPEC = {
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
};

function dryRun(compiler) {
  return JSON.parse(
    compiler.command(JSON.stringify({ cmd: "compile", dry_run: true, spec: SPEC })),
  );
}

test("dry-run compile returns a manifest, not built", () => {
  const out = dryRun(new Compiler());
  assert.strictEqual(out.built, false);
  assert.strictEqual(out.manifest.crate_name, "demo");
  assert.ok(out.manifest.project_hash);
});

test("the same spec is deterministic across instances", () => {
  const a = dryRun(new Compiler()).manifest;
  const b = dryRun(new Compiler()).manifest;
  assert.deepStrictEqual(a, b);
});

test("targets lists wasm and an mcu allowlist", () => {
  const out = JSON.parse(new Compiler().command(JSON.stringify({ cmd: "targets" })));
  assert.ok(out.targets.includes("wasm"));
  assert.ok(out.mcus.includes("thumbv6m-none-eabi"));
});

test("an unknown command returns an in-band error", () => {
  const out = JSON.parse(new Compiler().command(JSON.stringify({ cmd: "nope" })));
  assert.strictEqual(out.ok, false);
});

test("version is a string", () => {
  assert.strictEqual(typeof new Compiler().version(), "string");
});

module.exports = { SPEC, dryRun };
