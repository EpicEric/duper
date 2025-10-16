#![allow(non_snake_case)]
use serde_core::{Deserialize, Deserializer, Serialize, Serializer};

#[macro_export]
macro_rules! duper_serde_module {
    (
        $mod_name:ident,
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
                <$wrapped_type>::deserialize(deserializer)
            }
        }
    };
    (
        $mod_name:ident <$firsttyparam:ident $(, $typaram:ident)*>,
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
                <$wrapped_type>::deserialize(deserializer)
            }
        }
    };
}

// Stdlib

pub mod net {
    use super::*;

    duper_serde_module!(DuperIpAddr, ::std::net::IpAddr, "IpAddr");
    duper_serde_module!(DuperIpv4Addr, ::std::net::Ipv4Addr, "Ipv4Addr");
    duper_serde_module!(DuperIpv6Addr, ::std::net::Ipv6Addr, "Ipv6Addr");
    duper_serde_module!(DuperSocketAddr, ::std::net::SocketAddr, "SocketAddr");
    duper_serde_module!(DuperSocketAddrV4, ::std::net::SocketAddrV4, "SocketAddrV4");
    duper_serde_module!(DuperSocketAddrV6, ::std::net::SocketAddrV6, "SocketAddrV6");
}

pub mod num {
    use super::*;

    duper_serde_module!(DuperNonZeroI8, ::std::num::NonZeroI8, "NonZeroI8");
    duper_serde_module!(DuperNonZeroI16, ::std::num::NonZeroI16, "NonZeroI16");
    duper_serde_module!(DuperNonZeroI32, ::std::num::NonZeroI32, "NonZeroI32");
    duper_serde_module!(DuperNonZeroI64, ::std::num::NonZeroI64, "NonZeroI64");
    duper_serde_module!(DuperNonZeroI128, ::std::num::NonZeroI128, "NonZeroI128");
    duper_serde_module!(DuperNonZeroIsize, ::std::num::NonZeroIsize, "NonZeroIsize");
    duper_serde_module!(DuperNonZeroU8, ::std::num::NonZeroU8, "NonZeroU8");
    duper_serde_module!(DuperNonZeroU16, ::std::num::NonZeroU16, "NonZeroU16");
    duper_serde_module!(DuperNonZeroU32, ::std::num::NonZeroU32, "NonZeroU32");
    duper_serde_module!(DuperNonZeroU64, ::std::num::NonZeroU64, "NonZeroU64");
    duper_serde_module!(DuperNonZeroU128, ::std::num::NonZeroU128, "NonZeroU128");
    duper_serde_module!(DuperNonZeroUsize, ::std::num::NonZeroUsize, "NonZeroUsize");

    duper_serde_module!(DuperWrapping<T>, ::std::num::Wrapping<T>, "Wrapping");
    duper_serde_module!(DuperSaturating<T>, ::std::num::Saturating<T>, "Saturating");
}

pub mod atomic {
    use super::*;

    duper_serde_module!(
        DuperAtomicBool,
        ::std::sync::atomic::AtomicBool,
        "AtomicBool"
    );
    duper_serde_module!(DuperAtomicI8, ::std::sync::atomic::AtomicI8, "AtomicI8");
    duper_serde_module!(DuperAtomicI16, ::std::sync::atomic::AtomicI16, "AtomicI16");
    duper_serde_module!(DuperAtomicI32, ::std::sync::atomic::AtomicI32, "AtomicI32");
    duper_serde_module!(DuperAtomicI64, ::std::sync::atomic::AtomicI64, "AtomicI64");
    duper_serde_module!(
        DuperAtomicIsize,
        ::std::sync::atomic::AtomicIsize,
        "AtomicIsize"
    );
    duper_serde_module!(DuperAtomicU8, ::std::sync::atomic::AtomicU8, "AtomicU8");
    duper_serde_module!(DuperAtomicU16, ::std::sync::atomic::AtomicU16, "AtomicU16");
    duper_serde_module!(DuperAtomicU32, ::std::sync::atomic::AtomicU32, "AtomicU32");
    duper_serde_module!(DuperAtomicU64, ::std::sync::atomic::AtomicU64, "AtomicU64");
    duper_serde_module!(
        DuperAtomicUsize,
        ::std::sync::atomic::AtomicUsize,
        "AtomicUsize"
    );
}

duper_serde_module!(DuperDuration, ::std::time::Duration, "Duration");
duper_serde_module!(DuperSystemTime, ::std::time::SystemTime, "SystemTime");

duper_serde_module!(DuperPathBuf, ::std::path::PathBuf, "PathBuf");

pub mod DuperPath {
    use super::*;

    pub fn serialize<S>(value: &::std::path::Path, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Path", value)
    }
}

pub mod ffi {
    use super::*;

    duper_serde_module!(DuperCString, ::std::ffi::CString, "CString");
    duper_serde_module!(DuperOsString, ::std::ffi::OsString, "OsString");

    pub mod DuperCStr {
        use super::*;

        pub fn serialize<S>(value: &::std::ffi::CStr, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("CStr", value)
        }
    }

    pub mod DuperOsStr {
        use super::*;

        pub fn serialize<S>(value: &::std::ffi::OsStr, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("OsStr", value)
        }
    }
}

// TO-DO: Specialization...
pub mod DuperBox {
    use super::*;

    pub fn serialize<S, T>(value: &::std::boxed::Box<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        (**value).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::boxed::Box<T>, D::Error>
    where
        T: ?Sized + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(::std::boxed::Box::new)
    }
}
pub mod DuperCow {
    use super::*;

    pub fn serialize<'a, T, S>(
        value: &::std::borrow::Cow<'a, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize + ToOwned,
    {
        (**value).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::borrow::Cow<'de, T>, D::Error>
    where
        T: ?Sized + ToOwned,
        T::Owned: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::Owned::deserialize(deserializer).map(::std::borrow::Cow::Owned)
    }
}
pub mod DuperRc {
    use super::*;

    pub fn serialize<S, T>(value: &::std::rc::Rc<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        (**value).serialize(serializer)
    }
}
pub mod DuperRcWeak {
    use super::*;

    pub fn serialize<S, T>(value: &::std::rc::Weak<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        value.upgrade().serialize(serializer)
    }
}
pub mod DuperArc {
    use super::*;

    pub fn serialize<S, T>(value: &::std::sync::Arc<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        (**value).serialize(serializer)
    }
}
pub mod DuperArcWeak {
    use super::*;

    pub fn serialize<S, T>(value: &::std::sync::Weak<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        value.upgrade().serialize(serializer)
    }
}
pub mod DuperCell {
    use super::*;

    pub fn serialize<S, T>(value: &::std::cell::Cell<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Copy,
    {
        value.get().serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::cell::Cell<T>, D::Error>
    where
        T: Deserialize<'de> + Copy,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(::std::cell::Cell::new)
    }
}
pub mod DuperRefCell {
    use super::*;
    use serde_core::ser::Error;

    pub fn serialize<S, T>(
        value: &::std::cell::RefCell<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        match value.try_borrow() {
            Ok(value) => value.serialize(serializer),
            Err(_) => Err(S::Error::custom("already mutably borrowed")),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::cell::RefCell<T>, D::Error>
    where
        T: ?Sized + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(::std::cell::RefCell::new)
    }
}
pub mod DuperMutex {
    use super::*;
    use serde_core::ser::Error;

    pub fn serialize<S, T>(value: &::std::sync::Mutex<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        match value.lock() {
            Ok(value) => value.serialize(serializer),
            Err(_) => Err(S::Error::custom("lock poison error while serializing")),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::sync::Mutex<T>, D::Error>
    where
        T: ?Sized + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(::std::sync::Mutex::new)
    }
}
pub mod DuperRwLock {
    use super::*;
    use serde_core::ser::Error;

    pub fn serialize<S, T>(value: &::std::sync::RwLock<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ?Sized + Serialize,
    {
        match value.read() {
            Ok(value) => value.serialize(serializer),
            Err(_) => Err(S::Error::custom("lock poison error while serializing")),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<::std::sync::RwLock<T>, D::Error>
    where
        T: ?Sized + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(::std::sync::RwLock::new)
    }
}

duper_serde_module!(DuperVec<T>, ::std::vec::Vec<T>, "Vec");
duper_serde_module!(DuperReverse<T>, ::std::cmp::Reverse<T>, "Reverse");

pub mod collections {
    pub use super::*;

    duper_serde_module!(
        DuperBinaryHeap<T>,
        ::std::collections::BinaryHeap<T>,
        "BinaryHeap"
    );
    duper_serde_module!(
        DuperBTreeSet<T>,
        ::std::collections::BTreeSet<T>,
        "BTreeSet"
    );
    duper_serde_module!(DuperHashSet<T>, ::std::collections::HashSet<T>, "HashSet");
    duper_serde_module!(
        DuperLinkedList<T>,
        ::std::collections::LinkedList<T>,
        "LinkedList"
    );
    duper_serde_module!(
        DuperVecDeque<T>,
        ::std::collections::VecDeque<T>,
        "VecDeque"
    );
    duper_serde_module!(
        DuperBTreeMap<K, V>,
        ::std::collections::BTreeMap<K, V>,
        "BTreeMap"
    );
    duper_serde_module!(
        DuperHashMap<K, V>,
        ::std::collections::HashMap<K, V>,
        "HashMap"
    );
}

// External crates

#[cfg(feature = "bytes")]
duper_serde_module!(DuperBytes, ::bytes::Bytes, "Bytes");

#[cfg(feature = "uuid")]
duper_serde_module!(DuperUuid, ::uuid::Uuid, "Uuid");

#[cfg(feature = "chrono")]
pub mod chrono {
    use super::*;

    duper_serde_module!(DuperNaiveDateTime, ::chrono::NaiveDateTime, "NaiveDateTime");
    duper_serde_module!(DuperNaiveDate, ::chrono::NaiveDate, "NaiveDate");
    duper_serde_module!(DuperNaiveTime, ::chrono::NaiveTime, "NaiveTime");
    duper_serde_module!(DuperTimeDelta, ::chrono::TimeDelta, "TimeDelta");

    pub mod DuperDateTime {
        use super::*;
        use ::chrono::{DateTime as WrappedType, TimeZone};

        pub fn serialize<S, T>(value: &WrappedType<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: TimeZone,
            WrappedType<T>: Serialize,
        {
            serializer.serialize_newtype_struct("DateTime", value)
        }

        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<WrappedType<T>, D::Error>
        where
            D: Deserializer<'de>,
            T: TimeZone,
            WrappedType<T>: Deserialize<'de>,
        {
            WrappedType::<T>::deserialize(deserializer)
        }
    }
}
