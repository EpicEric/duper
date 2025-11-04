from typing import Any, Literal, TypeAlias, overload
from io import TextIOBase

from pydantic import BaseModel

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
    "Duper",
    "DuperType",
]

class Duper:
    """An annotation of a Duper's optional identifier. Used to annotate fields in ``duper.BaseModel``.

    >>> from typing import Annotated
    >>> from decimal import Decimal
    >>> from duper import BaseModel, Duper
    >>> class Foo(BaseModel):
    ...     regular: str | None
    ...     identified: Annotated[str | None, Duper("Bar")]
    ...     unidentified: Annotated[Decimal, Duper(None)]
    >>> obj = Foo(
    ...     regular="12",
    ...     identified="34",
    ...     unidentified=Decimal("56.7"),
    >>> )
    >>> obj.model_dump(mode="duper")
    <<< "Foo({regular: \\"12\\", identified: Bar(\\"34\\"), unidentified: \\"56.7\\"})"
    """

    def __init__(self, identifier: str | None) -> None: ...
    @property
    def identifier(self) -> str: ...

DuperType: TypeAlias = "dict[str, DuperType] | list[DuperType] | tuple[DuperType, ...] | str | bytes | int | float | bool | None"
"""All possible Python return types for Duper values."""

def dumps(
    obj: Any,  # pyright: ignore[reportExplicitAny, reportAny]
    *,
    indent: str | int | None = None,
    strip_identifiers: bool = False,
    minify: bool = False,
) -> str:
    """Serialize ``obj`` to a Duper value formatted ``str``.

    If ``indent`` is a positive integer, then Duper array elements and
    object members will be pretty-printed with that indent level. The
    indent may also be specified as a ``str`` containing spaces and/or
    tabs. ``None`` is the most compact representation.

    If ``strip_identifiers`` is ``True``, then this function will strip
    all identifiers from the serialized value.

    If ``minify`` is ``True``, then this function will remove any extra
    whitespace. This is incompatible with the ``indent`` option."""

def dump(
    obj: Any,  # pyright: ignore[reportExplicitAny, reportAny]
    fp: TextIOBase,
    *,
    indent: str | int | None = None,
    strip_identifiers: bool = False,
    minify: bool = False,
) -> None:
    """Serialize ``obj`` as a Duper value formatted stream to ``fp`` (a
    ``.write()``-supporting file-like object).

    If ``indent`` is a positive integer, then Duper array elements and
    object members will be pretty-printed with that indent level. The
    indent may also be specified as a ``str`` containing spaces and/or
    tabs. ``None`` is the most compact representation.

    If ``strip_identifiers`` is ``True``, then this function will strip
    all identifiers from the serialized value.

    If ``minify`` is ``True``, then this function will remove any extra
    whitespace. This is incompatible with the ``indent`` option."""

@overload
def loads(
    s: str, *, parse_any: Literal[False] = False
) -> BaseModel | list[DuperType] | tuple[DuperType, ...]: ...
@overload
def loads(
    s: str, *, parse_any: Literal[True]
) -> BaseModel | list[DuperType] | tuple[DuperType, ...] | DuperType: ...
def loads(
    s: str, *, parse_any: bool = False
) -> BaseModel | list[DuperType] | tuple[DuperType, ...] | DuperType:
    """Deserialize ``s`` (a ``str`` instance containing a Duper object or
    array) to a Pydantic model.

    If ``parse_any`` is ``True``, then this function will also deserialize
    types other than objects, arrays, and tuples.
    """

@overload
def load(
    fp: TextIOBase, *, parse_any: Literal[False] = False
) -> BaseModel | list[DuperType] | tuple[DuperType, ...]: ...
@overload
def load(
    fp: TextIOBase, *, parse_any: Literal[True]
) -> BaseModel | list[DuperType] | tuple[DuperType, ...] | DuperType: ...
def load(
    fp: TextIOBase, *, parse_any: bool = False
) -> BaseModel | list[DuperType] | tuple[DuperType, ...] | DuperType:
    """Deserialize ``fp`` (a ``.read()``-supporting file-like object
    containing a Duper object or array) to a Pydantic model.

    If ``parse_any`` is ``True``, then this function will also deserialize
    types other than objects, arrays, and tuples."""
