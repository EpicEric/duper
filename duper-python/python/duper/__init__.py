r"""Utilities for converting to and from Python types into the Duper format.

:mod:`duper` exposes an API similar to :mod:`json` and :mod:`pickle`."""

from ._duper import (
    dumps,
    dump,
    loads,
    load,
    Duper,
)

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
    "Duper",
]
