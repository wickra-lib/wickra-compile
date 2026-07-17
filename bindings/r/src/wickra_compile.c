/* R .Call glue for the wickra-compile C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_compile.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkcompile_finalize(SEXP ext) {
    WickraCompiler *h = (WickraCompiler *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_compile_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraCompiler *handle_of(SEXP ext) {
    WickraCompiler *h = (WickraCompiler *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-compile: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkcompile_version(void) {
    return Rf_mkString(wickra_compile_version());
}

SEXP wkcompile_new(void) {
    WickraCompiler *h = wickra_compile_new();
    if (!h) {
        Rf_error("wickra-compile: allocation failed");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkcompile_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkcompile_command(SEXP ext, SEXP cmd_json) {
    WickraCompiler *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_compile_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-compile: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_compile_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkcompile_version", (DL_FUNC)&wkcompile_version, 0},
    {"wkcompile_new", (DL_FUNC)&wkcompile_new, 0},
    {"wkcompile_command", (DL_FUNC)&wkcompile_command, 2},
    {NULL, NULL, 0}};

void R_init_wickracompile(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
