use crate::encoding::major_type::{Bytes, Token};
use crate::serialize::values::Value;

/// A small natural (23 or less). This will clamp the number.
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

/// A small integer of range `[-24, 23]`. This will clamp the number.
pub fn ismall(v: i8) -> Value<'static> {
    if v < 0 {
        negative_usmall(-(v as i16 + 1) as u8)
    } else {
        usmall(v as u8)
    }
}

/// An 8 bits integer of range `[-128, 127]`.
///
/// Note that not all CBOR 1 byte numbers can be represented with this function. Positive
/// naturals are of range `[0, 255]` and negative integer are `[-256, -1]`. To encode
/// those values you need to use either [u8()] for positive numbers, or [negative_u8()]
/// for negative numbers.
pub fn i8(v: i8) -> Value<'static> {
    if v < 0 {
        negative_u8(-(v as i16 + 1) as u8)
    } else {
        u8(v as u8)
    }
}

/// A 16 bits integer of range `[-32768, 32767]`.
///
/// Note that not all CBOR 2 bytes numbers can be represented with this function. Positive
/// naturals are of range `[0, 65535]` and negative integer are `[-65536, -1]`. To encode
/// those values you need to use either [u16()] for positive numbers, or [negative_u16()]
/// for negative numbers.
pub fn i16(v: i16) -> Value<'static> {
    if v < 0 {
        negative_u16(-(v as i32 + 1) as u16)
    } else {
        u16(v as u16)
    }
}

/// A 32 bits integer of range `[-2**31 - 1, 2**31]`.
///
/// Note that not all CBOR 4 bytes numbers can be represented with this function. Positive
/// naturals are of range `[0, 2**32]` and negative integer are `[-2**32 - 1, -1]`. To encode
/// those values you need to use either [u32()] for positive numbers, or [negative_u32()]
/// for negative numbers.
pub fn i32(v: i32) -> Value<'static> {
    if v < 0 {
        negative_u32(-(v as i64 + 1) as u32)
    } else {
        u32(v as u32)
    }
}

/// An integer that will be represented by the smallest amount of data possible.
pub fn int(v: i64) -> Value<'static> {
    if v < 0 {
        negative_uint(-(v as i128 + 1) as u64)
    } else {
        uint(v as u64)
    }
}

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A small negative number ([-24, -1]), passed as the raw u8. This will clamp the number
/// to correct values ([0, 23]). The CBOR spec says that negative numbers are decremented
/// before encoding, and the value received here should be representative of that.
/// Using `negative_small(3)` is supposed to represent -4. Similarly, `negative_small(0)` is
/// -1.
pub fn negative_usmall(v: u8) -> Value<'static> {
    if v > 23 {
        Token::NegativeInteger(Bytes::SameByte(23)).into()
    } else {
        Token::NegativeInteger(Bytes::SameByte(v)).into()
    }
}

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A 1 byte negative number, in the range of [-256, -1]. As with [negative_small], this
/// function encodes the number as is, e.g. `negative_i8(200)` is actually encoding -201.
pub fn negative_u8(v: u8) -> Value<'static> {
    Token::NegativeInteger(Bytes::OneByte(v)).into()
}

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A 2 bytes negative number, in the range of [-65536, -1]. As with [negative_small], this
/// function encodes the number as is, e.g. `negative_i8(200)` is actually encoding -201.
pub fn negative_u16(v: u16) -> Value<'static> {
    Token::NegativeInteger(Bytes::TwoBytes((v >> 8) as u8, (v >> 0) as u8)).into()
}

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A 4 bytes negative number, in the range of [-2**32 - 1, -1]. As with [negative_usmall], this
/// function encodes the number as is, e.g. `negative_i8(200)` is actually encoding -201.
pub fn negative_u32(v: u32) -> Value<'static> {
    Token::NegativeInteger(Bytes::FourBytes(
        (v >> 24) as u8,
        (v >> 16) as u8,
        (v >> 8) as u8,
        (v >> 0) as u8,
    ))
    .into()
}

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A 8 bytes negative number, in the range of [-2**64 - 1, -1]. As with [negative_usmall], this
/// function encodes the number as is, e.g. `negative_i8(200)` is actually encoding -201.
pub fn negative_u64(v: u64) -> Value<'static> {
    Token::NegativeInteger(Bytes::EightBytes(
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

/// The negative raw encoding methods. These should only be used if you know what you're
/// doing. They are provided for completeness of the spec (certain edge cases require
/// manually encoding values). If you want to encode values that are representative in
/// Rust (e.g. an i8 for [-128, 127]) you should use either [i8()] or if you know the
/// number must be negative [negative_i8()].
///
/// A 1 byte negative number, in the range of [-256, -1]. As with [negative_usmall], this
/// function encodes the number as is, e.g. `negative_i8(200)` is actually encoding -201.
pub fn negative_uint(v: u64) -> Value<'static> {
    if v < 24 {
        negative_usmall(v as u8)
    } else if v <= 0xFF {
        negative_u8(v as u8)
    } else if v <= 0xFFFF {
        negative_u16(v as u16)
    } else if v <= 0xFFFFFFFF {
        negative_u32(v as u32)
    } else {
        negative_u64(v)
    }
}
