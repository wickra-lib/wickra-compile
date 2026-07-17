package org.wickra.compile;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: the same spec yields a
// byte-identical manifest across calls, and every golden spec reproduces the
// exact project hash pinned in golden/expected — those bytes are what every
// binding produces, because the whole compiler lives once in the Rust core and
// this binding forwards its JSON verbatim.
class GoldenTest {
    private static final Pattern PROJECT_HASH =
            Pattern.compile("\"project_hash\"\\s*:\\s*\"([0-9a-f]+)\"");

    // binary_daemon embeds a CSV resolved relative to the working directory, so
    // it is covered by the Rust golden, not here.
    private static final String[] SPECS = {"sma_cross", "ema_trend", "rsi_reversion", "no_std_blink"};

    @Test
    void compileIsByteIdenticalAcrossCalls() {
        try (Compiler a = new Compiler(); Compiler b = new Compiler()) {
            assertEquals(a.command(CompilerTest.COMPILE_CMD), b.command(CompilerTest.COMPILE_CMD));
        }
    }

    @Test
    void everyGoldenSpecReproducesItsExpectedProjectHash() throws Exception {
        // `mvn` runs from bindings/java; the golden corpus is two levels up.
        Path golden = Path.of("..", "..", "golden");
        try (Compiler compiler = new Compiler()) {
            for (String name : SPECS) {
                String spec = Files.readString(golden.resolve("specs").resolve(name + ".json"));
                String expectedJson =
                        Files.readString(golden.resolve("expected").resolve(name + ".json"));
                String expected = projectHash(expectedJson);

                String cmd = "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":" + spec + "}";
                String got = projectHash(compiler.command(cmd));

                assertEquals(expected, got, name);
            }
        }
    }

    private static String projectHash(String json) {
        Matcher matcher = PROJECT_HASH.matcher(json);
        if (!matcher.find()) {
            throw new IllegalStateException("no project_hash in: " + json);
        }
        return matcher.group(1);
    }
}
