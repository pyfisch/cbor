//! Support for cbor tags
use core::{fmt, marker};
use serde::ser::{Serialize, Serializer};

/// signals that a newtype is from a CBOR tag
pub(crate) const CBOR_NEWTYPE_NAME: &str = "\0cbor_tag";

/// A value that is optionally tagged with a cbor tag
///
/// this only serves as an intermediate helper for tag serialization or deserialization
pub struct Tagged<T> {
    /// cbor tag
    pub tag: Option<u64>,
    /// value
    pub value: T,
}

impl<T> Tagged<T> {
    /// Create a new tagged value
    pub fn new(tag: Option<u64>, value: T) -> Self {
        Self { tag, value }
    }

    /// Get the inner value if the cbor tag has the expected value
    pub fn unwrap_if_tag<'de, D: serde::de::Deserializer<'de>>(
        self,
        expected_tag: u64,
    ) -> Result<T, D::Error> {
        match self.tag {
            Some(tag) if tag == expected_tag => Ok(self.value),
            Some(_) => Err(serde::de::Error::custom("unexpected cbor tag")),
            None => Err(serde::de::Error::custom("missing cbor tag")),
        }
    }
}

impl<T: Serialize> Serialize for Tagged<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        set_tag(self.tag);
        let r = s.serialize_newtype_struct(CBOR_NEWTYPE_NAME, &self.value);
        set_tag(None);
        r
    }
}

impl<'de, T: serde::de::Deserialize<'de>> serde::de::Deserialize<'de> for Tagged<T> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ValueVisitor<T>(marker::PhantomData<T>);

        impl<'de, T: serde::de::Deserialize<'de>> serde::de::Visitor<'de> for ValueVisitor<T> {
            type Value = Tagged<T>;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("a cbor tag newtype")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let t = get_tag();
                T::deserialize(deserializer).map(|v| Tagged::new(t, v))
            }
        }

        deserializer.deserialize_any(ValueVisitor::<T>(marker::PhantomData))
    }
}

/// function to get the current cbor tag
///
/// The only place where it makes sense to call this function is within visit_newtype_struct of a serde visitor.
/// This is a low level API. In most cases it is preferable to use Tagged
pub fn current_cbor_tag() -> Option<u64> {
    get_tag()
}

#[cfg(feature = "tags")]
pub(crate) fn set_tag(value: Option<u64>) {
    CBOR_TAG.with(|f| *f.borrow_mut() = value);
}

#[cfg(feature = "tags")]
pub(crate) fn get_tag() -> Option<u64> {
    CBOR_TAG.with(|f| *f.borrow())
}

#[cfg(not(feature = "tags"))]
pub(crate) fn set_tag(_value: Option<u64>) {}

#[cfg(not(feature = "tags"))]
pub(crate) fn get_tag() -> Option<u64> {
    None
}

#[cfg(feature = "tags")]
use std::cell::RefCell;

#[cfg(feature = "tags")]
thread_local!(static CBOR_TAG: RefCell<Option<u64>> = RefCell::new(None));
