"""Wickra Compile — compile a strategy spec into a standalone deployable.

Construct a :class:`Compiler`, drive it with command JSONs (``compile``,
``targets``, ``version``, ``artifact_bytes``, ``reset``), and read back the
response JSON. The same command protocol crosses every language binding, so this
Python front-end drives the exact same core — and returns the byte-identical
manifest — as the native CLI.
"""

from ._wickra_compile import Compiler, __version__

__all__ = ["Compiler", "__version__"]
