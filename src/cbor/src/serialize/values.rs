use crate::encoding::major_type::{Bytes, Token};
use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, PartialEq)]
pub struct Value<'a> {
    pub(crate) inner: Token<'a>,
}

impl Value<'_> {
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        self.inner.write(w)
    }

    /// The length in bytes of this value.
    pub fn len(&self) -> usize {
        match self.inner {
            Token::UnsignedInteger(bytes) => bytes.len() + 1,
            Token::NegativeInteger(bytes) => bytes.len() + 1,
            _ => 0,
        }
    }
}

impl<'a> From<Token<'a>> for Value<'a> {
    fn from(v: Token<'a>) -> Self {
        Value { inner: v }
    }
}

/// A small natural (23 or less). This will modulo the number.
pub fn usmall(v: u8) -> Value<'static> {
    if v < 24 {
        Token::UnsignedInteger(Bytes::SameByte(v)).into()
    } else {
        Token::UnsignedInteger(Bytes::SameByte(23)).into()
    }
}

pub fn u8(v: u8) -> Value<'static> {
    Token::UnsignedInteger(Bytes::OneByte(v)).into()
}

pub fn u16(v: u16) -> Value<'static> {
    Token::UnsignedInteger(Bytes::TwoBytes((v >> 8) as u8, (v >> 0) as u8)).into()
}

pub fn u32(v: u32) -> Value<'static> {
    Token::UnsignedInteger(Bytes::FourBytes(
        (v >> 24) as u8,
        (v >> 16) as u8,
        (v >> 8) as u8,
        (v >> 0) as u8,
    ))
    .into()
}

pub fn u64(v: u64) -> Value<'static> {
    Token::UnsignedInteger(Bytes::EightBytes(
        (v >> 56) as u8,
        (v >> 48) as u8,
        (v >> 40) as u8,
        (v >> 32) as u8,
        (v >> 24) as u8,
        (v >> 16) as u8,
        (v >> 8) as u8,
        (v >> 0) as u8,
    ))
    .into()
}

pub fn uint(v: u64) -> Value<'static> {
    if v < 24 {
        usmall(v as u8)
    } else if v <= 0xFF {
        u8(v as u8)
    } else if v <= 0xFFFF {
        u16(v as u16)
    } else if v <= 0xFFFFFFFF {
        u32(v as u32)
    } else {
        u64(v)
    }
}
