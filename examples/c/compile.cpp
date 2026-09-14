// A runnable C++ example: compile a strategy spec (dry run) and print the raw
// JSON manifest -- through the C++ hull. Every language example uses the same
// spec and prints the same project_hash.
//
// This goes through `wickra_compile.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_compile_command` for you -- the core carries the produced-but-
// undelivered response between the two calls, so a real build runs once, not
// twice -- and turns a refusal into an exception rather than a negative
// integer that is easy to ignore. Calling the C functions directly from C++
// works too, but then the hull would be shipped without anything building it.
#include <cstdio>
#include <string>

#include "wickra_compile.hpp"

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
    try {
        wickra::Compiler compiler;
        const std::string artifact = compiler.command(kCmd);
        std::printf("wickra-compile %s\n", wickra::Compiler::version().c_str());
        std::printf("output: %s\n", artifact.c_str());
        // An in-band refusal: the ABI answered, the core declined. That is a
        // response rather than an error, so the hull does not throw on it.
        if (artifact.find("\"ok\":false") != std::string::npos) {
            std::fprintf(stderr, "the compiler refused the spec\n");
            return 1;
        }
    } catch (const wickra::CompileError &err) {
        // Every failure arrives here: a handle the library refused, a call that
        // returned a negative code.
        std::fprintf(stderr, "%s\n", err.what());
        return 1;
    }
    return 0;
}
