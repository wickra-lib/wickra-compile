/* A runnable C example: compile a strategy spec (dry run) through the
 * wickra-compile C ABI and print the raw JSON manifest. Every language example
 * uses the same spec and prints the same project_hash. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_compile.h"

static const char *CMD =
    "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":"
    "{\"strategy\":{\"symbol\":\"btcusdt\",\"timeframe\":\"1h\","
    "\"indicators\":{\"fast\":{\"type\":\"Sma\",\"params\":[10]},"
    "\"slow\":{\"type\":\"Sma\",\"params\":[30]}},"
    "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
    "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
    "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}},"
    "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}}";

int main(void) {
    WickraCompiler *compiler = wickra_compile_new();
    if (!compiler) {
        fprintf(stderr, "failed to build compiler\n");
        return 1;
    }

    /* Length-out protocol: learn the length, then read into a caller buffer. */
    int len = wickra_compile_command(compiler, CMD, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_compile_free(compiler);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_compile_free(compiler);
        return 1;
    }
    wickra_compile_command(compiler, CMD, buf, (size_t)len + 1);

    printf("wickra-compile %s\n", wickra_compile_version());
    printf("output: %s\n", buf);

    free(buf);
    wickra_compile_free(compiler);
    return 0;
}
