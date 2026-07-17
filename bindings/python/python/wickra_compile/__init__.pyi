"""Type stubs for the wickra_compile package."""

__version__: str

class Compiler:
    """A compiler driven by JSON commands."""

    def __init__(self) -> None:
        """Construct a compiler handle."""
        ...

    def command(self, cmd_json: str) -> str:
        """Apply a command JSON and return the response JSON.

        Domain errors (a bad spec, an unknown command) are reported in-band as
        ``{"ok": false, "error": ...}`` JSON rather than raised.
        """
        ...

    @staticmethod
    def version() -> str:
        """The library version."""
        ...
