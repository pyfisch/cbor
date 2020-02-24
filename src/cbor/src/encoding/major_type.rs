#![allow(clippy::many_single_char_names)]
use crate::encoding::minor_type::MinorType;
use crate::serialize::values::Value;
use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MajorType<'a> {
    /// Major type 0: an unsigned integer.
    UnsignedInteger(MinorType),

    /// Major type 1: a negative integer. This can represent up to 2^64 (which is twice as many
    /// as i64). There is no representation of this in Rust, so people should use [negative_u64]
    /// for numbers above 2^63 - 1.
    NegativeInteger(MinorType),

    /// Major type 2: a byte string.
    Bytes { length: MinorType, bytes: &'a [u8] },

    /// Major type 3: a text string.
    Text { length: MinorType, string: &'a str },

    /// Major type 4: an array.
    Array {
        length: MinorType,
        values: &'a [Value<'a>],
    },

    /// Major type 4 (subtype 31): an indefinite size array.
    IndefiniteArrayStart,

    /// Major type 5: a map of pairs.
    Map {
        length: MinorType,
        pairs: &'a [(Value<'a>, Value<'a>)],
    },

    /// Major type 5 (subtype 31): an indefinite size map.
    IndefiniteMapStart,

    /// Major type 6: a semantic tagging of MajorType.
    Tag {
        tag: MinorType,
        value: &'a MajorType<'a>,
    },

    // Major type 7, subtype 0..19 are unassigned.
    /// Major type 7, subtype 20
    False,
    /// Major type 7, subtype 21
    True,
    /// Major type 7, subtype 22
    Null,
    /// Major type 7, subtype 23
    Undefined,

    /// Major type 7, subtype 0..19, 28..30, 32..255.
    /// This exists to allow for decoding of values without loss of information.
    Unassigned(u8),

    /// Major type 7, subtype 25
    HalfFloat(f32),
    /// Major type 7, subtype 26
    SingleFloat(f32),
    /// Major type 7, subtype 27
    DoubleFloat(f64),

    /// Major type 7, subtype 31
    Break,
}

impl MajorType<'_> {
    pub fn is_empty(&self) -> bool {
        // Major type always has 1 byte.
        false
    }

    pub fn len(&self) -> usize {
        match self {
            MajorType::UnsignedInteger(bytes) => 1 + bytes.len(),
            MajorType::NegativeInteger(bytes) => 1 + bytes.len(),
            MajorType::Bytes { length, bytes } => 1 + length.len() + bytes.len(),
            MajorType::Text { length, string } => 1 + length.len() + string.len(),
            MajorType::Array { length, values } => {
                let mut l = 1 + length.len();
                for i in 0..values.len() {
                    l += values[i].len();
                }
                l
            }
            MajorType::Map { length, pairs } => {
                let mut l = 1 + length.len();
                for i in 0..pairs.len() {
                    l += pairs[i].0.len() + pairs[i].1.len();
                }
                l
            }
            MajorType::Tag { tag, value } => 1 + tag.len() + value.len(),
            MajorType::False => 1,
            MajorType::True => 1,
            MajorType::Null => 1,
            MajorType::Undefined => 1,
            MajorType::UnassignedMajorType7(_) => 1,
            MajorType(_) => 1,
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match self {
            MajorType::UnsignedInteger(bytes) => {
                w.write(&[bytes.minor()])?;
                bytes.write(w)
            }
            MajorType::NegativeInteger(bytes) => {
                w.write(&[(1 << 5) + bytes.minor()])?;
                bytes.write(w)
            }
            MajorType::Bytes { length, bytes } => {
                w.write(&[(2 << 5) + length.minor()])?;
                length.write(w)?;
                w.write(*bytes)
            }
            MajorType::Text { length, string } => {
                w.write(&[(3 << 5) + length.minor()])?;
                length.write(w)?;
                w.write(string.as_bytes())
            }
            MajorType::Array { length, values } => {
                w.write(&[(4 << 5) + length.minor()])?;
                length.write(w)?;
                for i in 0..values.len() {
                    values[i].write(w)?;
                }
                Ok(())
            }
            MajorType::Map { length, pairs } => {
                w.write(&[(5 << 5) + length.minor()])?;
                length.write(w)?;
                for i in 0..pairs.len() {
                    pairs[i].0.write(w)?;
                    pairs[i].1.write(w)?;
                }
                Ok(())
            }
            MajorType::Tag { tag, value } => {
                w.write(&[(6 << 5) + tag.minor()])?;
                tag.write(w)?;
                value.write(w)
            }
        }
    }
}
