use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn tag<'a>(tag: u16, value: &'a Value<'a>) -> Value<'a> {
    let tag = match uint(tag as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::Tag {
        tag,
        value: &value.inner,
    }
    .into()
}
