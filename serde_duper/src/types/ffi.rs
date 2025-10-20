use super::*;

duper_serde_module!(
    DuperCString,
    DuperOptionCString,
    ::std::ffi::CString,
    "CString"
);

pub mod DuperCStr {
    use super::*;

    pub fn serialize<S>(value: &::std::ffi::CStr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("CStr", value)
    }
}

pub mod DuperOptionCStr {
    use super::*;

    pub fn serialize<S>(value: &Option<&::std::ffi::CStr>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("CStr", &value)
    }
}

// Helper type for deserialization
enum OsString<'a> {
    #[cfg(unix)]
    Unix(::std::borrow::Cow<'a, [u8]>),
    #[cfg(windows)]
    Windows(::std::borrow::Cow<'a, [u16]>),
}

// Helper visitor for deserialization
struct OsStringVisitor;

impl<'de> ::serde_core::de::Visitor<'de> for OsStringVisitor {
    type Value = OsString<'de>;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        #[cfg(unix)]
        {
            formatter.write_str("a Unix OsString")
        }
        #[cfg(windows)]
        {
            formatter.write_str("a Windows OsString")
        }
    }

    #[cfg(unix)]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: ::serde_core::de::Error,
    {
        Ok(OsString::Unix(std::borrow::Cow::Owned(v.to_vec())))
    }

    #[cfg(unix)]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: ::serde_core::de::Error,
    {
        Ok(OsString::Unix(std::borrow::Cow::Borrowed(v)))
    }

    #[cfg(unix)]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: ::serde_core::de::Error,
    {
        Ok(OsString::Unix(std::borrow::Cow::Owned(v)))
    }

    #[cfg(unix)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: ::serde_core::de::SeqAccess<'de>,
    {
        let mut vec = seq.size_hint().map(Vec::with_capacity).unwrap_or_default();
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }
        Ok(OsString::Unix(std::borrow::Cow::Owned(vec)))
    }
    #[cfg(windows)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: ::serde_core::de::SeqAccess<'de>,
    {
        let mut vec = seq.size_hint().map(Vec::with_capacity).unwrap_or_default();
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }
        Ok(OsString::Windows(std::borrow::Cow::Owned(vec)))
    }

    #[cfg(unix)]
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: ::serde_core::de::EnumAccess<'de>,
    {
        use ::serde_core::de::VariantAccess;

        let (variant, value) = data.variant::<::std::borrow::Cow<'de, str>>()?;
        match variant.as_ref() {
            "Unix" => Ok(OsString::Unix(value.newtype_variant()?)),
            "Windows" => Err(::serde_core::de::Error::custom(
                "cannot deserialize Windows OsString in Unix",
            )),
            variant => Err(::serde_core::de::Error::unknown_variant(variant, &["Unix"])),
        }
    }
    #[cfg(windows)]
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: ::serde_core::de::EnumAccess<'de>,
    {
        use ::serde_core::de::VariantAccess;

        let (variant, value) = data.variant::<::std::borrow::Cow<'de, str>>()?;
        match variant.as_ref() {
            "Windows" => Ok(OsString::Windows(value.newtype_variant()?)),
            "Unix" => Err(::serde_core::de::Error::custom(
                "cannot deserialize Unix OsString in Windows",
            )),
            variant => Err(::serde_core::de::Error::unknown_variant(
                variant,
                &["Windows"],
            )),
        }
    }
}

#[cfg(unix)]
impl<'de> Deserialize<'de> for OsString<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum("OsString", &["Unix"], OsStringVisitor)
    }
}
#[cfg(windows)]
impl<'de> Deserialize<'de> for OsString<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum("OsString", &["Windows"], OsStringVisitor)
    }
}

pub mod DuperOsString {
    #[cfg(unix)]
    use ::std::os::unix::ffi::{OsStrExt, OsStringExt};
    #[cfg(windows)]
    use ::std::os::windows::ffi::{OsStrExt, OsStringExt};

    use super::*;

    #[cfg(unix)]
    impl Serialize for OsString<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let OsString::Unix(bytes) = self;
            serializer.serialize_bytes(bytes)
        }
    }
    #[cfg(windows)]
    impl Serialize for OsString<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let OsString::windows(bytes) = self;
            serializer.serialize_bytes(bytes)
        }
    }

    #[cfg(unix)]
    pub fn serialize<S>(value: &::std::ffi::OsString, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            "OsString",
            0,
            "Unix",
            &OsString::Unix(::std::borrow::Cow::Borrowed(value.as_bytes())),
        )
    }
    #[cfg(windows)]
    pub fn serialize<S>(value: &::std::ffi::OsString, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            "OsString",
            0,
            "Windows",
            &OsString::Windows(::std::borrow::Cow::Owned(value.encode_wide().collect())),
        )
    }

    #[cfg(unix)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::std::ffi::OsString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let OsString::Unix(value) = OsString::deserialize(deserializer)?;
        Ok(::std::ffi::OsString::from_vec(value.into_owned()))
    }
    #[cfg(windows)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::std::ffi::OsString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let OsString::Windows(value) = OsString::deserialize(deserializer)?;
        Ok(::std::ffi::OsString::from_wide(value.into_owned()))
    }
}

pub mod DuperOptionOsString {
    #[cfg(unix)]
    use ::std::os::unix::ffi::OsStringExt;
    #[cfg(windows)]
    use ::std::os::windows::ffi::{OsStrExt, OsStringExt};

    use super::*;

    pub fn serialize<S>(
        value: &Option<::std::ffi::OsString>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => DuperOsString::serialize(value, serializer),
            None => serializer
                .serialize_newtype_struct("OsString", &Option::<::std::ffi::OsString>::None),
        }
    }

    struct OptionOsString<'a>(Option<OsString<'a>>);

    struct OptionOsStringVisitor;

    impl<'de> ::serde_core::de::Visitor<'de> for OptionOsStringVisitor {
        type Value = OptionOsString<'de>;

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            #[cfg(unix)]
            {
                formatter.write_str("an optional Unix OsString")
            }
            #[cfg(windows)]
            {
                formatter.write_str("an optional Windows OsString")
            }
        }

        #[cfg(unix)]
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(OptionOsString(Some(deserializer.deserialize_enum(
                "OsString",
                &["Unix"],
                OsStringVisitor,
            )?)))
        }
        #[cfg(windows)]
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(OptionOsString(Some(deserializer.deserialize_enum(
                "OsString",
                &["Windows"],
                OsStringVisitor,
            )?)))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: ::serde_core::de::Error,
        {
            Ok(OptionOsString(None))
        }
    }

    impl<'de> Deserialize<'de> for OptionOsString<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_option(OptionOsStringVisitor)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<::std::ffi::OsString>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match OptionOsString::deserialize(deserializer)?.0 {
            Some(os_string) => {
                #[cfg(unix)]
                {
                    let OsString::Unix(value) = os_string;
                    Ok(Some(::std::ffi::OsString::from_vec(value.into_owned())))
                }
                #[cfg(windows)]
                {
                    let OsString::Windows(value) = os_string;
                    Ok(Some(::std::ffi::OsString::from_wide(value.into_owned())))
                }
            }
            None => Ok(None),
        }
    }
}

pub mod DuperOsStr {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStrExt;

    use super::*;

    enum OsStr<'a> {
        #[cfg(unix)]
        Unix(&'a [u8]),
    }

    #[cfg(unix)]
    impl Serialize for OsStr<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let OsStr::Unix(bytes) = self;
            serializer.serialize_bytes(bytes)
        }
    }
    #[cfg(windows)]
    impl Serialize for OsStr<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let OsStr::Windows(wides) = self;
            let mut seq = serializer.serialize_seq(wides.len());
            for wide in wides {
                seq.serialize_u16(wide)?;
            }
            seq.end();
        }
    }

    #[cfg(unix)]
    pub fn serialize<S>(value: &::std::ffi::OsStr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant("OsStr", 0, "Unix", &OsStr::Unix(value.as_bytes()))
    }
    #[cfg(windows)]
    pub fn serialize<S>(value: &::std::ffi::OsStr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_variant(
            "OsStr",
            0,
            "Windows",
            &OsStr::Windows(value.encode_wide()),
        )
    }
}

pub mod DuperOptionOsStr {
    use super::*;

    pub fn serialize<S>(
        value: &Option<&::std::ffi::OsStr>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => DuperOsStr::serialize(value, serializer),
            None => {
                serializer.serialize_newtype_struct("OsStr", &Option::<::std::ffi::OsString>::None)
            }
        }
    }
}
