package org.wickra.compile;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

/**
 * Operating-mode equivalence through the binding: the manifest a dry run
 * describes is the manifest a real build builds. {@code compile} runs two ways
 * -- {@code dry_run: true} stops after codegen and returns the manifest,
 * {@code dry_run: false} writes the project and invokes {@code cargo} on it --
 * and both must carry the same manifest, with {@code built} and {@code path}
 * the only difference. The no_std spec is the one built for real: its
 * generated project has no dependencies, so it compiles in seconds, and it
 * needs only the {@code thumbv7em-none-eabihf} target the CI jobs install. The
 * core pins this in Rust (operating_modes.rs); this checks the boundary the
 * Java binding crosses. The JSON is handled as text: the {@code "manifest"}
 * object is cut out by brace depth and compared verbatim, which is exactly the
 * byte equality the family claims.
 */
class ModeTest {
    /** The {@code "manifest":{...}} object of an artifact response, verbatim. */
    static String manifestOf(String artifact) {
        int at = artifact.indexOf("\"manifest\":");
        if (at < 0) {
            throw new IllegalStateException("no manifest in: " + artifact);
        }
        int start = artifact.indexOf('{', at);
        int depth = 0;
        for (int i = start; i < artifact.length(); i++) {
            char c = artifact.charAt(i);
            if (c == '{') {
                depth++;
            } else if (c == '}') {
                depth--;
                if (depth == 0) {
                    return artifact.substring(start, i + 1);
                }
            }
        }
        throw new IllegalStateException("unterminated manifest in: " + artifact);
    }

    /** The part of an artifact response before its manifest: the artifact's own
     * fields, without the manifest's file entries and their {@code path} keys. */
    static String head(String artifact) {
        int at = artifact.indexOf("\"manifest\":");
        return at < 0 ? artifact : artifact.substring(0, at);
    }

    @Test
    void aRealBuildCarriesTheDryRunManifest() throws IOException {
        Path golden = Path.of("..", "..", "golden");
        String spec = Files.readString(golden.resolve("specs").resolve("no_std_blink.json"));
        String expected = Files.readString(golden.resolve("expected").resolve("no_std_blink.json")).strip();

        try (Compiler compiler = new Compiler()) {
            String dry = compiler.command("{\"cmd\":\"compile\",\"dry_run\":true,\"spec\":" + spec + "}");
            assertTrue(dry.contains("\"built\":false"), dry);
            assertFalse(head(dry).contains("\"path\":"), dry);
            assertEquals(expected, manifestOf(dry), "the dry-run manifest does not match the blessed golden");

            Path outDir = Files.createTempDirectory("wickra-compile-modes-");
            try {
                String outJson = "\"" + outDir.toString().replace('\\', '/') + "\"";
                String built = compiler.command(
                        "{\"cmd\":\"compile\",\"dry_run\":false,\"out_dir\":" + outJson + ",\"spec\":" + spec + "}");
                assertTrue(built.contains("\"built\":true"), built);
                int at = head(built).indexOf("\"path\":\"");
                assertTrue(at >= 0, built);
                String artifact = built.substring(at + 8, built.indexOf('"', at + 8));
                assertTrue(Files.isRegularFile(Path.of(artifact)), "artifact missing at " + artifact);
                assertEquals(manifestOf(dry), manifestOf(built), "the built manifest differs from the dry-run manifest");
            } finally {
                try (Stream<Path> walk = Files.walk(outDir)) {
                    walk.sorted(Comparator.reverseOrder()).forEach(p -> p.toFile().delete());
                }
            }
        }
    }
}
