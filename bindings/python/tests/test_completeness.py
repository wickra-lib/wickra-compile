"""The Python surface exposes exactly the documented API."""

import wickra_compile
from wickra_compile import Compiler


def test_module_exports() -> None:
    assert set(wickra_compile.__all__) == {"Compiler", "__version__"}


def test_compiler_methods() -> None:
    for name in ("command", "version"):
        assert hasattr(Compiler, name)


def test_version_is_a_string() -> None:
    assert isinstance(wickra_compile.__version__, str)
    assert wickra_compile.__version__
