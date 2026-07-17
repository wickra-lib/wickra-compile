package org.wickra.compile;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: the same spec yields a
// byte-identical manifest across calls, and those bytes are what every other
// binding produces too, because the whole compiler lives once in the Rust core
// and this binding forwards its JSON verbatim.
class GoldenTest {
    @Test
    void compileIsByteIdenticalAcrossCalls() {
        try (Compiler a = new Compiler(); Compiler b = new Compiler()) {
            assertEquals(a.command(CompilerTest.COMPILE_CMD), b.command(CompilerTest.COMPILE_CMD));
        }
    }
}
