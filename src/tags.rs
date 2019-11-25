//! Support for cbor tags
use serde::de::{Deserializer, Error};
use serde::ser::{Serialize, Serializer};

/// signals that a newtype is from a CBOR tag
pub(crate) const CBOR_NEWTYPE_NAME: &str = "\0cbor_tag";

/// extensions for all serde serializers to add cbor tag functionality
pub trait SerializerExt: Serializer {
    /// basically serialize_newtype_struct with a cbor tag value
    fn serialize_cbor_tagged<T: Serialize>(
        self,
        tag: u64,
        value: &T,
    ) -> core::result::Result<Self::Ok, Self::Error> {
        set_tag(Some(tag));
        let r = self.serialize_newtype_struct(CBOR_NEWTYPE_NAME, value);
        set_tag(None);
        r
    }
}

impl<S: Serializer> SerializerExt for S {}

/// extensions for all serde deserializers to add cbor tag functionality
pub trait DeserializerExt<'de>: Deserializer<'de> {
    /// get the current cbor tag
    fn get_cbor_tag(&self) -> Option<u64> {
        get_tag()
    }

    /// expect the given cbor tag
    fn expect_cbor_tag(&self, tag: u64) -> Result<(), Self::Error> {
        match get_tag() {
            Some(t) if t == tag => Ok(()),
            Some(_) => Err(Self::Error::custom("unexpected cbor tag")),
            None => Err(Self::Error::custom("missing cbor tag!")),
        }
    }
}

impl<'de, D: Deserializer<'de>> DeserializerExt<'de> for D {}

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
