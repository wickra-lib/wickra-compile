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

cat("wickra-compile R tests passed\n")
