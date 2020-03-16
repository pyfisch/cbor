use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;
use crate::serialize::values::Value;

/// A small natural (23 or less). This will clamp the number.
pub fn usmall(v: u8) -> Value<'static> {
    if v < 24 {
        Value::simple(MajorType::UnsignedInteger(MinorType::SameByte(v)))
    } else {
        Value::simple(MajorType::UnsignedInteger(MinorType::SameByte(23)))
    }
}

pub fn u8(v: u8) -> Value<'static> {
    Value::simple(MajorType::UnsignedInteger(MinorType::OneByte(v)))
}

pub fn u16(v: u16) -> Value<'static> {
    Value::simple(MajorType::UnsignedInteger(MinorType::TwoBytes(v)))
}

pub fn u32(v: u32) -> Value<'static> {
    Value::simple(MajorType::UnsignedInteger(MinorType::FourBytes(v)))
}

pub fn u64(v: u64) -> Value<'static> {
    Value::simple(MajorType::UnsignedInteger(MinorType::EightBytes(v)))
}

pub fn uint(v: u64) -> Value<'static> {
    if v < 24 {
        usmall(v as u8)
    } else if v <= 0xFF {
        u8(v as u8)
    } else if v <= 0xFFFF {
        u16(v as u16)
    } else if v <= 0xFFFF_FFFF {
        u32(v as u32)
    } else {
        u64(v)
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
    if v < 24 {
        Value::simple(MajorType::NegativeInteger(MinorType::SameByte(v)))
    } else {
        Value::simple(MajorType::NegativeInteger(MinorType::SameByte(23)))
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
    Value::simple(MajorType::NegativeInteger(MinorType::OneByte(v)))
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
    Value::simple(MajorType::NegativeInteger(MinorType::TwoBytes(v)))
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
    Value::simple(MajorType::NegativeInteger(MinorType::FourBytes(v)))
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
    Value::simple(MajorType::NegativeInteger(MinorType::EightBytes(v)))
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
    } else if v <= 0xFFFF_FFFF {
        negative_u32(v as u32)
    } else {
        negative_u64(v)
    }
}

/// A small integer of range `[-24, 23]`. This will clamp the number.
pub fn ismall(v: i8) -> Value<'static> {
    if v < 0 {
        negative_usmall(-(v + 1) as u8)
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
        negative_u8(-(v + 1) as u8)
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
        negative_u16(-(v + 1) as u16)
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
        negative_u32(-(v + 1) as u32)
    } else {
        u32(v as u32)
    }
}

/// A 64 bits integer of range `[-2**63 - 1, 2**63]`.
///
/// Note that not all CBOR 4 bytes numbers can be represented with this function. Positive
/// naturals are of range `[0, 2**64]` and negative integer are `[-2**64 - 1, -1]`. To encode
/// those values you need to use either [u32()] for positive numbers, or [negative_u32()]
/// for negative numbers.
pub fn i64(v: i64) -> Value<'static> {
    if v < 0 {
        negative_u64(-(v + 1) as u64)
    } else {
        u64(v as u64)
    }
}

/// An integer that will be represented by the smallest amount of data possible.
///
/// Note that not all CBOR 4 bytes numbers can be represented with this function. Positive
/// naturals are of range `[0, 2**64]` and negative integer are `[-2**32 - 1, -1]`. To encode
/// those values you need to use either [uint()] for positive numbers, or [negative_uint()]
/// for negative numbers.
pub fn int(v: i64) -> Value<'static> {
    if v < 0 {
        negative_uint(-(v + 1) as u64)
    } else {
        uint(v as u64)
    }
}
