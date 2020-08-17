mod numbers;
pub use numbers::*;
#[cfg(test)]
mod numbers_test;

mod text;
pub use text::*;

#[cfg(test)]
mod text_test;

use crate::serialize::values::Value;

pub fn peek<'a>(bytes: &'a [u8]) -> Option<Value<'a>> {
    // In order to maintain the SAME VALUE serialization as the peek, use every type of numbers
    // with their value counterpart.
    numbers::usmall(bytes)
        .or_else(|| numbers::u8(bytes))
        .or_else(|| numbers::u16(bytes))
        .or_else(|| numbers::u32(bytes))
        .or_else(|| numbers::u64(bytes))
        .or_else(|| numbers::negative_usmall(bytes))
        .or_else(|| numbers::negative_u8(bytes))
        .or_else(|| numbers::negative_u16(bytes))
        .or_else(|| numbers::negative_u32(bytes))
        .or_else(|| numbers::negative_u64(bytes))
        .or_else(|| text::text(bytes))
}
