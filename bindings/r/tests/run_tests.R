## Plain-R tests for the wickra-compile R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickracompile)

## A dry-run compile: pure codegen + manifest, no toolchain. The strategy is a
## valid wickra_backtest::StrategySpec so the compiler accepts it.
compile_cmd <- paste0(
  '{"cmd":"compile","dry_run":true,"spec":{',
  '"strategy":{"symbol":"x","timeframe":"1h",',
  '"indicators":{"f":{"type":"Ema","params":[3]}},',
  '"entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},',
  '"sizing":{"type":"fixed_qty","qty":1}},',
  '"target":{"kind":"wasm"},"crate_name":"demo"}}'
)

## version
stopifnot(nzchar(wkcompile_version()))

## a dry-run compile returns a manifest, not a built artifact
compiler <- wkcompile_new()
out <- wkcompile_command(compiler, compile_cmd)
stopifnot(grepl('"project_hash"', out, fixed = TRUE))
stopifnot(grepl('"built":false', out, fixed = TRUE))

## the manifest is byte-identical across compilers (the cross-language golden core)
compiler2 <- wkcompile_new()
out2 <- wkcompile_command(compiler2, compile_cmd)
stopifnot(identical(out, out2))

## an unknown command is an in-band error, not a hard error
inband <- wkcompile_command(compiler, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## Cross-language golden: every golden spec reproduces the exact project_hash
## pinned in golden/expected. binary_daemon embeds a CSV resolved relative to the
## working directory, so it is covered by the Rust golden, not here. The golden
## corpus lives at the repository root; locate it relative to this test, skipping
## cleanly if it is not reachable (e.g. an isolated R CMD check sandbox).
project_hash <- function(json) {
  m <- regmatches(json, regexpr('"project_hash":"[0-9a-f]+"', json))
  sub('"project_hash":"([0-9a-f]+)"', "\\1", m)
}

golden <- NULL
for (candidate in c("../../golden", "../../../golden")) {
  if (dir.exists(candidate)) {
    golden <- candidate
    break
  }
}

if (!is.null(golden)) {
  for (name in c("sma_cross", "ema_trend", "rsi_reversion", "no_std_blink")) {
    spec <- readChar(file.path(golden, "specs", paste0(name, ".json")),
      file.info(file.path(golden, "specs", paste0(name, ".json")))$size)
    expected <- project_hash(readChar(
      file.path(golden, "expected", paste0(name, ".json")),
      file.info(file.path(golden, "expected", paste0(name, ".json")))$size))
    resp <- wkcompile_command(compiler,
      paste0('{"cmd":"compile","dry_run":true,"spec":', spec, "}"))
    stopifnot(project_hash(resp) == expected)
  }
  cat("wickra-compile R golden checks passed\n")
} else {
  cat("golden corpus not reachable; skipping cross-language golden\n")
}

cat("wickra-compile R tests passed\n")
