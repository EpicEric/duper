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
from pathlib import Path
import re
from typing import NamedTuple
from typing_extensions import TypedDict
from uuid import UUID

from pydantic import ByteSize

from duper import BaseModel


def test_pydantic_simple():
    class Simple(BaseModel):
        int: int
        list: list[str]
        tuple: tuple[bool, None]
        map: dict[str, float]
        bytes: bytes

    val = Simple(
        int=42,
        list=["hello", '"world"'],
        tuple=(True, None),
        map={"some key with spaces": 3.14e30},
        bytes=b"abcde",
    )

    val_dump = val.model_dump(mode="duper")

    assert (
        val_dump
        == """Simple({int: 42, list: ["hello", r#""world""#], tuple: (true, null), map: {"some key with spaces": 3.14e30}, bytes: b"abcde"})"""
    )

    val2 = Simple.model_validate_duper(val_dump)

    assert val == val2


def test_pydantic_complex():
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

    class Complex(BaseModel):
        datetime: datetime
        uuid: UUID
        deque: deque[str]
        named_tuple: MyTuple
        set: set[int]
        bytesize: ByteSize
        decimal: Decimal
        enum: Color
        typeddict: Point2D
        path: Path
        regex: Regex
        sub: Submodel

    val = Complex(
        datetime="2025-10-12T20:01:28.400086",
        uuid="a708f86d-ee5b-4ce8-b505-8f59d3d26850",
        deque=deque(),
        named_tuple=(34, 35),
        set={1, 2, 1, 4},
        bytesize="3000 KiB",
        decimal=Decimal("12.34"),
        enum=Color.GREEN,
        typeddict={"x": 1, "y": 2, "label": "good"},
        path="/dev/null",
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
        == """Complex({datetime: DateTime("2025-10-12T20:01:28.400086"), uuid: Uuid("a708f86d-ee5b-4ce8-b505-8f59d3d26850"), deque: Deque([]), named_tuple: (34, 35), set: Set([1, 2, 4]), bytesize: ByteSize(3072000), decimal: Decimal("12.34"), enum: IPv4Address(2), typeddict: {x: 1, y: 2, label: "good"}, path: PosixPath("/dev/null"), regex: Regex({pattern: Pattern("^Hello w.rld!$"), matches: null}), sub: Submodel({address4: IPv4Address("192.168.0.1"), interface4: IPv4Interface("192.168.0.2/32"), network4: IPv4Network("192.168.0.0/24"), address6: IPv6Address("2001:db8::1"), interface6: IPv6Interface("2001:db8::2/128"), network6: IPv6Network("2001:db8::/128")})})"""
    )

    val2 = Complex.model_validate_duper(val_dump)

    assert val == val2


def test_pydantic_network():
    from pydantic import (
        AnyUrl,
        AnyHttpUrl,
        HttpUrl,
        AnyWebsocketUrl,
        WebsocketUrl,
        FileUrl,
        FtpUrl,
        PostgresDsn,
        CockroachDsn,
        AmqpDsn,
        RedisDsn,
        MongoDsn,
        KafkaDsn,
        NatsDsn,
        MySQLDsn,
        MariaDBDsn,
        ClickHouseDsn,
        SnowflakeDsn,
        EmailStr,
        NameEmail,
        IPvAnyAddress,
        IPvAnyInterface,
        IPvAnyNetwork,
    )

    class Network(BaseModel):
        any_url: AnyUrl
        any_http_url: AnyHttpUrl
        http_url: HttpUrl
        any_websocket_url: AnyWebsocketUrl
        websocket_url: WebsocketUrl
        file_url: FileUrl
        ftp_url: FtpUrl
        postgres_dsn: PostgresDsn
        cockroach_dsn: CockroachDsn
        amqp_dsn: AmqpDsn
        redis_dsn: RedisDsn
        mongo_dsn: MongoDsn
        kafka_dsn: KafkaDsn
        nats_dsn: NatsDsn
        mysql_dsn: MySQLDsn
        mariadb_dsn: MariaDBDsn
        clickhouse_dsn: ClickHouseDsn
        snowflake_dsn: SnowflakeDsn
        email_str: EmailStr
        name_email: NameEmail
        ip_address: IPvAnyAddress
        ip_interface: IPvAnyInterface
        ip_network: IPvAnyNetwork

    val = Network(
        any_url="https://example.com",
        any_http_url="https://pypi.org",
        http_url="https://example.com:8080/path?query=param#fragment",
        any_websocket_url="ws://example.com/chat",
        websocket_url="wss://example.com:8080/chat",
        file_url="file:///home/user/document.pdf",
        ftp_url="ftp://user:pass@ftp.example.com/file.txt",
        postgres_dsn="postgres://user:pass@localhost:5432/mydb",
        cockroach_dsn="cockroachdb://user:pass@localhost:26257/mydb",
        amqp_dsn="amqp://user:pass@localhost:5672/vhost",
        redis_dsn="redis://user:pass@localhost:6379/0",
        mongo_dsn="mongodb://user:pass@localhost:27017/mydb",
        kafka_dsn="kafka://localhost:9092",
        nats_dsn="nats://user:pass@localhost:4222",
        mysql_dsn="mysql://user:pass@localhost:3306/mydb",
        mariadb_dsn="mariadb://user:pass@localhost:3306/mydb",
        clickhouse_dsn="clickhouse://user:pass@localhost:8123/mydb",
        snowflake_dsn="snowflake://user:pass@account.region/db/schema?warehouse=wh",
        email_str="user@example.com",
        name_email="John Doe <john.doe@example.com>",
        ip_address="192.168.1.1",
        ip_interface="192.168.1.1/24",
        ip_network="192.168.0.0/16",
    )

    val_dump = val.model_dump(mode="duper")

    assert (
        val_dump
        == """Network({any_url: AnyUrl("https://example.com/"), any_http_url: AnyHttpUrl("https://pypi.org/"), http_url: HttpUrl("https://example.com:8080/path?query=param#fragment"), any_websocket_url: AnyWebsocketUrl("ws://example.com/chat"), websocket_url: WebsocketUrl("wss://example.com:8080/chat"), file_url: FileUrl("file:///home/user/document.pdf"), ftp_url: FtpUrl("ftp://user:pass@ftp.example.com/file.txt"), postgres_dsn: PostgresDsn("postgres://user:pass@localhost:5432/mydb"), cockroach_dsn: CockroachDsn("cockroachdb://user:pass@localhost:26257/mydb"), amqp_dsn: AmqpDsn("amqp://user:pass@localhost:5672/vhost"), redis_dsn: RedisDsn("redis://user:pass@localhost:6379/0"), mongo_dsn: MongoDsn("mongodb://user:pass@localhost:27017/mydb"), kafka_dsn: KafkaDsn("kafka://localhost:9092"), nats_dsn: NatsDsn("nats://user:pass@localhost:4222"), mysql_dsn: MySQLDsn("mysql://user:pass@localhost:3306/mydb"), mariadb_dsn: MariaDBDsn("mariadb://user:pass@localhost:3306/mydb"), clickhouse_dsn: ClickHouseDsn("clickhouse://user:pass@localhost:8123/mydb"), snowflake_dsn: SnowflakeDsn("snowflake://user:pass@account.region/db/schema?warehouse=wh"), email_str: "user@example.com", name_email: NameEmail("John Doe <john.doe@example.com>"), ip_address: IPv4Address("192.168.1.1"), ip_interface: IPv4Interface("192.168.1.1/24"), ip_network: IPv4Network("192.168.0.0/16")})"""
    )

    val2 = Network.model_validate_duper(val_dump)

    assert val == val2


def test_pydantic_extra():
    from pydantic_extra_types.color import Color
    from pydantic_extra_types.country import (
        CountryAlpha2,
        CountryAlpha3,
        CountryNumericCode,
        CountryShortName,
    )
    from pydantic_extra_types.cron import CronStr
    from pydantic_extra_types.payment import PaymentCardNumber
    from pydantic_extra_types.routing_number import ABARoutingNumber
    from pydantic_extra_types.mongo_object_id import MongoObjectId
    from pydantic_extra_types.language_code import (
        LanguageAlpha2,
        LanguageName,
        ISO639_3,
        ISO639_5,
    )
    from pydantic_extra_types.script_code import ISO_15924
    from pydantic_extra_types.s3 import S3Path
    from pydantic_extra_types.semantic_version import SemanticVersion
    from pydantic_extra_types.timezone_name import TimeZoneName
    from pydantic_extra_types.ulid import ULID

    class Extra(BaseModel):
        color: Color
        country_alpha2: CountryAlpha2
        country_alpha3: CountryAlpha3
        country_numeric_code: CountryNumericCode
        country_short_name: CountryShortName
        cron_str: CronStr
        payment_card_number: PaymentCardNumber
        aba_routing_number: ABARoutingNumber
        mongo_object_id: MongoObjectId
        language_alpha2: LanguageAlpha2
        language_name: LanguageName
        iso639_3: ISO639_3
        iso639_5: ISO639_5
        iso_15924: ISO_15924
        s3_path: S3Path
        semantic_version: SemanticVersion
        time_zone_name: TimeZoneName
        ulid: ULID

    val = Extra(
        color="#FF0000",
        country_alpha2="US",
        country_alpha3="BRA",
        country_numeric_code="840",
        country_short_name="Portugal",
        cron_str="*/5 0 * * *",
        payment_card_number="4111111111111111",
        aba_routing_number="021000021",
        mongo_object_id="64f8a4b1e4b0c8a9d4e5f6a7",
        language_alpha2="en",
        language_name="English",
        iso639_3="eng",
        iso639_5="gem",
        iso_15924="Latn",
        s3_path="s3://logs/2023/10/25/app.log",
        semantic_version="1.2.3",
        time_zone_name="America/New_York",
        ulid="01F9Z3ZQJZ9QJZ9QJZ9QJZ9QJZ",
    )

    val_dump = val.model_dump(mode="duper")

    assert (
        val_dump
        == """Extra({color: Color("red"), country_alpha2: CountryAlpha2("US"), country_alpha3: CountryAlpha3("BRA"), country_numeric_code: CountryNumericCode("840"), country_short_name: CountryShortName("Portugal"), cron_str: CronStr("*/5 0 * * *"), payment_card_number: PaymentCardNumber("4111111111111111"), aba_routing_number: ABARoutingNumber("021000021"), mongo_object_id: ObjectId("64f8a4b1e4b0c8a9d4e5f6a7"), language_alpha2: LanguageAlpha2("en"), language_name: LanguageName("English"), iso639_3: ISO639-3("eng"), iso639_5: ISO639-5("gem"), iso_15924: ISO15924("Latn"), s3_path: S3Path("s3://logs/2023/10/25/app.log"), semantic_version: SemanticVersion("1.2.3"), time_zone_name: TimeZoneName("America/New_York"), ulid: Ulid("01F9Z3ZQJZ9QJZ9QJZ9QJZ9QJZ")})"""
    )

    val2 = Extra.model_validate_duper(val_dump)

    assert val == val2
