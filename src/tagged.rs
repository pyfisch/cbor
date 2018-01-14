//! Helper function for emitting CBOR tags

use serde::ser::{Serialize, SerializeTupleStruct, Serializer};
use error;
use std::fmt;

pub(crate) static CBOR_NO_LENGTH_FIELD: &'static str = "__CBOR_NO_LENGTH_FIELD";

#[derive(Copy, Clone)]
pub(crate) struct CborTag(pub u64);

impl Serialize for CborTag {
    /// This function is never actually called, but only used for pointer comparisons
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        panic!("BUG: Should never be called");
    }
}

impl fmt::Display for CborTag {
    /// This function is never actually called, but only used for pointer comparisons
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        panic!("BUG: Should never be called");
    }
}

#[inline]
/// Serializes a value like normally, but if the serializer is the CBOR serializer, it will also
/// emit the tag value.
pub fn serialize_tagged<S, T>(tag: u64, value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    use serde::ser::Error;

    let s_fn = S::Error::custom::<CborTag>;
    let cbor_fn = error::Error::custom::<CborTag>;
    if s_fn as usize == cbor_fn as usize {
        // We should only get in here if we are in a serializer that uses the error type from this
        // crate. This should never happen for any other serializers.
        //
        // However if it does, then we will emit a tuple struct with name
        // __CBOR_NO_LENGTH_FIELD. For our serializer this triggers the behavior of outputting the
        // CBOR tag, but other in other serializers this is likely to panic.
        let mut tuple_serializer = serializer.serialize_tuple_struct(CBOR_NO_LENGTH_FIELD, 2)?;
        tuple_serializer.serialize_field(&CborTag(tag))?;
        tuple_serializer.serialize_field(&value)?;
        tuple_serializer.end()
    } else {
        value.serialize(serializer)
    }
}
