"""A runnable Python example: compile a strategy spec (dry run) and print its
deterministic manifest.

    pip install wickra-compile
    python examples/python/compile.py

Every language example uses the same spec and prints the same project_hash —
that is the cross-language guarantee.
"""

import json

from wickra_compile import Compiler

SPEC = {
    "strategy": {
        "symbol": "btcusdt",
        "timeframe": "1h",
        "indicators": {
            "fast": {"type": "Sma", "params": [10]},
            "slow": {"type": "Sma", "params": [30]},
        },
        "entry": {"cross_above": ["fast", "slow"]},
        "exit": {"cross_below": ["fast", "slow"]},
        "sizing": {"type": "fixed_qty", "qty": 1},
    },
    "target": {"kind": "wasm"},
    "crate_name": "demo",
}


def main() -> None:
    compiler = Compiler()
    out = json.loads(
        compiler.command(json.dumps({"cmd": "compile", "dry_run": True, "spec": SPEC}))
    )
    manifest = out["manifest"]

    print(f"wickra-compile {Compiler.version()}")
    print(f"crate: {manifest['crate_name']}")
    print(f"files: {len(manifest['files'])}")
    print(f"project_hash: {manifest['project_hash']}")


if __name__ == "__main__":
    main()
