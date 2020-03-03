use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn bytes(bytes: &[u8]) -> Value {
    let length = match uint(bytes.len() as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::ByteString { length, bytes }.into()
}

pub fn indefinite_bytes<'a>(chunks: &'a [&'a [u8]]) -> Value<'a> {
    MajorType::IndefiniteByteString { chunks }.into()
}
