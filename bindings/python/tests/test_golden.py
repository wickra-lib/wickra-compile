"""The manifest is deterministic and independent of optimisation-only fields."""

import json

from wickra_compile import Compiler

BASE = {
    "strategy": {
        "symbol": "btc",
        "timeframe": "1h",
        "indicators": {"f": {"type": "Sma", "params": [10]}},
        "entry": {"cross_above": ["f", "f"]},
        "exit": {"cross_below": ["f", "f"]},
        "sizing": {"type": "fixed_qty", "qty": 1},
    },
    "target": {"kind": "binary"},
}


def _manifest(spec: dict) -> dict:
    out = Compiler().command(json.dumps({"cmd": "compile", "dry_run": True, "spec": spec}))
    return json.loads(out)["manifest"]


def test_same_spec_same_project_hash() -> None:
    a = _manifest(BASE)
    b = _manifest(BASE)
    assert a["project_hash"] == b["project_hash"]
    assert a["spec_hash"] == b["spec_hash"]


def test_target_changes_project_hash() -> None:
    binary = _manifest(BASE)
    wasm_spec = {**BASE, "target": {"kind": "wasm"}}
    wasm = _manifest(wasm_spec)
    assert binary["project_hash"] != wasm["project_hash"]
