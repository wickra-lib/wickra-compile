# A runnable R example: compile a strategy spec (dry run) through the binding.
#
#   R CMD INSTALL bindings/r
#   Rscript examples/r/compile.R
#
# Every language example uses the same spec and prints the same project_hash.
library(wickracompile)

spec <- paste0(
  '{"strategy":{"symbol":"btcusdt","timeframe":"1h",',
  '"indicators":{"fast":{"type":"Sma","params":[10]},',
  '"slow":{"type":"Sma","params":[30]}},',
  '"entry":{"cross_above":["fast","slow"]},',
  '"exit":{"cross_below":["fast","slow"]},',
  '"sizing":{"type":"fixed_qty","qty":1}},',
  '"target":{"kind":"wasm"},"crate_name":"demo"}'
)

compiler <- wkcompile_new()
response <- wkcompile_command(
  compiler,
  paste0('{"cmd":"compile","dry_run":true,"spec":', spec, "}")
)

cat(sprintf("wickra-compile %s\n", wkcompile_version()))
cat(response, "\n")
