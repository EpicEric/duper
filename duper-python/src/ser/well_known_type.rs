use std::borrow::Cow;

use duper::{
    DuperArray, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString, DuperTemporal,
    DuperValue,
};
use pyo3::{exceptions::PyValueError, prelude::*, types::*};

use crate::{
    Duper,
    ser::{serialize_pyany, serialize_pyclass_identifier},
    temporal::TemporalString,
};

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum WellKnownType<'py> {
    // bson
    BsonObjectId(Bound<'py, PyAny>),
    // collections
    Deque(Bound<'py, PyAny>),
    // dataclasses
    Dataclass(Bound<'py, PyAny>),
    // datetime
    DateTime(Bound<'py, PyAny>),
    TimeDelta(Bound<'py, PyAny>),
    Date(Bound<'py, PyAny>),
    Time(Bound<'py, PyAny>),
    // decimal
    Decimal(Bound<'py, PyAny>),
    // enum
    Enum(Bound<'py, PyAny>),
    // ipaddress
    IPv4Address(Bound<'py, PyAny>),
    IPv4Interface(Bound<'py, PyAny>),
    IPv4Network(Bound<'py, PyAny>),
    IPv6Address(Bound<'py, PyAny>),
    IPv6Interface(Bound<'py, PyAny>),
    IPv6Network(Bound<'py, PyAny>),
    // pathlib
    Path(Bound<'py, PyAny>),
    PosixPath(Bound<'py, PyAny>),
    WindowsPath(Bound<'py, PyAny>),
    PurePath(Bound<'py, PyAny>),
    PurePosixPath(Bound<'py, PyAny>),
    PureWindowsPath(Bound<'py, PyAny>),
    // pydantic
    BaseModel(Bound<'py, PyAny>),
    ByteSize(Bound<'py, PyAny>),
    // pydantic network types
    AnyUrl(Bound<'py, PyAny>),
    AnyHttpUrl(Bound<'py, PyAny>),
    HttpUrl(Bound<'py, PyAny>),
    AnyWebsocketUrl(Bound<'py, PyAny>),
    WebsocketUrl(Bound<'py, PyAny>),
    FileUrl(Bound<'py, PyAny>),
    FtpUrl(Bound<'py, PyAny>),
    PostgresDsn(Bound<'py, PyAny>),
    CockroachDsn(Bound<'py, PyAny>),
    AmqpDsn(Bound<'py, PyAny>),
    RedisDsn(Bound<'py, PyAny>),
    MongoDsn(Bound<'py, PyAny>),
    KafkaDsn(Bound<'py, PyAny>),
    NatsDsn(Bound<'py, PyAny>),
    MySQLDsn(Bound<'py, PyAny>),
    MariaDBDsn(Bound<'py, PyAny>),
    ClickHouseDsn(Bound<'py, PyAny>),
    SnowflakeDsn(Bound<'py, PyAny>),
    EmailStr(Bound<'py, PyAny>),
    NameEmail(Bound<'py, PyAny>),
    IPvAnyAddress(Bound<'py, PyAny>),
    IPvAnyInterface(Bound<'py, PyAny>),
    IPvAnyNetwork(Bound<'py, PyAny>),
    // pydantic extra types
    Color(Bound<'py, PyAny>),
    CountryAlpha2(Bound<'py, PyAny>),
    CountryAlpha3(Bound<'py, PyAny>),
    CountryNumericCode(Bound<'py, PyAny>),
    CountryShortName(Bound<'py, PyAny>),
    CronStr(Bound<'py, PyAny>),
    PaymentCardNumber(Bound<'py, PyAny>),
    AbaRoutingNumber(Bound<'py, PyAny>),
    LanguageAlpha2(Bound<'py, PyAny>),
    LanguageName(Bound<'py, PyAny>),
    Iso639_3(Bound<'py, PyAny>),
    Iso639_5(Bound<'py, PyAny>),
    Iso15924(Bound<'py, PyAny>),
    S3Path(Bound<'py, PyAny>),
    SemanticVersion(Bound<'py, PyAny>),
    TimeZoneName(Bound<'py, PyAny>),
    Ulid(Bound<'py, PyAny>),
    // re
    Pattern(Bound<'py, PyAny>),
    // temporal string
    TemporalStringInstant(duper::DuperTemporal<'static>),
    TemporalStringZonedDateTime(duper::DuperTemporal<'static>),
    TemporalStringPlainDate(duper::DuperTemporal<'static>),
    TemporalStringPlainTime(duper::DuperTemporal<'static>),
    TemporalStringPlainDateTime(duper::DuperTemporal<'static>),
    TemporalStringPlainYearMonth(duper::DuperTemporal<'static>),
    TemporalStringPlainMonthDay(duper::DuperTemporal<'static>),
    TemporalStringDuration(duper::DuperTemporal<'static>),
    TemporalString(duper::DuperTemporal<'static>),
    // uuid
    Uuid(Bound<'py, PyAny>),
}

impl<'py> WellKnownType<'py> {
    pub(crate) fn identify(value: &Bound<'py, PyAny>) -> PyResult<Option<Self>> {
        if !value.hasattr("__class__")? {
            return Ok(None);
        }
        let inspect: Bound<'py, PyModule> = value.py().import("inspect")?;
        let mro = inspect
            .getattr("getmro")?
            .call1((value.getattr("__class__")?,))?;
        for class in mro.try_iter()? {
            let Ok(class) = class else {
                continue;
            };
            let module_attr = class.getattr("__module__")?;
            let module: &str = module_attr.extract()?;
            let classname_attr = class.getattr("__name__")?;
            let classname: &str = classname_attr.extract()?;
            match (module, classname) {
                // bson
                ("bson.objectid", "ObjectId") => {
                    return Ok(Some(WellKnownType::BsonObjectId(value.clone())));
                }
                // collections
                ("collections", "deque") => return Ok(Some(WellKnownType::Deque(value.clone()))),
                // datetime
                ("datetime", "datetime") => {
                    return Ok(Some(WellKnownType::DateTime(value.clone())));
                }
                ("datetime", "timedelta") => {
                    return Ok(Some(WellKnownType::TimeDelta(value.clone())));
                }
                ("datetime", "date") => return Ok(Some(WellKnownType::Date(value.clone()))),
                ("datetime", "time") => return Ok(Some(WellKnownType::Time(value.clone()))),
                // decimal
                ("decimal", "Decimal") => return Ok(Some(WellKnownType::Decimal(value.clone()))),
                // enum
                ("enum", "Enum") => {
                    return Ok(Some(WellKnownType::Enum(value.clone())));
                }
                // ipaddress
                ("ipaddress", "IPv4Address") => {
                    return Ok(Some(WellKnownType::IPv4Address(value.clone())));
                }
                ("ipaddress", "IPv4Interface") => {
                    return Ok(Some(WellKnownType::IPv4Interface(value.clone())));
                }
                ("ipaddress", "IPv4Network") => {
                    return Ok(Some(WellKnownType::IPv4Network(value.clone())));
                }
                ("ipaddress", "IPv6Address") => {
                    return Ok(Some(WellKnownType::IPv6Address(value.clone())));
                }
                ("ipaddress", "IPv6Interface") => {
                    return Ok(Some(WellKnownType::IPv6Interface(value.clone())));
                }
                ("ipaddress", "IPv6Network") => {
                    return Ok(Some(WellKnownType::IPv6Network(value.clone())));
                }
                // pathlib
                ("pathlib", "Path") => return Ok(Some(WellKnownType::Path(value.clone()))),
                ("pathlib", "PosixPath") => {
                    return Ok(Some(WellKnownType::PosixPath(value.clone())));
                }
                ("pathlib", "WindowsPath") => {
                    return Ok(Some(WellKnownType::WindowsPath(value.clone())));
                }
                ("pathlib", "PurePath") => return Ok(Some(WellKnownType::PurePath(value.clone()))),
                ("pathlib", "PurePosixPath") => {
                    return Ok(Some(WellKnownType::PurePosixPath(value.clone())));
                }
                ("pathlib", "PureWindowsPath") => {
                    return Ok(Some(WellKnownType::PureWindowsPath(value.clone())));
                }
                // pydantic
                ("pydantic.main", "BaseModel") => {
                    return Ok(Some(WellKnownType::BaseModel(value.clone())));
                }
                ("pydantic", "ByteSize") => {
                    return Ok(Some(WellKnownType::ByteSize(value.clone())));
                }
                // pydantic network types
                ("pydantic.networks", "AnyUrl") => {
                    return Ok(Some(WellKnownType::AnyUrl(value.clone())));
                }
                ("pydantic.networks", "AnyHttpUrl") => {
                    return Ok(Some(WellKnownType::AnyHttpUrl(value.clone())));
                }
                ("pydantic.networks", "HttpUrl") => {
                    return Ok(Some(WellKnownType::HttpUrl(value.clone())));
                }
                ("pydantic.networks", "AnyWebsocketUrl") => {
                    return Ok(Some(WellKnownType::AnyWebsocketUrl(value.clone())));
                }
                ("pydantic.networks", "WebsocketUrl") => {
                    return Ok(Some(WellKnownType::WebsocketUrl(value.clone())));
                }
                ("pydantic.networks", "FileUrl") => {
                    return Ok(Some(WellKnownType::FileUrl(value.clone())));
                }
                ("pydantic.networks", "FtpUrl") => {
                    return Ok(Some(WellKnownType::FtpUrl(value.clone())));
                }
                ("pydantic.networks", "PostgresDsn") => {
                    return Ok(Some(WellKnownType::PostgresDsn(value.clone())));
                }
                ("pydantic.networks", "CockroachDsn") => {
                    return Ok(Some(WellKnownType::CockroachDsn(value.clone())));
                }
                ("pydantic.networks", "AmqpDsn") => {
                    return Ok(Some(WellKnownType::AmqpDsn(value.clone())));
                }
                ("pydantic.networks", "RedisDsn") => {
                    return Ok(Some(WellKnownType::RedisDsn(value.clone())));
                }
                ("pydantic.networks", "MongoDsn") => {
                    return Ok(Some(WellKnownType::MongoDsn(value.clone())));
                }
                ("pydantic.networks", "KafkaDsn") => {
                    return Ok(Some(WellKnownType::KafkaDsn(value.clone())));
                }
                ("pydantic.networks", "NatsDsn") => {
                    return Ok(Some(WellKnownType::NatsDsn(value.clone())));
                }
                ("pydantic.networks", "MySQLDsn") => {
                    return Ok(Some(WellKnownType::MySQLDsn(value.clone())));
                }
                ("pydantic.networks", "MariaDBDsn") => {
                    return Ok(Some(WellKnownType::MariaDBDsn(value.clone())));
                }
                ("pydantic.networks", "ClickHouseDsn") => {
                    return Ok(Some(WellKnownType::ClickHouseDsn(value.clone())));
                }
                ("pydantic.networks", "SnowflakeDsn") => {
                    return Ok(Some(WellKnownType::SnowflakeDsn(value.clone())));
                }
                ("pydantic.networks", "EmailStr") => {
                    return Ok(Some(WellKnownType::EmailStr(value.clone())));
                }
                ("pydantic.networks", "NameEmail") => {
                    return Ok(Some(WellKnownType::NameEmail(value.clone())));
                }
                ("pydantic.networks", "IPvAnyAddress") => {
                    return Ok(Some(WellKnownType::IPvAnyAddress(value.clone())));
                }
                ("pydantic.networks", "IPvAnyInterface") => {
                    return Ok(Some(WellKnownType::IPvAnyInterface(value.clone())));
                }
                ("pydantic.networks", "IPvAnyNetwork") => {
                    return Ok(Some(WellKnownType::IPvAnyNetwork(value.clone())));
                }
                // pydantic extra types
                ("pydantic_extra_types.color", "Color") => {
                    return Ok(Some(WellKnownType::Color(value.clone())));
                }
                ("pydantic_extra_types.country", "CountryAlpha2") => {
                    return Ok(Some(WellKnownType::CountryAlpha2(value.clone())));
                }
                ("pydantic_extra_types.country", "CountryAlpha3") => {
                    return Ok(Some(WellKnownType::CountryAlpha3(value.clone())));
                }
                ("pydantic_extra_types.country", "CountryNumericCode") => {
                    return Ok(Some(WellKnownType::CountryNumericCode(value.clone())));
                }
                ("pydantic_extra_types.country", "CountryShortName") => {
                    return Ok(Some(WellKnownType::CountryShortName(value.clone())));
                }
                ("pydantic_extra_types.cron", "CronStr") => {
                    return Ok(Some(WellKnownType::CronStr(value.clone())));
                }
                ("pydantic_extra_types.payment", "PaymentCardNumber") => {
                    return Ok(Some(WellKnownType::PaymentCardNumber(value.clone())));
                }
                ("pydantic_extra_types.routing_number", "ABARoutingNumber") => {
                    return Ok(Some(WellKnownType::AbaRoutingNumber(value.clone())));
                }
                ("pydantic_extra_types.language_code", "LanguageAlpha2") => {
                    return Ok(Some(WellKnownType::LanguageAlpha2(value.clone())));
                }
                ("pydantic_extra_types.language_code", "LanguageName") => {
                    return Ok(Some(WellKnownType::LanguageName(value.clone())));
                }
                ("pydantic_extra_types.language_code", "ISO639_3") => {
                    return Ok(Some(WellKnownType::Iso639_3(value.clone())));
                }
                ("pydantic_extra_types.language_code", "ISO639_5") => {
                    return Ok(Some(WellKnownType::Iso639_5(value.clone())));
                }
                ("pydantic_extra_types.script_code", "ISO_15924") => {
                    return Ok(Some(WellKnownType::Iso15924(value.clone())));
                }
                ("pydantic_extra_types.s3", "S3Path") => {
                    return Ok(Some(WellKnownType::S3Path(value.clone())));
                }
                ("pydantic_extra_types.semantic_version", "SemanticVersion") => {
                    return Ok(Some(WellKnownType::SemanticVersion(value.clone())));
                }
                ("pydantic_extra_types.timezone_name", "TimeZoneName") => {
                    return Ok(Some(WellKnownType::TimeZoneName(value.clone())));
                }
                ("ulid", "ULID") => {
                    return Ok(Some(WellKnownType::Ulid(value.clone())));
                }
                // re
                ("re", "Pattern") => return Ok(Some(WellKnownType::Pattern(value.clone()))),
                // uuid
                ("uuid", "UUID") => return Ok(Some(WellKnownType::Uuid(value.clone()))),
                _ => (),
            }
        }
        let dataclasses: Bound<'py, PyModule> = value.py().import("dataclasses")?;
        if dataclasses
            .getattr("is_dataclass")?
            .call1((value,))?
            .extract()?
        {
            return Ok(Some(WellKnownType::Dataclass(value.clone())));
        }
        // temporal string
        if let Ok(value) = value.cast::<TemporalString>() {
            let temporal = &value.get().temporal;
            match temporal {
                DuperTemporal::Instant(_) => {
                    return Ok(Some(WellKnownType::TemporalStringInstant(temporal.clone())));
                }
                DuperTemporal::ZonedDateTime(_) => {
                    return Ok(Some(WellKnownType::TemporalStringZonedDateTime(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::PlainDate(_) => {
                    return Ok(Some(WellKnownType::TemporalStringPlainDate(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::PlainTime(_) => {
                    return Ok(Some(WellKnownType::TemporalStringPlainTime(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::PlainDateTime(_) => {
                    return Ok(Some(WellKnownType::TemporalStringPlainDateTime(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::PlainYearMonth(_) => {
                    return Ok(Some(WellKnownType::TemporalStringPlainYearMonth(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::PlainMonthDay(_) => {
                    return Ok(Some(WellKnownType::TemporalStringPlainMonthDay(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::Duration(_) => {
                    return Ok(Some(WellKnownType::TemporalStringDuration(
                        temporal.clone(),
                    )));
                }
                DuperTemporal::Unspecified(_) => {
                    return Ok(Some(WellKnownType::TemporalString(temporal.clone())));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn serialize(self) -> PyResult<DuperValue<'py>> {
        match self {
            // bson
            WellKnownType::BsonObjectId(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ObjectId")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // collections
            WellKnownType::Deque(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Deque")).expect("valid identifier"),
                ),
                inner: DuperInner::Array(DuperArray::from(
                    value
                        .try_iter()?
                        .map(|elem| -> PyResult<DuperValue<'_>> { serialize_pyany(elem?) })
                        .collect::<PyResult<Vec<_>>>()?,
                )),
            }),
            // dataclasses
            WellKnownType::Dataclass(value) => Ok(DuperValue {
                identifier: serialize_pyclass_identifier(&value)?,
                inner: serialize_pyany(value.getattr("__dict__")?)?.inner,
            }),
            // datetime
            WellKnownType::DateTime(value) => Ok({
                let datetime: Cow<'_, str> =
                    Cow::Owned(value.call_method0("isoformat")?.extract()?);
                let tzinfo = value.getattr("tzinfo")?;
                if tzinfo.is_none() || tzinfo.getattr("utcoffset")?.call1((value,))?.is_none() {
                    DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("PlainDateTime"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(
                            DuperTemporal::try_plain_date_time_from(datetime).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Failed to parse Temporal value: {err}"
                                ))
                            })?,
                        ),
                    }
                } else {
                    DuperValue {
                        identifier: Some(
                            DuperIdentifier::try_from(Cow::Borrowed("Instant"))
                                .expect("valid identifier"),
                        ),
                        inner: DuperInner::Temporal(
                            DuperTemporal::try_instant_from(datetime).map_err(|err| {
                                PyValueError::new_err(format!(
                                    "Failed to parse Temporal value: {err}"
                                ))
                            })?,
                        ),
                    }
                }
            }),
            WellKnownType::TimeDelta(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Duration")).expect("valid identifier"),
                ),
                inner: {
                    let py = value.py();
                    let datetime_module = py.import("datetime")?;
                    let pydantic_module = py.import("pydantic")?;
                    let adapter = pydantic_module
                        .getattr("TypeAdapter")?
                        .call1((datetime_module.getattr("timedelta")?,))?;
                    let kwargs = PyDict::new(py);
                    kwargs.set_item("mode", "json")?;
                    let dump = adapter
                        .getattr("dump_python")?
                        .call((value,), Some(&kwargs))?;
                    DuperInner::Temporal(
                        DuperTemporal::try_duration_from(Cow::Owned(dump.extract()?)).map_err(
                            |err| {
                                PyValueError::new_err(format!(
                                    "Failed to parse Temporal value: {err}"
                                ))
                            },
                        )?,
                    )
                },
            }),
            WellKnownType::Date(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PlainDate"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(
                    DuperTemporal::try_plain_date_from(Cow::Owned(
                        value.call_method0("isoformat")?.extract()?,
                    ))
                    .map_err(|err| {
                        PyValueError::new_err(format!("Failed to parse Temporal value: {err}"))
                    })?,
                ),
            }),
            WellKnownType::Time(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PlainTime"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(
                    DuperTemporal::try_plain_time_from(Cow::Owned(
                        value.call_method0("isoformat")?.extract()?,
                    ))
                    .map_err(|err| {
                        PyValueError::new_err(format!("Failed to parse Temporal value: {err}"))
                    })?,
                ),
            }),
            // decimal
            WellKnownType::Decimal(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Decimal")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // enum
            WellKnownType::Enum(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Address"))
                        .expect("valid identifier"),
                ),
                inner: serialize_pyany(value.getattr("value")?)?.inner,
            }),
            // ipaddress
            WellKnownType::IPv4Address(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Address"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv4Interface(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Interface"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv4Network(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv4Network"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Address(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Address"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Interface(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Interface"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPv6Network(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPv6Network"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // pathlib
            WellKnownType::Path(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Path")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PosixPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PosixPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::WindowsPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("WindowsPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PurePath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PurePath")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PurePosixPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PurePosixPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PureWindowsPath(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PureWindowsPath"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // pydantic
            WellKnownType::BaseModel(value) => serialize_pydantic_model(value),
            WellKnownType::ByteSize(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ByteSize")).expect("valid identifier"),
                ),
                inner: serialize_pyany(value.call_method0("__int__")?)?.inner,
            }),
            // pydantic network types
            WellKnownType::AnyUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("AnyUrl")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::AnyHttpUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("AnyHttpUrl"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::HttpUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("HttpUrl")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::AnyWebsocketUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("AnyWebsocketUrl"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::WebsocketUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("WebsocketUrl"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::FileUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("FileUrl")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::FtpUrl(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("FtpUrl")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PostgresDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PostgresDsn"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CockroachDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CockroachDsn"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::AmqpDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("AmqpDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::RedisDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("RedisDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::MongoDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("MongoDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::KafkaDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("KafkaDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::NatsDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("NatsDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::MySQLDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("MySQLDsn")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::MariaDBDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("MariaDBDsn"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::ClickHouseDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ClickHouseDsn"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::SnowflakeDsn(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("SnowflakeDsn"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::EmailStr(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("EmailStr")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::NameEmail(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("NameEmail"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPvAnyAddress(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPvAnyAddress"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPvAnyInterface(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPvAnyInterface"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::IPvAnyNetwork(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("IPvAnyNetwork"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // pydantic extra types
            WellKnownType::Color(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Color")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CountryAlpha2(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CountryAlpha2"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CountryAlpha3(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CountryAlpha3"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CountryNumericCode(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CountryNumericCode"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CountryShortName(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CountryShortName"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::CronStr(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("CronStr")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::PaymentCardNumber(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("PaymentCardNumber"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::AbaRoutingNumber(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ABARoutingNumber"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::LanguageAlpha2(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("LanguageAlpha2"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::LanguageName(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("LanguageName"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::Iso639_3(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ISO639-3")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::Iso639_5(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ISO639-5")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::Iso15924(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("ISO15924")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::S3Path(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("S3Path")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::SemanticVersion(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("SemanticVersion"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::TimeZoneName(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("TimeZoneName"))
                        .expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            WellKnownType::Ulid(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Ulid")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
            // re
            WellKnownType::Pattern(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Pattern")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(
                    value.getattr("pattern")?.extract()?,
                ))),
            }),
            // temporal string
            WellKnownType::TemporalStringInstant(value) => Ok(DuperValue {
                identifier: Some(DuperIdentifier::try_from("Instant").expect("valid identifier")),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringZonedDateTime(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from("ZonedDateTime").expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringPlainDate(value) => Ok(DuperValue {
                identifier: Some(DuperIdentifier::try_from("PlainDate").expect("valid identifier")),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringPlainTime(value) => Ok(DuperValue {
                identifier: Some(DuperIdentifier::try_from("PlainTime").expect("valid identifier")),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringPlainDateTime(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from("PlainDateTime").expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringPlainYearMonth(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from("PlainYearMonth").expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringPlainMonthDay(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from("PlainMonthDay").expect("valid identifier"),
                ),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalStringDuration(value) => Ok(DuperValue {
                identifier: Some(DuperIdentifier::try_from("Duration").expect("valid identifier")),
                inner: DuperInner::Temporal(value),
            }),
            WellKnownType::TemporalString(value) => Ok(DuperValue {
                identifier: None,
                inner: DuperInner::Temporal(value),
            }),
            // uuid
            WellKnownType::Uuid(value) => Ok(DuperValue {
                identifier: Some(
                    DuperIdentifier::try_from(Cow::Borrowed("Uuid")).expect("valid identifier"),
                ),
                inner: DuperInner::String(DuperString::from(Cow::Owned(value.str()?.extract()?))),
            }),
        }
    }
}

fn serialize_pydantic_model<'py>(obj: Bound<'py, PyAny>) -> PyResult<DuperValue<'py>> {
    if let Ok(class) = obj.getattr("__class__")
        && let model_fields = class.getattr("model_fields")?
        && model_fields.is_instance_of::<PyDict>()
    {
        let field_dict = model_fields.cast::<PyDict>()?;
        let fields: PyResult<Vec<_>> = field_dict
            .iter()
            .map(|(field_name, field_info)| {
                let field_name: &Bound<'py, PyString> = field_name.cast()?;
                let value = obj.getattr(field_name)?;
                let duper_value = serialize_pyany(value)?;
                let duper_metadata = field_info
                    .getattr("metadata")?
                    .try_iter()?
                    .find(|metadata| match metadata {
                        Ok(metadata) => metadata.is_instance_of::<Duper>(),
                        Err(_) => false,
                    })
                    .transpose()?;
                let identifier = duper_metadata.map_or(duper_value.identifier, |duper| {
                    duper
                        .cast::<Duper>()
                        .expect("checked Duper instance")
                        .get()
                        .identifier
                        .clone()
                });
                Ok((
                    DuperKey::from(Cow::Owned(field_name.to_string())),
                    DuperValue {
                        identifier,
                        inner: duper_value.inner,
                    },
                ))
            })
            .collect();
        if obj.hasattr("model_config")?
            && let Ok(attr) = obj.getattr("model_config")
            && let Ok(model_config) = attr.cast::<PyDict>()
            && model_config.contains("title")?
        {
            if let Some(title) = model_config.get_item("title")?
                && let Ok(Some(title)) = title.extract::<Option<&str>>()
            {
                Ok(DuperValue {
                    identifier: Some(
                        DuperIdentifier::try_from_lossy(Cow::Owned(title.to_string())).map_err(
                            |error| {
                                PyErr::new::<PyValueError, String>(format!(
                                    "Invalid identifier: {title} ({error})"
                                ))
                            },
                        )?,
                    ),
                    inner: DuperInner::Object(
                        DuperObject::try_from(fields?)
                            .expect("no duplicate keys in Pydantic model"),
                    ),
                })
            } else {
                Ok(DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(
                        DuperObject::try_from(fields?)
                            .expect("no duplicate keys in Pydantic model"),
                    ),
                })
            }
        } else {
            Ok(DuperValue {
                identifier: serialize_pyclass_identifier(&obj)?,
                inner: DuperInner::Object(
                    DuperObject::try_from(fields?).expect("no duplicate keys in Pydantic model"),
                ),
            })
        }
    } else {
        Err(PyErr::new::<PyValueError, String>(format!(
            "unsupported type: {}",
            obj.get_type()
        )))
    }
}
