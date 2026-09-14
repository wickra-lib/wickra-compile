"""Operating-mode equivalence through the binding: the manifest a dry run
describes is the manifest a real build builds.

``compile`` runs two ways -- ``dry_run: true`` stops after codegen and returns
the manifest, ``dry_run: false`` writes the project and invokes ``cargo`` on it
-- and both must carry the same manifest bytes, with ``built`` and ``path`` the
only difference. The no_std spec is the one built for real: its generated
project has no dependencies, so it compiles in seconds, and it needs only the
``thumbv7em-none-eabihf`` target the CI jobs install. The core pins this in Rust
(``operating_modes.rs``); this checks the boundary the Python binding crosses.
A missing corpus is a failure, not a skip.

Plain functions and plain asserts, so the module runs unchanged under pytest
(3.10 and up) and under ``run_without_pytest.py`` (the 3.9 row).
"""

import json
import os
import pathlib
import shutil
import tempfile

from wickra_compile import Compiler

GOLDEN = pathlib.Path(__file__).resolve().parents[3] / "golden"


def test_a_real_build_carries_the_dry_run_manifest() -> None:
    spec_raw = (GOLDEN / "specs" / "no_std_blink.json").read_text(encoding="utf-8")
    expected = json.loads((GOLDEN / "expected" / "no_std_blink.json").read_text(encoding="utf-8"))

    compiler = Compiler()
    dry = json.loads(compiler.command('{"cmd":"compile","dry_run":true,"spec":%s}' % spec_raw))
    assert dry["built"] is False
    assert "path" not in dry
    assert dry["manifest"] == expected

    out_dir = tempfile.mkdtemp(prefix="wickra-compile-modes-")
    try:
        built = json.loads(
            compiler.command(
                '{"cmd":"compile","dry_run":false,"out_dir":%s,"spec":%s}'
                % (json.dumps(out_dir.replace("\\", "/")), spec_raw)
            )
        )
        assert built.get("built") is True, built
        assert os.path.isfile(built["path"]), built["path"]
        assert built["manifest"] == dry["manifest"]
    finally:
        shutil.rmtree(out_dir, ignore_errors=True)
