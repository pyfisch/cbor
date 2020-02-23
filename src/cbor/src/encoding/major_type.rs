#![allow(clippy::many_single_char_names)]
use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bytes {
    SameByte(u8),
    OneByte(u8),
    TwoBytes(u8, u8),
    FourBytes(u8, u8, u8, u8),
    EightBytes(u8, u8, u8, u8, u8, u8, u8, u8),
}

impl Bytes {
    /// Returns the minor type of these bytes.
    pub fn minor(&self) -> u8 {
        match self {
            Bytes::SameByte(a) => *a,
            Bytes::OneByte(_) => 24,
            Bytes::TwoBytes(_, _) => 25,
            Bytes::FourBytes(_, _, _, _) => 26,
            Bytes::EightBytes(_, _, _, _, _, _, _, _) => 27,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Bytes::SameByte(_) => 0,
            Bytes::OneByte(_) => 1,
            Bytes::TwoBytes(_, _) => 2,
            Bytes::FourBytes(_, _, _, _) => 4,
            Bytes::EightBytes(_, _, _, _, _, _, _, _) => 8,
        }
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match *self {
            Bytes::SameByte(_) => Ok(()),
            Bytes::OneByte(a) => w.write(&[a]),
            Bytes::TwoBytes(a, b) => w.write(&[a, b]),
            Bytes::FourBytes(a, b, c, d) => w.write(&[a, b, c, d]),
            Bytes::EightBytes(a, b, c, d, e, f, g, h) => w.write(&[a, b, c, d, e, f, g, h]),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// Major type 0: an unsigned integer.
    UnsignedInteger(Bytes),
    /// Major type 1: a negative integer. This can represent up to 2^64 (which is twice as many
    /// as i64). There is no representation of this in Rust, so people should use [negative_int]
    /// for numbers above 2^63 - 1.
    NegativeInteger(Bytes),

    /// Major type 2: a byte string.
    Bytes { length: Bytes, bytes: &'a [u8] },
    //// Major type 3: a text string.
    // Text(Bytes, &'a str),
    //    Tag(u64, Token<'a>),
}

impl Token<'_> {
    pub fn len(&self) -> usize {
        match self {
            Token::UnsignedInteger(bytes) => 1 + bytes.len(),
            Token::NegativeInteger(bytes) => 1 + bytes.len(),
            Token::Bytes { length, bytes } => 1 + length.len() + bytes.len(),
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match self {
            Token::UnsignedInteger(bytes) => {
                w.write(&[(0 << 5) + bytes.minor()])?;
                bytes.write(w)
            }
            Token::NegativeInteger(bytes) => {
                w.write(&[(1 << 5) + bytes.minor()])?;
                bytes.write(w)
            }
            Token::Bytes { length, bytes } => {
                w.write(&[(2 << 5) + length.minor()])?;
                length.write(w)?;
                w.write(*bytes)
            }
        }
    }
}
