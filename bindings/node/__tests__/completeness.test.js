"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Compiler } = require("../index.js");

test("the Compiler surface exposes command and version", () => {
  const c = new Compiler();
  assert.strictEqual(typeof c.command, "function");
  assert.strictEqual(typeof c.version, "function");
});
