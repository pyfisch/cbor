use std::cell::RefCell;

pub const CBOR_NEWTYPE_NAME: &str = "__cbor_tag";

/// extensions for all serde serializers
pub trait SerializerExt: serde::ser::Serializer {
    /// basically serialize_newtype_struct with a cbor tag value
    fn serialize_cbor_tagged<T: serde::ser::Serialize>(
        self,
        tag: u64,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        set_tag(Some(tag));
        let r = self.serialize_newtype_struct(CBOR_NEWTYPE_NAME, value);
        set_tag(None);
        r
    }
}

impl<S: serde::ser::Serializer> SerializerExt for S {}

thread_local!(static CBOR_TAG: RefCell<Option<u64>> = RefCell::new(None));

pub(crate) fn set_tag(value: Option<u64>) {
    CBOR_TAG.with(|f| {
        *f.borrow_mut() = value;
    });
}

pub(crate) fn get_tag() -> Option<u64> {
    CBOR_TAG.with(|f| *f.borrow())
}
