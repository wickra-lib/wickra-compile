// A runnable C++ example: compile a strategy spec (dry run) through the
// wickra-compile C ABI and print the raw JSON manifest. Every language example
// uses the same spec and prints the same project_hash.
#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>

extern "C" {
#include "wickra_compile.h"
}

namespace {
const char *kCmd =
    "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":"
    "{\"strategy\":{\"symbol\":\"btcusdt\",\"timeframe\":\"1h\","
    "\"indicators\":{\"fast\":{\"type\":\"Sma\",\"params\":[10]},"
    "\"slow\":{\"type\":\"Sma\",\"params\":[30]}},"
    "\"entry\":{\"cross_above\":[\"fast\",\"slow\"]},"
    "\"exit\":{\"cross_below\":[\"fast\",\"slow\"]},"
    "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}},"
    "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}}";
}  // namespace

int main() {
    WickraCompiler *compiler = wickra_compile_new();
    if (!compiler) {
        std::cerr << "failed to build compiler\n";
        return 1;
    }

    // Length-out protocol: learn the length, then read into a caller buffer.
    int len = wickra_compile_command(compiler, kCmd, nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed: code " << len << "\n";
        wickra_compile_free(compiler);
        return 1;
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_compile_command(compiler, kCmd, buf.data(), buf.size());

    std::cout << "wickra-compile " << wickra_compile_version() << "\n";
    std::cout << "output: " << std::string(buf.data()) << "\n";

    wickra_compile_free(compiler);
    return 0;
}
