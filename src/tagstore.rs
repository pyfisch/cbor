use std::cell::RefCell;

/// extensions for all serde serializers
pub trait SerializerExt: serde::ser::Serializer {

    /// basically serialize_newtype_struct with a cbor tag value
    fn serialize_cbor_tagged<T: serde::ser::Serialize>(self, tag: u64, value: &T) -> std::result::Result<Self::Ok, Self::Error> {
        set_tag(Some(tag));
        self.serialize_newtype_struct("__cbor_tag", value)
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
    CBOR_TAG.with(|f| {
        let mut b = f.borrow_mut();
        let r = *b;
        *b = None;
        r
    })
}
