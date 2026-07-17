# wickra-compile (Python)

Python bindings for [`wickra-compile`](https://github.com/wickra-lib/wickra-compile):
compile a Wickra strategy spec into a standalone deployable (a WASM module, a
self-contained binary, or a `no_std` artifact) and get a **deterministic
manifest** — byte-identical to every other language binding.

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

## Install

```bash
pip install wickra-compile
```

Built with [maturin](https://www.maturin.rs/) and [PyO3](https://pyo3.rs/)
(abi3, Python 3.9+).
