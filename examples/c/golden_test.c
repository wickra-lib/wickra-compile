/* Cross-language golden parity and operating-mode equivalence, from C.
 *
 * Golden: compile each committed golden/specs/*.json as a dry run and assert
 * the manifest equals golden/expected/<spec>.json byte-for-byte -- the
 * `"manifest"` object is cut out of the artifact response by brace depth. The
 * ABI returns the core's compact command output verbatim, so byte equality is
 * the exact cross-language parity check -- the same one Python, Node, Go, C#,
 * Java, R and WASM make. binary_daemon embeds a CSV resolved relative to the
 * working directory, so it is covered by the Rust golden, not here.
 *
 * Operating mode: `compile` runs two ways -- `dry_run: true` stops after
 * codegen and returns the manifest, `dry_run: false` writes the project and
 * invokes cargo on it -- and both must carry the same manifest, with `built`
 * and `path` the only difference. The no_std spec is the one built for real:
 * its generated project has no dependencies, so it compiles in seconds, and
 * it needs only the thumbv7em-none-eabihf target. The core pins this in Rust
 * (operating_modes.rs); this checks the boundary six of the ten language
 * reaches cross.
 *
 * C has no directory API that is portable between POSIX and Windows, so the
 * spec list is globbed by CMake at configure time and written into
 * golden_specs.h. That keeps the property the other bindings get from a
 * runtime glob: a spec added to the corpus is covered here without editing
 * this file.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT, MODES_OUT_DIR */
#include "wickra_compile.h"

/* Read a whole file. Caller frees. Returns NULL and reports on failure. */
static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

/* Trim ASCII whitespace in place and return the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

/* A growable string. */
typedef struct {
    char *buf;
    size_t len;
    size_t cap;
} Str;

static int str_push(Str *s, const char *text, size_t n) {
    if (s->len + n + 1 > s->cap) {
        size_t cap = s->cap ? s->cap : 4096;
        while (cap < s->len + n + 1) {
            cap *= 2;
        }
        char *grown = (char *)realloc(s->buf, cap);
        if (!grown) {
            return 0;
        }
        s->buf = grown;
        s->cap = cap;
    }
    memcpy(s->buf + s->len, text, n);
    s->len += n;
    s->buf[s->len] = '\0';
    return 1;
}

static int str_puts(Str *s, const char *text) { return str_push(s, text, strlen(text)); }

/* Apply one read-only command through the two-call length protocol. Caller
 * frees. The hub caches the response of a mutating command between the length
 * call and the delivering call, so a real build runs once, not twice. */
static char *run(WickraCompiler *compiler, const char *cmd) {
    int32_t len = wickra_compile_command(compiler, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed with code %d\n", (int)len);
        return NULL;
    }
    char *out = (char *)malloc((size_t)len + 1);
    if (!out) {
        return NULL;
    }
    if (wickra_compile_command(compiler, cmd, out, (size_t)len + 1) < 0) {
        free(out);
        return NULL;
    }
    return out;
}

/* The `"manifest":{...}` object of an artifact response, cut out by brace
 * depth. Returns a pointer into `artifact` and NUL-terminates it in place. */
static char *manifest_of(char *artifact) {
    char *at = strstr(artifact, "\"manifest\":");
    if (!at) {
        return NULL;
    }
    char *start = strchr(at, '{');
    int depth = 0;
    for (char *q = start; q && *q; q++) {
        if (*q == '{') {
            depth++;
        } else if (*q == '}') {
            depth--;
            if (depth == 0) {
                q[1] = '\0';
                return start;
            }
        }
    }
    return NULL;
}

/* Whether the artifact response carries a top-level `path` -- the part before
 * `"manifest":`, since the manifest's file entries have `path` keys too. */
static int head_has_path(const char *artifact) {
    const char *manifest = strstr(artifact, "\"manifest\":");
    const char *path = strstr(artifact, "\"path\":");
    return path && (!manifest || path < manifest);
}

/* `{"cmd":"compile","dry_run":<true|false>[,"out_dir":...],"spec":<spec>}` on a
 * fresh handle. Caller frees. */
static char *compile_spec(const char *spec, int dry_run, const char *out_dir) {
    WickraCompiler *compiler = wickra_compile_new();
    if (!compiler) {
        fprintf(stderr, "wickra_compile_new returned null\n");
        return NULL;
    }
    Str cmd = {0};
    int ok = str_puts(&cmd, dry_run ? "{\"cmd\":\"compile\",\"dry_run\":true" : "{\"cmd\":\"compile\",\"dry_run\":false");
    if (ok && out_dir) {
        ok = str_puts(&cmd, ",\"out_dir\":\"") && str_puts(&cmd, out_dir) && str_puts(&cmd, "\"");
    }
    ok = ok && str_puts(&cmd, ",\"spec\":") && str_puts(&cmd, spec) && str_puts(&cmd, "}");
    if (!ok) {
        free(cmd.buf);
        wickra_compile_free(compiler);
        return NULL;
    }
    char *out = run(compiler, cmd.buf);
    free(cmd.buf);
    wickra_compile_free(compiler);
    return out;
}

int main(void) {
    printf("wickra-compile %s: golden parity + operating modes over %zu spec(s)\n",
           wickra_compile_version(), (size_t)GOLDEN_SPEC_COUNT);
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "golden corpus not found\n");
        return 1;
    }
    int failures = 0;
    for (size_t i = 0; GOLDEN_SPECS[i]; i++) {
        if (strcmp(GOLDEN_SPECS[i], "binary_daemon.json") == 0) {
            continue; /* embeds a CSV relative to the working directory: the Rust golden's */
        }
        char spec_path[1024];
        char expected_path[1024];
        snprintf(spec_path, sizeof spec_path, "%s/specs/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        snprintf(expected_path, sizeof expected_path, "%s/expected/%s", GOLDEN_DIR, GOLDEN_SPECS[i]);
        char *spec = slurp(spec_path);
        char *expected_raw = slurp(expected_path);
        if (!spec || !expected_raw) {
            free(spec);
            free(expected_raw);
            failures++;
            continue;
        }
        char *expected = trim(expected_raw);
        char *dry = compile_spec(spec, 1, NULL);
        char *manifest = dry ? manifest_of(dry) : NULL;
        if (!manifest) {
            fprintf(stderr, "%s: command failed\n", GOLDEN_SPECS[i]);
            failures++;
        } else if (strcmp(manifest, expected) != 0) {
            fprintf(stderr, "%s: the dry-run manifest does not match the blessed golden\n", GOLDEN_SPECS[i]);
            failures++;
        } else {
            printf("  %s: ok\n", GOLDEN_SPECS[i]);
        }
        free(dry);
        free(spec);
        free(expected_raw);
    }

    /* Operating mode: the no_std spec built for real. */
    char *spec = slurp(GOLDEN_DIR "/specs/no_std_blink.json");
    char *dry = spec ? compile_spec(spec, 1, NULL) : NULL;
    char *built = spec ? compile_spec(spec, 0, MODES_OUT_DIR) : NULL;
    if (!dry || !built) {
        fprintf(stderr, "no_std_blink: command failed\n");
        failures++;
    } else if (!strstr(dry, "\"built\":false") || head_has_path(dry)) {
        fprintf(stderr, "no_std_blink: a dry run must not build: %s\n", dry);
        failures++;
    } else if (!strstr(built, "\"built\":true")) {
        fprintf(stderr, "no_std_blink: the build did not run: %s\n", built);
        failures++;
    } else {
        /* The artifact's own path precedes the manifest; the manifest's file
         * entries carry paths of their own, so the search stops there. */
        const char *at = head_has_path(built) ? strstr(built, "\"path\":\"") : NULL;
        char artifact[1024] = {0};
        if (at) {
            at += 8;
            size_t n = strcspn(at, "\"");
            if (n < sizeof artifact) {
                memcpy(artifact, at, n);
            }
        }
        FILE *f = artifact[0] ? fopen(artifact, "rb") : NULL;
        char *dm = manifest_of(dry);
        char *bm = manifest_of(built);
        if (!f) {
            fprintf(stderr, "no_std_blink: artifact missing at %s\n", artifact);
            failures++;
        } else if (!dm || !bm || strcmp(dm, bm) != 0) {
            fprintf(stderr, "no_std_blink: the built manifest differs from the dry-run manifest\n");
            failures++;
        } else {
            printf("  no_std_blink: a real build carries the dry-run manifest\n");
        }
        if (f) {
            fclose(f);
        }
    }
    free(dry);
    free(built);
    free(spec);
    if (failures) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("ok\n");
    return 0;
}
