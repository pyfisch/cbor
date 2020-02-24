use crate::encoding::major_type::MajorType;
use crate::serialize::values::{uint, Value};

pub fn text(string: &str) -> Value {
    let length = match uint(string.len() as u64).inner {
        MajorType::UnsignedInteger(bytes) => bytes,
        _ => unreachable!(),
    };

    MajorType::Text { length, string }.into()
}
