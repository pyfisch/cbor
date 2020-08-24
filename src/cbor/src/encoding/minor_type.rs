#![allow(clippy::many_single_char_names)]
use crate::serialize::{Write, WriteError, WriteTo};

/// A minor type as a value holder, which is the values determining additional information.
/// For example, for an Array, this would be
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MinorType {
    /// A small integer that fits in the minor type, 0 to 23.
    SameByte(u8),

    /// An integer that takes up 1 byte after the major-type byte.
    OneByte(u8),

    /// An integer that takes up to 2 bytes after major-type byte.
    TwoBytes(u16),

    /// An integer that takes up to 4 bytes after major-type byte.
    FourBytes(u32),

    /// An integer that takes up to 8 bytes after major-type byte.
    EightBytes(u64),

    /// Indication for certain Major Types that the size is unbounded and
    /// the encoder/decoder should use a "break" token.
    Indefinite(),

    /// A value of 28-30 is reserved and should never be used. Deserializing
    /// will likely lead to errors, and serializing will fail.
    Reserved(u8),
}

impl MinorType {
    /// Encode the size of a text, array or map.
    pub fn size(v: usize) -> Self {
        if v <= 23 {
            MinorType::SameByte(v as u8)
        } else if v <= 0xFF {
            MinorType::OneByte(v as u8)
        } else if v <= 0xFFFF {
            MinorType::TwoBytes(v as u16)
        } else if v <= 0xFFFFFFFF {
            MinorType::FourBytes(v as u32)
        } else {
            MinorType::EightBytes(v as u64)
        }
    }

    /// Returns the minor type of these bytes.
    pub fn minor(&self) -> u8 {
        match self {
            MinorType::SameByte(x) => *x,
            MinorType::OneByte(_) => 24,
            MinorType::TwoBytes(_) => 25,
            MinorType::FourBytes(_) => 26,
            MinorType::EightBytes(_) => 27,
            MinorType::Indefinite() => 31,
            MinorType::Reserved(x) => *x,
        }
    }
}

impl WriteTo for MinorType {
    fn len(&self) -> usize {
        match self {
            MinorType::SameByte(_) => 0,
            MinorType::OneByte(_) => 1,
            MinorType::TwoBytes(_) => 2,
            MinorType::FourBytes(_) => 4,
            MinorType::EightBytes(_) => 8,
            MinorType::Indefinite() => 0,
            MinorType::Reserved(_) => 0,
        }
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, WriteError> {
        match self {
            MinorType::SameByte(_) => Ok(0),
            MinorType::OneByte(v) => w.write(&v.to_be_bytes()),
            MinorType::TwoBytes(v) => w.write(&v.to_be_bytes()),
            MinorType::FourBytes(v) => w.write(&v.to_be_bytes()),
            MinorType::EightBytes(v) => w.write(&v.to_be_bytes()),
            MinorType::Indefinite() => Ok(0),
            MinorType::Reserved(_) => Err(WriteError::ReservedCborValue()),
        }
    }
}

impl From<&[u8]> for MinorType {
    fn from(bytes: &[u8]) -> Self {
        let minor = bytes[0] & 0x1F;
        match minor {
            24 => MinorType::OneByte(bytes[1] as u8),
            25 => MinorType::TwoBytes(((bytes[1] as u16) << 8) as u16 + (bytes[2] as u16) as u16),
            26 => MinorType::FourBytes(
                ((bytes[1] as u32) << 24) as u32
                    + ((bytes[2] as u32) << 16) as u32
                    + ((bytes[3] as u32) << 8) as u32
                    + (bytes[4] as u32) as u32,
            ),
            27 => MinorType::EightBytes(
                ((bytes[1] as u64) << 56)
                    + ((bytes[2] as u64) << 48) as u64
                    + ((bytes[3] as u64) << 40) as u64
                    + ((bytes[4] as u64) << 32) as u64
                    + ((bytes[5] as u64) << 24) as u64
                    + ((bytes[6] as u64) << 16) as u64
                    + ((bytes[7] as u64) << 8) as u64
                    + (bytes[8] as u64) as u64,
            ),
            31 => MinorType::Indefinite(),
            x => {
                if x < 24 {
                    MinorType::SameByte(x)
                } else {
                    MinorType::Reserved(x)
                }
            }
        }
    }
}
