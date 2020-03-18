#![allow(clippy::many_single_char_names)]
use crate::encoding::minor_type::MinorType;
use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MajorType {
    /// Major type 0: an unsigned (positive) integer. The MinorType is the value of the integer.
    UnsignedInteger(MinorType),

    /// Major type 1: a negative integer. This can represent up to 2^64 (or -2^64 -1, which is
    /// more than twice as many as i64). There are no representation of this in Rust, so people
    /// should use [negative_u64] for numbers below -2^63 - 1.
    NegativeInteger(MinorType),

    /// Major type 2: a byte string. The MinorType is the length of the byte string, or indefinite
    /// for an indefinitely long byte string.
    ByteString(MinorType),

    /// Major type 3: a text string. The MinorType is the length of the string.
    /// If the MinorType is Indefinite, the text has no determinate length, and ends
    /// with a break.
    Text(MinorType),

    /// Major type 4: an array. The MinorType is the length of the array.
    /// If the MinorType is Indefinite, the array has no length, and ends
    /// with a break.
    Array(MinorType),

    /// Major type 5: a map of pairs. The MinorType is the number of key-value pairs.
    Map(MinorType),

    /// Major type 6: a semantic tagging of MajorType. The MinorType is the tag value.
    Tag(MinorType),

    /// Major type 7, subtype 20
    False(),
    /// Major type 7, subtype 21
    True(),
    /// Major type 7, subtype 22
    Null(),
    /// Major type 7, subtype 23
    Undefined(),

    /// Major type 7, subtype 25. This is only used for passing values if they are already
    /// decoded / encoded. You need the "half_float" feature enabled to be able to serialize
    /// or deserialize those.
    HalfFloat(u16),
    /// Major type 7, subtype 26
    SingleFloat(u32),
    /// Major type 7, subtype 27
    DoubleFloat(u64),

    /// Major type 7, subtype 0..19, 24, 28-30. Map to subtypes 0..19, 28..30, 32..255.
    /// This exists to allow for decoding of values without loss of information.
    UnassignedSimpleData(u8),

    /// Major type 7, subtype 31
    Break(),
}

impl MajorType {
    pub fn len(&self) -> usize {
        match self {
            MajorType::UnsignedInteger(minor) => 1 + minor.len(),
            MajorType::NegativeInteger(minor) => 1 + minor.len(),
            MajorType::ByteString(minor) => 1 + minor.len(),
            MajorType::Text(minor) => 1 + minor.len(),
            MajorType::Array(minor) => 1 + minor.len(),
            MajorType::Map(minor) => 1 + minor.len(),
            MajorType::Tag(minor) => 1 + minor.len(),
            MajorType::False() => 1,
            MajorType::True() => 1,
            MajorType::Null() => 1,
            MajorType::Undefined() => 1,
            MajorType::HalfFloat(_) => 3,
            MajorType::SingleFloat(_) => 5,
            MajorType::DoubleFloat(_) => 9,
            MajorType::UnassignedSimpleData(d) => {
                if *d >= 32 {
                    2
                } else {
                    1
                }
            }
            MajorType::Break() => 1,
        }
    }

    pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match self {
            MajorType::UnsignedInteger(minor) => {
                w.write(&[0 + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::NegativeInteger(minor) => {
                w.write(&[(1 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::ByteString(minor) => {
                w.write(&[(2 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::Text(minor) => {
                w.write(&[(3 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::Array(minor) => {
                w.write(&[(4 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::Map(minor) => {
                w.write(&[(5 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::Tag(minor) => {
                w.write(&[(6 << 5) + minor.minor()])?;
                minor.write_to(w)
            }
            MajorType::False() => w.write(&[(7 << 5) + 20]),
            MajorType::True() => w.write(&[(7 << 5) + 21]),
            MajorType::Null() => w.write(&[(7 << 5) + 22]),
            MajorType::Undefined() => w.write(&[(7 << 5) + 23]),
            MajorType::HalfFloat(f) => w.write(&[(7 << 5) + 25, (*f >> 8) as u8, *f as u8]),
            MajorType::SingleFloat(f) => w.write(&[
                (7 << 5) + 26,
                (*f >> 24) as u8,
                (*f >> 16) as u8,
                (*f >> 8) as u8,
                *f as u8,
            ]),
            MajorType::DoubleFloat(f) => w.write(&[
                (7 << 5) + 27,
                (*f >> 56) as u8,
                (*f >> 48) as u8,
                (*f >> 40) as u8,
                (*f >> 32) as u8,
                (*f >> 24) as u8,
                (*f >> 16) as u8,
                (*f >> 8) as u8,
                *f as u8,
            ]),
            MajorType::UnassignedSimpleData(d) => {
                if *d >= 32 {
                    w.write(&[(7 << 5) + 24, *d])
                } else {
                    w.write(&[(7 << 5) + *d])
                }
            }
            MajorType::Break() => w.write(&[(7 << 5) + 31]),
        }
    }
}
