"""Smoke test: construct a compiler, dry-run compile, parse the manifest."""

import json

from wickra_compile import Compiler, __version__

SPEC = {
    "strategy": {
        "symbol": "x",
        "timeframe": "1h",
        "indicators": {"f": {"type": "Ema", "params": [3]}},
        "entry": {"cross_above": ["f", "f"]},
        "exit": {"cross_below": ["f", "f"]},
        "sizing": {"type": "fixed_qty", "qty": 1},
    },
    "target": {"kind": "wasm"},
    "crate_name": "demo",
}


def _dry_run(compiler: Compiler) -> dict:
    cmd = {"cmd": "compile", "dry_run": True, "spec": SPEC}
    return json.loads(compiler.command(json.dumps(cmd)))


def test_dry_run_returns_manifest() -> None:
    out = _dry_run(Compiler())
    assert out["built"] is False
    assert out["manifest"]["crate_name"] == "demo"
    assert "project_hash" in out["manifest"]


def test_deterministic_across_instances() -> None:
    a = _dry_run(Compiler())["manifest"]
    b = _dry_run(Compiler())["manifest"]
    assert a == b


def test_targets_and_version() -> None:
    compiler = Compiler()
    targets = json.loads(compiler.command(json.dumps({"cmd": "targets"})))
    assert "wasm" in targets["targets"]
    assert "thumbv6m-none-eabi" in targets["mcus"]
    assert Compiler.version() == __version__


def test_unknown_command_is_in_band_error() -> None:
    out = json.loads(Compiler().command(json.dumps({"cmd": "nope"})))
    assert out["ok"] is False
