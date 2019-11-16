use std::cell::RefCell;

/// basically serialize_newtype_struct with a cbor tag value
pub fn serialize_cbor_tagged<S: serde::ser::Serializer, T: serde::ser::Serialize>(
    serializer: S,
    tag: u64,
    value: &T,
) -> std::result::Result<S::Ok, S::Error> {
    set_tag(Some(tag));
    serializer.serialize_newtype_struct("__cbor_tag", value)
}

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
