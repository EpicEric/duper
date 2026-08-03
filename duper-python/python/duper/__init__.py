r"""Utilities for converting to and from Python types into the Duper format.

:mod:`duper` exposes an API similar to :mod:`json` and :mod:`pickle`, except
that custom Pydantic ``BaseModel``s are returned.."""

from ._duper import (
    Duper,
    DuperType,
    TemporalString,
    dump,
    dumps,
    load,
    loads,
)
from .pydantic import BaseModel

__all__ = [
    "BaseModel",
    "Duper",
    "DuperType",
    "TemporalString",
    "dump",
    "dumps",
    "load",
    "loads",
]
