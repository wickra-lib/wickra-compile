# Security policy

## Supported versions

Wickra Compile is pre-1.0; only the latest `0.x` release line receives security
fixes.

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a vulnerability

Please report suspected vulnerabilities privately via
[GitHub Security Advisories](https://github.com/wickra-lib/wickra-compile/security/advisories/new),
not in public issues. We aim to acknowledge within a few days and to coordinate a
fix and disclosure timeline with you.

## Codegen note (read before running the build step)

Wickra Compile **generates code** and can **invoke `cargo` on it**. Treat it like
any code generator that runs a compiler:

- Run it only on **trusted specs**. A spec becomes source that a build step
  compiles and runs (`build.rs`, proc-macros, the resulting artifact).
- Code generation itself does **no network access** and spawns no processes; only
  the optional build step (the `build` feature, or the CLI without `--dry-run`)
  invokes `cargo`, which then fetches and builds dependencies.
- Compile untrusted third-party specs in a sandbox with no secrets and restricted
  network/file-system access.

See [THREAT_MODEL.md](THREAT_MODEL.md) for the full analysis.
