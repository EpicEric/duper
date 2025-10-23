r"""Utilities for converting to and from Python types into the Duper format.

:mod:`duper` exposes an API similar to :mod:`json` and :mod:`pickle`, except
that custom Pydantic ``BaseModel``s are returned.."""

from ._duper import (
    dumps,
    dump,
    loads,
    load,
    Duper,
    DuperType,
)
from .pydantic import BaseModel

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
    "Duper",
    "DuperType",
    "BaseModel",
]
