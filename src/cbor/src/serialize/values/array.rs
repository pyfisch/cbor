use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn array<'a>(values: &'a [Value]) -> Value<'a> {
    let length = match uint(values.len() as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::Array { length, values }.into()
}
