#![allow(non_snake_case)]
use serde_core::{Deserialize, Deserializer, Serialize, Serializer, de};

/// A helper macro to generate boilerplate for most serializable/deserializable
/// types.
#[macro_export]
macro_rules! duper_serde_module {
    (
        $mod_name:ident,
        $option_mod_name:ident,
        $wrapped_type:ty,
        $type_name:literal
    ) => {
        pub mod $mod_name {
            use super::*;

            pub fn serialize<S>(value: &$wrapped_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_newtype_struct($type_name, value)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<$wrapped_type, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = $wrapped_type;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a newtype struct ", $type_name))
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        <$wrapped_type>::deserialize(deserializer)
                    }
                }

                deserializer.deserialize_newtype_struct($type_name, Visitor)
            }
        }
        pub mod $option_mod_name {
            use super::*;

            pub fn serialize<S>(value: &Option<$wrapped_type>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_newtype_struct($type_name, value)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<$wrapped_type>, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = Option<$wrapped_type>;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a newtype struct ", $type_name))
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        <Option<$wrapped_type>>::deserialize(deserializer)
                    }
                }

                deserializer.deserialize_newtype_struct($type_name, Visitor)
            }
        }
    };
    (
        <$firsttyparam:ident $(, $typaram:ident)*>
        $mod_name:ident,
        $option_mod_name:ident,
        $wrapped_type:path,
        $type_name:literal
    ) => {
        pub mod $mod_name {
            use super::*;

            pub fn serialize<S, $firsttyparam $(, $typaram)*>(value: &$wrapped_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                $wrapped_type: Serialize,
            {
                serializer.serialize_newtype_struct($type_name, value)
            }

            pub fn deserialize<'de, D, $firsttyparam $(, $typaram)*>(deserializer: D) -> Result<$wrapped_type, D::Error>
            where
                D: Deserializer<'de>,
                $wrapped_type: Deserialize<'de>,
            {
                struct Visitor<$firsttyparam $(, $typaram)*> {
                    _marker: ::std::marker::PhantomData<$wrapped_type>,
                }

                impl<'de, $firsttyparam $(, $typaram)*> de::Visitor<'de> for Visitor<$firsttyparam $(, $typaram)*>
                where
                    $wrapped_type: Deserialize<'de>,
                {
                    type Value = $wrapped_type;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a newtype struct ", $type_name))
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        <$wrapped_type>::deserialize(deserializer)
                    }
                }

                deserializer.deserialize_newtype_struct($type_name, Visitor { _marker: ::std::marker::PhantomData })
            }
        }
        pub mod $option_mod_name {
            use super::*;

            pub fn serialize<S, $firsttyparam $(, $typaram)*>(value: &Option<$wrapped_type>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                $wrapped_type: Serialize,
            {
                serializer.serialize_newtype_struct($type_name, value)
            }

            pub fn deserialize<'de, D, $firsttyparam $(, $typaram)*>(deserializer: D) -> Result<Option<$wrapped_type>, D::Error>
            where
                D: Deserializer<'de>,
                $wrapped_type: Deserialize<'de>,
            {
                struct Visitor<$firsttyparam $(, $typaram)*> {
                    _marker: ::std::marker::PhantomData<$wrapped_type>,
                }

                impl<'de, $firsttyparam $(, $typaram)*> de::Visitor<'de> for Visitor<$firsttyparam $(, $typaram)*>
                where
                    $wrapped_type: Deserialize<'de>,
                {
                    type Value = Option<$wrapped_type>;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a newtype struct ", $type_name))
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        <Option<$wrapped_type>>::deserialize(deserializer)
                    }
                }

                deserializer.deserialize_newtype_struct($type_name, Visitor { _marker: ::std::marker::PhantomData })
            }
        }
    };
}

// -- Standard library --

pub mod net {
    use super::*;

    duper_serde_module!(DuperIpAddr, DuperOptionIpAddr, ::std::net::IpAddr, "IpAddr");
    duper_serde_module!(
        DuperIpv4Addr,
        DuperOptionIpv4Addr,
        ::std::net::Ipv4Addr,
        "Ipv4Addr"
    );
    duper_serde_module!(
        DuperIpv6Addr,
        DuperOptionIpv6Addr,
        ::std::net::Ipv6Addr,
        "Ipv6Addr"
    );
    duper_serde_module!(
        DuperSocketAddr,
        DuperOptionSocketAddr,
        ::std::net::SocketAddr,
        "SocketAddr"
    );
    duper_serde_module!(
        DuperSocketAddrV4,
        DuperOptionSocketAddrV4,
        ::std::net::SocketAddrV4,
        "SocketAddrV4"
    );
    duper_serde_module!(
        DuperSocketAddrV6,
        DuperOptionSocketAddrV6,
        ::std::net::SocketAddrV6,
        "SocketAddrV6"
    );
}

pub mod num {
    use super::*;

    duper_serde_module!(
        DuperNonZeroI8,
        DuperOptionNonZeroI8,
        ::std::num::NonZeroI8,
        "NonZeroI8"
    );
    duper_serde_module!(
        DuperNonZeroI16,
        DuperOptionNonZeroI16,
        ::std::num::NonZeroI16,
        "NonZeroI16"
    );
    duper_serde_module!(
        DuperNonZeroI32,
        DuperOptionNonZeroI32,
        ::std::num::NonZeroI32,
        "NonZeroI32"
    );
    duper_serde_module!(
        DuperNonZeroI64,
        DuperOptionNonZeroI64,
        ::std::num::NonZeroI64,
        "NonZeroI64"
    );
    duper_serde_module!(
        DuperNonZeroI128,
        DuperOptionNonZeroI128,
        ::std::num::NonZeroI128,
        "NonZeroI128"
    );
    duper_serde_module!(
        DuperNonZeroIsize,
        DuperOptionNonZeroIsize,
        ::std::num::NonZeroIsize,
        "NonZeroIsize"
    );
    duper_serde_module!(
        DuperNonZeroU8,
        DuperOptionNonZeroU8,
        ::std::num::NonZeroU8,
        "NonZeroU8"
    );
    duper_serde_module!(
        DuperNonZeroU16,
        DuperOptionNonZeroU16,
        ::std::num::NonZeroU16,
        "NonZeroU16"
    );
    duper_serde_module!(
        DuperNonZeroU32,
        DuperOptionNonZeroU32,
        ::std::num::NonZeroU32,
        "NonZeroU32"
    );
    duper_serde_module!(
        DuperNonZeroU64,
        DuperOptionNonZeroU64,
        ::std::num::NonZeroU64,
        "NonZeroU64"
    );
    duper_serde_module!(
        DuperNonZeroU128,
        DuperOptionNonZeroU128,
        ::std::num::NonZeroU128,
        "NonZeroU128"
    );
    duper_serde_module!(
        DuperNonZeroUsize,
        DuperOptionNonZeroUsize,
        ::std::num::NonZeroUsize,
        "NonZeroUsize"
    );

    duper_serde_module!(<T> DuperWrapping, DuperOptionWrapping, ::std::num::Wrapping<T>, "Wrapping");
    duper_serde_module!(<T> DuperSaturating, DuperOptionSaturating, ::std::num::Saturating<T>, "Saturating");
}

pub mod atomic {
    use super::*;

    duper_serde_module!(
        DuperAtomicBool,
        DuperOptionAtomicBool,
        ::std::sync::atomic::AtomicBool,
        "AtomicBool"
    );
    duper_serde_module!(
        DuperAtomicI8,
        DuperOptionAtomicI8,
        ::std::sync::atomic::AtomicI8,
        "AtomicI8"
    );
    duper_serde_module!(
        DuperAtomicI16,
        DuperOptionAtomicI16,
        ::std::sync::atomic::AtomicI16,
        "AtomicI16"
    );
    duper_serde_module!(
        DuperAtomicI32,
        DuperOptionAtomicI32,
        ::std::sync::atomic::AtomicI32,
        "AtomicI32"
    );
    duper_serde_module!(
        DuperAtomicI64,
        DuperOptionAtomicI64,
        ::std::sync::atomic::AtomicI64,
        "AtomicI64"
    );
    duper_serde_module!(
        DuperAtomicIsize,
        DuperOptionAtomicIsize,
        ::std::sync::atomic::AtomicIsize,
        "AtomicIsize"
    );
    duper_serde_module!(
        DuperAtomicU8,
        DuperOptionAtomicU8,
        ::std::sync::atomic::AtomicU8,
        "AtomicU8"
    );
    duper_serde_module!(
        DuperAtomicU16,
        DuperOptionAtomicU16,
        ::std::sync::atomic::AtomicU16,
        "AtomicU16"
    );
    duper_serde_module!(
        DuperAtomicU32,
        DuperOptionAtomicU32,
        ::std::sync::atomic::AtomicU32,
        "AtomicU32"
    );
    duper_serde_module!(
        DuperAtomicU64,
        DuperOptionAtomicU64,
        ::std::sync::atomic::AtomicU64,
        "AtomicU64"
    );
    duper_serde_module!(
        DuperAtomicUsize,
        DuperOptionAtomicUsize,
        ::std::sync::atomic::AtomicUsize,
        "AtomicUsize"
    );
}

#[cfg(not(feature = "temporal"))]
duper_serde_module!(
    DuperDuration,
    DuperOptionDuration,
    ::std::time::Duration,
    "Duration"
);

#[cfg(feature = "temporal")]
pub mod DuperDuration {
    use super::*;
    use ::std::time::Duration as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = temporal_rs::Duration::new(
            0,
            0,
            0,
            0,
            0,
            0,
            i64::try_from(value.as_secs())
                .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?,
            0,
            0,
            value.subsec_nanos().into(),
        )
        .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?;
        let duration = duper::DuperTemporal::try_duration_from(std::borrow::Cow::Owned(
            value
                .as_temporal_string(Default::default())
                .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?,
        ))
        .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?;
        duper::serde::temporal::TemporalString(duration).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::str::FromStr;

        let temporal = duper::serde::temporal::TemporalString::deserialize(deserializer)?;
        match temporal.0 {
            duper::DuperTemporal::Duration(inner) | duper::DuperTemporal::Unspecified(inner) => {
                let duration = temporal_rs::Duration::from_str(inner.into_inner().as_ref())
                    .map_err(|error| {
                        <D::Error as serde_core::de::Error>::custom(error.to_string())
                    })?;
                let mut secs_rounding_options = temporal_rs::options::RoundingOptions::default();
                secs_rounding_options.largest_unit = Some(temporal_rs::options::Unit::Second);
                secs_rounding_options.smallest_unit = Some(temporal_rs::options::Unit::Second);
                let secs = duration
                    .round(secs_rounding_options, None)
                    .map_err(|error| {
                        <D::Error as serde_core::de::Error>::custom(error.to_string())
                    })?;
                let nanosecs = duration.subtract(&secs).map_err(|error| {
                    <D::Error as serde_core::de::Error>::custom(error.to_string())
                })?;
                let mut nanosecs_rounding_options =
                    temporal_rs::options::RoundingOptions::default();
                nanosecs_rounding_options.largest_unit =
                    Some(temporal_rs::options::Unit::Nanosecond);
                let nanosecs =
                    nanosecs
                        .round(nanosecs_rounding_options, None)
                        .map_err(|error| {
                            <D::Error as serde_core::de::Error>::custom(error.to_string())
                        })?;
                Ok(WrappedType::new(
                    secs.seconds()
                        .try_into()
                        .map_err(|error: std::num::TryFromIntError| {
                            <D::Error as serde_core::de::Error>::custom(error.to_string())
                        })?,
                    nanosecs.nanoseconds().try_into().map_err(
                        |error: std::num::TryFromIntError| {
                            <D::Error as serde_core::de::Error>::custom(error.to_string())
                        },
                    )?,
                ))
            }
            _ => Err(<D::Error as serde_core::de::Error>::custom(format!(
                "expected Temporal Duration, found {:?}",
                temporal.0
            ))),
        }
    }
}
#[cfg(feature = "temporal")]
pub mod DuperOptionDuration {
    use super::*;
    use ::std::time::Duration as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => {
                let value = temporal_rs::Duration::new(
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    i64::try_from(value.as_secs())
                        .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?,
                    0,
                    0,
                    value.subsec_nanos().into(),
                )
                .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?;
                let duration = duper::DuperTemporal::try_duration_from(std::borrow::Cow::Owned(
                    value
                        .as_temporal_string(Default::default())
                        .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?,
                ))
                .map_err(|error| <S::Error as serde_core::ser::Error>::custom(error))?;
                duper::serde::temporal::TemporalString(duration).serialize(serializer)
            }
            None => serializer.serialize_newtype_struct("Duration", &Option::<()>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a Duration")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                use std::str::FromStr;

                let temporal = duper::serde::temporal::TemporalString::deserialize(deserializer)?;
                match temporal.0 {
                    duper::DuperTemporal::Duration(inner)
                    | duper::DuperTemporal::Unspecified(inner) => {
                        let duration = temporal_rs::Duration::from_str(inner.into_inner().as_ref())
                            .map_err(|error| {
                                <D::Error as serde_core::de::Error>::custom(error.to_string())
                            })?;
                        let mut secs_rounding_options =
                            temporal_rs::options::RoundingOptions::default();
                        secs_rounding_options.largest_unit =
                            Some(temporal_rs::options::Unit::Second);
                        secs_rounding_options.smallest_unit =
                            Some(temporal_rs::options::Unit::Second);
                        let secs =
                            duration
                                .round(secs_rounding_options, None)
                                .map_err(|error| {
                                    <D::Error as serde_core::de::Error>::custom(error.to_string())
                                })?;
                        let nanosecs = duration.subtract(&secs).map_err(|error| {
                            <D::Error as serde_core::de::Error>::custom(error.to_string())
                        })?;
                        let mut nanosecs_rounding_options =
                            temporal_rs::options::RoundingOptions::default();
                        nanosecs_rounding_options.largest_unit =
                            Some(temporal_rs::options::Unit::Nanosecond);
                        let nanosecs =
                            nanosecs
                                .round(nanosecs_rounding_options, None)
                                .map_err(|error| {
                                    <D::Error as serde_core::de::Error>::custom(error.to_string())
                                })?;
                        Ok(Some(WrappedType::new(
                            secs.seconds().try_into().map_err(
                                |error: std::num::TryFromIntError| {
                                    <D::Error as serde_core::de::Error>::custom(error.to_string())
                                },
                            )?,
                            nanosecs.nanoseconds().try_into().map_err(
                                |error: std::num::TryFromIntError| {
                                    <D::Error as serde_core::de::Error>::custom(error.to_string())
                                },
                            )?,
                        )))
                    }
                    _ => Err(<D::Error as serde_core::de::Error>::custom(format!(
                        "expected Temporal Duration, found {:?}",
                        temporal.0
                    ))),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor)
    }
}

duper_serde_module!(
    DuperSystemTime,
    DuperOptionSystemTime,
    ::std::time::SystemTime,
    "SystemTime"
);

duper_serde_module!(
    DuperPathBuf,
    DuperOptionPathBuf,
    ::std::path::PathBuf,
    "PathBuf"
);
pub mod DuperPath {
    use super::*;

    pub fn serialize<S>(value: &::std::path::Path, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Path", value)
    }
}
pub mod DuperOptionPath {
    use super::*;

    pub fn serialize<S>(
        value: &Option<&::std::path::Path>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Path", &value)
    }
}

pub mod ffi;

duper_serde_module!(<T> DuperVec, DuperOptionVec, ::std::vec::Vec<T>, "Vec");
duper_serde_module!(<T> DuperReverse, DuperOptionReverse, ::std::cmp::Reverse<T>, "Reverse");

pub mod collections {
    pub use super::*;

    duper_serde_module!(
        <T>
        DuperBinaryHeap,
        DuperOptionBinaryHeap,
        ::std::collections::BinaryHeap<T>,
        "BinaryHeap"
    );
    duper_serde_module!(
        <T>
        DuperBTreeSet,
        DuperOptionBTreeSet,
        ::std::collections::BTreeSet<T>,
        "BTreeSet"
    );
    duper_serde_module!(<T> DuperHashSet, DuperOptionHashSet, ::std::collections::HashSet<T>, "HashSet");
    duper_serde_module!(
        <T>
        DuperLinkedList,
        DuperOptionLinkedList,
        ::std::collections::LinkedList<T>,
        "LinkedList"
    );
    duper_serde_module!(
        <T>
        DuperVecDeque,
        DuperOptionVecDeque,
        ::std::collections::VecDeque<T>,
        "VecDeque"
    );
    duper_serde_module!(
        <K, V>
        DuperBTreeMap,
        DuperOptionBTreeMap,
        ::std::collections::BTreeMap<K, V>,
        "BTreeMap"
    );
    duper_serde_module!(
        <K, V>
        DuperHashMap,
        DuperOptionHashMap,
        ::std::collections::HashMap<K, V>,
        "HashMap"
    );
}

// External crates

#[cfg(feature = "bytes")]
duper_serde_module!(DuperBytes, DuperOptionBytes, ::bytes::Bytes, "Bytes");

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "decimal")]
pub mod DuperDecimal {
    use super::*;
    use ::rust_decimal::Decimal as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Decimal", value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = WrappedType;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a newtype struct Decimal")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                ::rust_decimal::serde::str::deserialize(deserializer)
            }
        }

        deserializer.deserialize_newtype_struct("Decimal", Visitor)
    }
}
#[cfg(feature = "decimal")]
pub mod DuperOptionDecimal {
    use super::*;
    use ::rust_decimal::Decimal as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Decimal", value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a newtype struct Decimal")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                ::rust_decimal::serde::str_option::deserialize(deserializer)
            }
        }

        deserializer.deserialize_newtype_struct("Decimal", Visitor)
    }
}

#[cfg(feature = "ipnet")]
duper_serde_module!(DuperIpNet, DuperOptionIpNet, ::ipnet::IpNet, "IpNet");
#[cfg(feature = "ipnet")]
duper_serde_module!(
    DuperIpv4Net,
    DuperOptionIpv4Net,
    ::ipnet::Ipv4Net,
    "Ipv4Net"
);
#[cfg(feature = "ipnet")]
duper_serde_module!(
    DuperIpv6Net,
    DuperOptionIpv6Net,
    ::ipnet::Ipv6Net,
    "Ipv6Net"
);

#[cfg(feature = "regex")]
pub mod DuperRegex {
    use super::*;
    use ::regex::Regex as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Regex", value.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = WrappedType;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a Regex")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                WrappedType::new(v).map_err(|error| E::custom(error))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(self)
            }
        }

        deserializer.deserialize_newtype_struct("Regex", Visitor)
    }
}
#[cfg(feature = "regex")]
pub mod DuperOptionRegex {
    use super::*;
    use ::regex::Regex as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_newtype_struct("Regex", value.as_str()),
            None => serializer.serialize_newtype_struct("Regex", &Option::<()>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a Regex")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Some(WrappedType::new(v).map_err(|error| E::custom(error))).transpose()
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(self)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_option(self)
            }
        }

        deserializer.deserialize_newtype_struct("Regex", Visitor)
    }
}
#[cfg(feature = "regex")]
pub mod DuperBytesRegex {
    use super::*;
    use ::regex::bytes::Regex as WrappedType;

    pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("BytesRegex", value.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = WrappedType;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a BytesRegex")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                WrappedType::new(v).map_err(|error| E::custom(error))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(self)
            }
        }

        deserializer.deserialize_newtype_struct("BytesRegex", Visitor)
    }
}
#[cfg(feature = "regex")]
pub mod DuperOptionBytesRegex {
    use super::*;
    use ::regex::bytes::Regex as WrappedType;

    pub fn serialize<S>(value: &Option<WrappedType>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_newtype_struct("BytesRegex", value.as_str()),
            None => serializer.serialize_newtype_struct("BytesRegex", &Option::<()>::None),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<WrappedType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<WrappedType>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a BytesRegex")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Some(WrappedType::new(v).map_err(|error| E::custom(error))).transpose()
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(self)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_option(self)
            }
        }

        deserializer.deserialize_newtype_struct("BytesRegex", Visitor)
    }
}

#[cfg(feature = "uuid")]
duper_serde_module!(DuperUuid, DuperOptionUuid, ::uuid::Uuid, "Uuid");
