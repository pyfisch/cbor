use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn key_value<'a>(key: Value<'a>, value: Value<'a>) -> (Value<'a>, Value<'a>) {
    (key, value)
}

pub fn map<'a>(pairs: &'a [(Value<'a>, Value<'a>)]) -> Value<'a> {
    let length = match uint(pairs.len() as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::Map { length, pairs }.into()
}

pub fn indefinite_map<'a>(pairs: &'a [(Value<'a>, Value<'a>)]) -> Value<'a> {
    MajorType::IndefiniteMap { pairs }.into()
}
