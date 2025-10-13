from dataclasses import dataclass
from collections import deque
from datetime import datetime
from decimal import Decimal
from enum import Enum
from ipaddress import (
    IPv4Address,
    IPv4Interface,
    IPv4Network,
    IPv6Address,
    IPv6Interface,
    IPv6Network,
)
import re
from typing import NamedTuple
from typing_extensions import TypedDict
from uuid import UUID

from pydantic import ByteSize

from duper.pydantic import BaseModel


def test_pydantic_model_dump_simple():
    class Foo(BaseModel):
        int: int
        list: list[str]
        tuple: tuple[bool, None]
        map: dict[str, float]
        bytes: bytes

    val = Foo(
        int=42,
        list=["hello", '"world"'],
        tuple=(True, None),
        map={"some key with spaces": 3.14e30},
        bytes=b"abcde",
    )

    val_dump = val.model_dump(mode="duper")

    assert (
        val_dump
        == """Foo({int: 42, list: ["hello", r#""world""#], tuple: (true, null), map: {"some key with spaces": 3.14e30}, bytes: b"abcde"})"""
    )

    val2 = Foo.model_validate_duper(val_dump)

    assert val == val2


def test_pydantic_model_dump_complex():
    class MyTuple(NamedTuple):
        x: int
        y: int

    class Color(Enum):
        RED = 1
        GREEN = 2
        BLUE = 3

    class Point2D(TypedDict):
        x: int
        y: int
        label: str

    class Submodel(BaseModel):
        address4: IPv4Address
        interface4: IPv4Interface
        network4: IPv4Network
        address6: IPv6Address
        interface6: IPv6Interface
        network6: IPv6Network

    @dataclass
    class Regex:
        pattern: re.Pattern
        matches: list[str] | None = None

    class Bar(BaseModel):
        datetime: datetime
        uuid: UUID
        deque: deque[str]
        named_tuple: MyTuple
        set: set[int]
        bytesize: ByteSize
        decimal: Decimal
        enum: Color
        typeddict: Point2D
        regex: Regex
        sub: Submodel

    val = Bar(
        datetime="2025-10-12T20:01:28.400086",
        uuid="a708f86d-ee5b-4ce8-b505-8f59d3d26850",
        deque=deque(),
        named_tuple=(34, 35),
        set={1, 2, 1, 4},
        bytesize="3000 KiB",
        decimal=Decimal("12.34"),
        enum=Color.GREEN,
        typeddict={"x": 1, "y": 2, "label": "good"},
        regex=Regex(pattern=re.compile(r"^Hello w.rld!$")),
        sub=Submodel(
            address4=IPv4Address("192.168.0.1"),
            interface4=IPv4Interface("192.168.0.2"),
            network4=IPv4Network("192.168.0.0/24"),
            address6=IPv6Address("2001:db8::1"),
            interface6=IPv6Interface("2001:db8::2"),
            network6=IPv6Network("2001:db8::/128"),
        ),
    )

    val_dump = val.model_dump(mode="duper")

    assert (
        val_dump
        == """Bar({datetime: Datetime("2025-10-12T20:01:28.400086"), uuid: Uuid("a708f86d-ee5b-4ce8-b505-8f59d3d26850"), deque: Deque([]), named_tuple: (34, 35), set: Set([1, 2, 4]), bytesize: Bytesize(3072000), decimal: Decimal("12.34"), enum: Color(2), typeddict: {x: 1, y: 2, label: "good"}, regex: Regex({pattern: Pattern("^Hello w.rld!$"), matches: null}), sub: Submodel({address4: Ipv4address("192.168.0.1"), interface4: Ipv4interface("192.168.0.2/32"), network4: Ipv4network("192.168.0.0/24"), address6: Ipv6address("2001:db8::1"), interface6: Ipv6interface("2001:db8::2/128"), network6: Ipv6network("2001:db8::/128")})})"""
    )

    val2 = Bar.model_validate_duper(val_dump)

    assert val == val2
