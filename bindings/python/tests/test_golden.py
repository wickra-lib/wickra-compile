"""The manifest is deterministic and independent of optimisation-only fields."""

import json
from pathlib import Path

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

# The cross-language golden: every binding reproduces the exact project_hash
# pinned in golden/expected. binary_daemon embeds a CSV resolved relative to the
# working directory, so it is covered by the Rust golden, not here.
GOLDEN = Path(__file__).resolve().parents[3] / "golden"
SPECS = ("sma_cross", "ema_trend", "rsi_reversion", "no_std_blink")


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


def test_every_golden_spec_reproduces_its_expected_project_hash() -> None:
    for name in SPECS:
        # Splice the raw spec JSON verbatim so a float like 1.0 keeps its exact
        # form (matching the CLI and every other binding).
        spec_raw = (GOLDEN / "specs" / f"{name}.json").read_text()
        expected = json.loads((GOLDEN / "expected" / f"{name}.json").read_text())
        out = Compiler().command(f'{{"cmd":"compile","dry_run":true,"spec":{spec_raw}}}')
        got = json.loads(out)["manifest"]["project_hash"]
        assert got == expected["project_hash"], name
