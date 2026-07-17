package org.wickra.compile;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Compiles a Wickra strategy spec into a standalone deployable and a
 * deterministic manifest, driven by JSON commands over the Wickra C ABI
 * (FFM/Panama). Construct one, drive it with command JSON ({@code compile},
 * {@code targets}, {@code version}, {@code artifact_bytes}, {@code reset}) and
 * read back the response JSON — the same protocol as the CLI and every other
 * binding.
 */
public final class Compiler implements AutoCloseable {
    private static final Pattern HANDLE = Pattern.compile("\"handle\"\\s*:\\s*(\\d+)");
    private static final Pattern LEN = Pattern.compile("\"len\"\\s*:\\s*(\\d+)");

    private MemorySegment handle;

    /** Construct a compiler handle. */
    public Compiler() {
        try {
            MemorySegment created = (MemorySegment) Native.NEW.invokeExact();
            if (created.address() == 0) {
                throw new IllegalStateException("wickra-compile: allocation failed");
            }
            this.handle = created;
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    /**
     * Apply a command JSON and return the response JSON. Uses the C ABI's
     * length-out protocol: a first call learns the length, then the response is
     * read into a caller-owned buffer. Domain errors (a bad command, an unknown
     * command, an invalid spec) come back in-band as {@code {"ok":false,...}}
     * JSON, not as an exception.
     */
    public String command(String cmdJson) {
        if (handle == null) {
            throw new IllegalStateException("compiler is closed");
        }
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment cmd = arena.allocateFrom(cmdJson);
            int len = (int) Native.COMMAND.invokeExact(handle, cmd, MemorySegment.NULL, 0L);
            if (len < 0) {
                throw new IllegalStateException("wickra-compile: command failed (code " + len + ")");
            }
            MemorySegment buf = arena.allocate(len + 1L);
            int written = (int) Native.COMMAND.invokeExact(handle, cmd, buf, (long) (len + 1));
            return buf.getString(0);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    /**
     * Read the raw bytes of a file through the C ABI byte reader.
     *
     * @throws IllegalStateException if the file could not be read
     */
    public byte[] artifactBytes(String path) {
        if (handle == null) {
            throw new IllegalStateException("compiler is closed");
        }
        String response = command("{\"cmd\":\"artifact_bytes\",\"path\":\"" + escape(path) + "\"}");
        if (response.contains("\"ok\":false") || response.contains("\"error\"")) {
            throw new IllegalStateException("wickra-compile: " + response);
        }
        long byteHandle = extract(HANDLE, response, "handle");
        int len = (int) extract(LEN, response, "len");
        if (len == 0) {
            return new byte[0];
        }
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment buf = arena.allocate(len);
            long n = (long) Native.ARTIFACT_READ.invokeExact(handle, byteHandle, buf, (long) len);
            if (n < 0) {
                throw new IllegalStateException("wickra-compile: artifact read failed (code " + n + ")");
            }
            return buf.asSlice(0, n).toArray(ValueLayout.JAVA_BYTE);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    /** The library version. */
    public static String version() {
        try {
            MemorySegment ptr = (MemorySegment) Native.VERSION.invokeExact();
            return ptr.reinterpret(Long.MAX_VALUE).getString(0);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    /** Free the native compiler handle. */
    @Override
    public void close() {
        if (handle != null) {
            try {
                Native.FREE.invokeExact(handle);
            } catch (Throwable t) {
                throw new RuntimeException(t);
            }
            handle = null;
        }
    }

    private static long extract(Pattern pattern, String json, String field) {
        Matcher matcher = pattern.matcher(json);
        if (!matcher.find()) {
            throw new IllegalStateException("wickra-compile: missing " + field + " in response: " + json);
        }
        return Long.parseLong(matcher.group(1));
    }

    /** Escape a string for embedding in a JSON string literal. */
    private static String escape(String s) {
        return s.replace("\\", "\\\\").replace("\"", "\\\"");
    }
}
