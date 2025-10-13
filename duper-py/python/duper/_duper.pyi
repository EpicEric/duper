from typing import Any
from io import TextIOBase

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
]

def dumps(
    obj: Any,
    *,
    indent: int | None = None,
    strip_identifiers: bool = False,
) -> str:
    """Serialize obj as a Duper value formatted str."""

def dump(
    obj: Any,
    fp: TextIOBase,
    *,
    indent: int | None = None,
    strip_identifiers: bool = False,
) -> None:
    """Serialize obj as a Duper value formatted stream to fp (a file-like object)."""

def loads(s: str, *, parse_any: bool = False) -> Any:
    """Deserialize s (a str instance containing a Duper object or array) to a Python object."""

def load(fp: TextIOBase, *, parse_any: bool = False) -> Any:
    """Deserialize fp (a file-like object containing a Duper object or array) to a Python object."""
