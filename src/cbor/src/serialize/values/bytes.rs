use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn bytes(bytes: &[u8]) -> Value {
    let length = match uint(bytes.len() as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::Bytes { length, bytes }.into()
}
