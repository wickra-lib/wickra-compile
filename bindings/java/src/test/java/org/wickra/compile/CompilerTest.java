package org.wickra.compile;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.file.Files;
import java.nio.file.Path;
import org.junit.jupiter.api.Test;

class CompilerTest {
    // A dry-run compile: pure codegen + manifest, no toolchain. The strategy is a
    // valid wickra_backtest::StrategySpec so the compiler accepts it.
    static final String COMPILE_CMD =
            "{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":{"
                    + "\"strategy\":{\"symbol\":\"x\",\"timeframe\":\"1h\","
                    + "\"indicators\":{\"f\":{\"type\":\"Ema\",\"params\":[3]}},"
                    + "\"entry\":{\"cross_above\":[\"f\",\"f\"]},\"exit\":{\"cross_below\":[\"f\",\"f\"]},"
                    + "\"sizing\":{\"type\":\"fixed_qty\",\"qty\":1}},"
                    + "\"target\":{\"kind\":\"wasm\"},\"crate_name\":\"demo\"}}";

    @Test
    void versionIsNonEmpty() {
        assertFalse(Compiler.version().isEmpty());
    }

    @Test
    void compileDryRunReturnsManifest() {
        try (Compiler compiler = new Compiler()) {
            String out = compiler.command(COMPILE_CMD);
            assertTrue(out.contains("\"project_hash\""), out);
            assertTrue(out.contains("\"built\":false"), out);
        }
    }

    @Test
    void unknownCommandIsInBandError() {
        try (Compiler compiler = new Compiler()) {
            // The C ABI hub folds a domain error into {"ok":false,...} JSON, so an
            // unknown command surfaces in-band rather than as an exception.
            String raw = compiler.command("{\"cmd\":\"nope\"}");
            assertTrue(raw.contains("\"ok\":false"), raw);
        }
    }

    @Test
    void artifactBytesReadsAFile() throws Exception {
        Path path = Files.createTempFile("wickra-compile", ".bin");
        byte[] want = {1, 2, 3, 4, 5, 42, (byte) 200, 0, (byte) 255};
        try {
            Files.write(path, want);
            try (Compiler compiler = new Compiler()) {
                byte[] got = compiler.artifactBytes(path.toString());
                assertArrayEquals(want, got);
            }
        } finally {
            Files.deleteIfExists(path);
        }
    }

    @Test
    void artifactBytesMissingFileThrows() {
        try (Compiler compiler = new Compiler()) {
            assertThrows(IllegalStateException.class,
                    () -> compiler.artifactBytes("does-not-exist-7c1f.bin"));
        }
    }
}
