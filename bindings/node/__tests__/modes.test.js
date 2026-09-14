"use strict";

// Operating-mode equivalence through the binding: the manifest a dry run
// describes is the manifest a real build builds. `compile` runs two ways --
// `dry_run: true` stops after codegen and returns the manifest, `dry_run:
// false` writes the project and invokes `cargo` on it -- and both must carry
// the same manifest, with `built` and `path` the only difference. The no_std
// spec is the one built for real: its generated project has no dependencies,
// so it compiles in seconds, and it needs only the `thumbv7em-none-eabihf`
// target the CI jobs install. The core pins this in Rust (operating_modes.rs);
// this checks the boundary the Node binding crosses. A missing corpus is a
// failure, not a skip.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { Compiler } = require("../index.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");

test("a real build carries the dry-run manifest", () => {
  // Splice the raw spec JSON verbatim, so a float like `1.0` keeps its form.
  const specRaw = fs.readFileSync(path.join(GOLDEN, "specs", "no_std_blink.json"), "utf8");
  const expected = JSON.parse(fs.readFileSync(path.join(GOLDEN, "expected", "no_std_blink.json"), "utf8"));

  const compiler = new Compiler();
  const dry = JSON.parse(compiler.command(`{"cmd":"compile","dry_run":true,"spec":${specRaw}}`));
  assert.strictEqual(dry.built, false);
  assert.strictEqual(dry.path, undefined);
  assert.deepStrictEqual(dry.manifest, expected);

  const outDir = fs.mkdtempSync(path.join(os.tmpdir(), "wickra-compile-modes-"));
  try {
    const built = JSON.parse(
      compiler.command(
        `{"cmd":"compile","dry_run":false,"out_dir":${JSON.stringify(outDir.replace(/\\/g, "/"))},"spec":${specRaw}}`,
      ),
    );
    assert.strictEqual(built.built, true, JSON.stringify(built));
    assert.ok(fs.existsSync(built.path), `artifact missing at ${built.path}`);
    assert.deepStrictEqual(built.manifest, dry.manifest);
  } finally {
    fs.rmSync(outDir, { recursive: true, force: true });
  }
});
