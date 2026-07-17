#' The wickra-compile library version.
#' @return A version string.
#' @export
wkcompile_version <- function() {
  .Call(C_wkcompile_version)
}

#' Construct a compiler handle.
#' @return A `wickra_compile` handle (an external pointer).
#' @export
wkcompile_new <- function() {
  .Call(C_wkcompile_new)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param compiler A compiler handle from [wkcompile_new()].
#' @param cmd_json A command JSON string.
#' @return The response as a JSON string.
#' @export
wkcompile_command <- function(compiler, cmd_json) {
  .Call(C_wkcompile_command, compiler, cmd_json)
}
