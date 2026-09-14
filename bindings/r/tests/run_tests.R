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
## corpus lives at the repository root; a missing corpus is a failure, not a
## skip (the shipped tests/smoke.R is what runs inside the tarball).
project_hash <- function(json) {
  m <- regmatches(json, regexpr('"project_hash":"[0-9a-f]+"', json))
  sub('"project_hash":"([0-9a-f]+)"', "\\1", m)
}

slurp <- function(path) readChar(path, file.info(path)$size)

## CI runs this from the repository root and a developer from bindings/r, so
## the corpus is looked for upward from the working directory.
golden <- NULL
d <- normalizePath(getwd(), mustWork = FALSE)
for (i in seq_len(8)) {
  candidate <- file.path(d, "golden")
  if (dir.exists(file.path(candidate, "specs"))) {
    golden <- candidate
    break
  }
  d <- dirname(d)
}
stopifnot(!is.null(golden))

for (name in c("sma_cross", "ema_trend", "rsi_reversion", "no_std_blink")) {
  spec <- slurp(file.path(golden, "specs", paste0(name, ".json")))
  expected <- project_hash(slurp(file.path(golden, "expected", paste0(name, ".json"))))
  resp <- wkcompile_command(compiler,
    paste0('{"cmd":"compile","dry_run":true,"spec":', spec, "}"))
  stopifnot(project_hash(resp) == expected)
}
cat("wickra-compile R golden checks passed\n")

## Operating-mode equivalence: the manifest a dry run describes is the manifest
## a real build builds. `compile` runs two ways -- dry_run true stops after
## codegen, dry_run false writes the project and invokes cargo -- and both must
## carry the same manifest, with built and path the only difference. The no_std
## spec is the one built for real: no dependencies, seconds to compile, only the
## thumbv7em-none-eabihf target needed. The core pins this in Rust; this checks
## the boundary the R binding crosses. The "manifest" object is cut out of the
## response text by brace depth and compared verbatim.
manifest_of <- function(artifact) {
  at <- regexpr('"manifest":\\{', artifact)
  stopifnot(at > 0)
  start <- at + nchar('"manifest":')
  chars <- strsplit(substr(artifact, start, nchar(artifact)), "")[[1]]
  depth <- 0L
  for (i in seq_along(chars)) {
    if (chars[i] == "{") depth <- depth + 1L
    if (chars[i] == "}") {
      depth <- depth - 1L
      if (depth == 0L) return(paste(chars[seq_len(i)], collapse = ""))
    }
  }
  stop("unterminated manifest")
}

spec <- slurp(file.path(golden, "specs", "no_std_blink.json"))
expected <- trimws(slurp(file.path(golden, "expected", "no_std_blink.json")))
dry <- wkcompile_command(compiler, paste0('{"cmd":"compile","dry_run":true,"spec":', spec, "}"))
stopifnot(grepl('"built":false', dry, fixed = TRUE))
## the artifact's own path precedes the manifest; the manifest's file entries
## carry paths of their own, so only the head of the response is searched.
head_of <- function(artifact) sub('"manifest":.*$', "", artifact)
stopifnot(!grepl('"path":', head_of(dry), fixed = TRUE))
stopifnot(identical(manifest_of(dry), expected))

out_dir <- file.path(tempdir(), paste0("wickra-compile-modes-", Sys.getpid()))
dir.create(out_dir)
built <- wkcompile_command(compiler, paste0(
  '{"cmd":"compile","dry_run":false,"out_dir":"', gsub("\\\\", "/", out_dir),
  '","spec":', spec, "}"))
stopifnot(grepl('"built":true', built, fixed = TRUE))
artifact <- sub('.*"path":"([^"]+)".*', "\\1", head_of(built))
stopifnot(file.exists(artifact))
stopifnot(identical(manifest_of(built), manifest_of(dry)))
unlink(out_dir, recursive = TRUE)
cat("wickra-compile R operating modes: a real build carries the dry-run manifest\n")

cat("wickra-compile R tests passed\n")
