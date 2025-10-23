from typing import Any, Literal, TypeAlias, overload
from io import TextIOBase

from pydantic import BaseModel

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
    "Duper",
]

class Duper:
    """A frozen Duper value with an optional identifier.

    Both of these are exposed as properties ``value`` and ``identifier``,
    respectively."""

DuperType: TypeAlias = (
    dict[str, DuperType]
    | list[DuperType]
    | tuple[DuperType, ...]
    | str
    | bytes
    | int
    | float
    | bool
    | None
)

def dumps(
    obj: Any,
    *,
    indent: str | int | None = None,
    strip_identifiers: bool = False,
) -> str:
    """Serialize ``obj`` to a Duper value formatted ``str``.

    If ``indent`` is a positive integer, then Duper array elements and
    object members will be pretty-printed with that indent level. The
    indent may also be specified as a ``str`` containing spaces and/or
    tabs. ``None`` is the most compact representation.

    If ``strip_identifiers`` is ``True``, then this function will strip
    all identifiers from the serialized value."""

def dump(
    obj: Any,
    fp: TextIOBase,
    *,
    indent: str | int | None = None,
    strip_identifiers: bool = False,
) -> None:
    """Serialize ``obj`` as a Duper value formatted stream to ``fp`` (a
    ``.write()``-supporting file-like object).

    If ``indent`` is a positive integer, then Duper array elements and
    object members will be pretty-printed with that indent level. The
    indent may also be specified as a ``str`` containing spaces and/or
    tabs. ``None`` is the most compact representation.

    If ``strip_identifiers`` is ``True``, then this function will strip
    all identifiers from the serialized value."""

@overload
def loads(
    s: str, *, parse_any: Literal[False] = False
) -> BaseModel | list[DuperType]: ...
@overload
def loads(
    s: str, *, parse_any: Literal[True]
) -> BaseModel | list[DuperType] | DuperType: ...
def loads(
    s: str, *, parse_any: bool = False
) -> BaseModel | list[DuperType] | DuperType:
    """Deserialize ``s`` (a ``str`` instance containing a Duper object or
    array) to a Pydantic model.

    If ``parse_any`` is ``True``, then this function will also deserialize
    types other than objects and arrays.
    """

@overload
def load(
    fp: TextIOBase, *, parse_any: Literal[False] = False
) -> BaseModel | list[DuperType]: ...
@overload
def load(
    fp: TextIOBase, *, parse_any: Literal[True]
) -> BaseModel | list[DuperType] | DuperType: ...
def load(
    fp: TextIOBase, *, parse_any: bool = False
) -> BaseModel | list[DuperType] | DuperType:
    """Deserialize ``fp`` (a ``.read()``-supporting file-like object
    containing a Duper object or array) to a Pydantic model.

    If ``parse_any`` is ``True``, then this function will also deserialize
    types other than objects and arrays."""
