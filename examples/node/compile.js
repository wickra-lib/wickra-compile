// A runnable Node.js example: compile a strategy spec (dry run) and print its
// deterministic manifest.
//
//   npm install
//   node examples/node/compile.js
//
// Every language example uses the same spec and prints the same project_hash.
"use strict";

const { Compiler } = require("wickra-compile");

const SPEC = {
  strategy: {
    symbol: "btcusdt",
    timeframe: "1h",
    indicators: {
      fast: { type: "Sma", params: [10] },
      slow: { type: "Sma", params: [30] },
    },
    entry: { cross_above: ["fast", "slow"] },
    exit: { cross_below: ["fast", "slow"] },
    sizing: { type: "fixed_qty", qty: 1 },
  },
  target: { kind: "wasm" },
  crate_name: "demo",
};

const compiler = new Compiler();
const out = JSON.parse(
  compiler.command(JSON.stringify({ cmd: "compile", dry_run: true, spec: SPEC })),
);

console.log(`wickra-compile ${compiler.version()}`);
console.log(`crate: ${out.manifest.crate_name}`);
console.log(`files: ${out.manifest.files.length}`);
console.log(`project_hash: ${out.manifest.project_hash}`);
