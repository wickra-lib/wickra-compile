<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Compile — compile a strategy spec into a standalone deployable" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/ci.svg)](https://github.com/wickra-lib/wickra-compile/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-compile)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/pypi.svg)](https://pypi.org/project/wickra-compile/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-compile/license.svg)](https://github.com/wickra-lib/wickra-compile#license)

# Wickra Compile — Python

---

**Part of the [Wickra ecosystem](#ecosystem): — for Python. `pip install wickra-compile` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for [`wickra-compile`](https://github.com/wickra-lib/wickra-compile):
compile a Wickra strategy spec into a standalone deployable (a WASM module, a
self-contained binary, or a `no_std` artifact) and get a **deterministic
manifest** — byte-identical to every other language binding.

## Install

```bash
pip install wickra-compile
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

Built with [maturin](https://www.maturin.rs/) and [PyO3](https://pyo3.rs/)
(abi3, Python 3.9+).

## Quick start

```python
import json
from wickra_compile import Compiler

spec = {
    "strategy": { "symbol": "BTCUSDT", "timeframe": "1h",
        "indicators": {"fast": {"type": "Sma", "params": [10]},
                       "slow": {"type": "Sma", "params": [30]}},
        "entry": {"cross_above": ["fast", "slow"]},
        "exit":  {"cross_below": ["fast", "slow"]},
        "sizing": {"type": "fixed_qty", "qty": 1} },
    "target": {"kind": "wasm"}, "opt_level": "size",
}

c = Compiler()
out = json.loads(c.command(json.dumps({"cmd": "compile", "dry_run": True, "spec": spec})))
print(out["manifest"]["project_hash"])   # deterministic across runs and languages
```

`command` mirrors `Compiler::command_json`: the commands are `compile`,
`targets`, `version`, `artifact_bytes` and `reset`. Domain errors come back
in-band as `{"ok": false, "error": ...}` JSON. A `compile` with `dry_run: true`
needs no toolchain; `dry_run: false` invokes `cargo`.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-compile/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-compile>
- **Docs** (guides, spec reference, cookbook): <https://compile.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-compile/tree/main/examples/python)

Wickra Compile ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-compile/blob/main/SECURITY.md>.

## Disclaimer

Wickra Compile is a code-generation tool, provided "as is" without warranty of
any kind. It generates projects and can invoke `cargo` to build them — run it
only on specs you trust. Nothing here is financial advice; compiled strategies
are your responsibility, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-compile/blob/main/LICENSE-MIT) at your option.
