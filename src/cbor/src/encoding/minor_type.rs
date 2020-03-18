#![allow(clippy::many_single_char_names)]
use crate::serialize::{Write, WriteError};

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
}

impl MinorType {
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

    pub fn u64(v: u64) -> Self {
        Self::size(v as usize)
    }

    /// The amount of bytes taken by this minor type, or 0 if it is encoded in the MajorType.
    pub fn len(&self) -> usize {
        match self {
            MinorType::SameByte(_) => 0,
            MinorType::OneByte(_) => 1,
            MinorType::TwoBytes(_) => 2,
            MinorType::FourBytes(_) => 4,
            MinorType::EightBytes(_) => 8,
            MinorType::Indefinite() => 0,
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
        }
    }

    pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match self {
            MinorType::SameByte(_) => Ok(()),
            MinorType::OneByte(v) => w.write(&[*v]),
            MinorType::TwoBytes(v) => w.write(&[(*v >> 8) as u8, *v as u8]),
            MinorType::FourBytes(v) => w.write(&[
                (*v >> 24) as u8,
                (*v >> 16) as u8,
                (*v >> 8) as u8,
                (*v) as u8,
            ]),
            MinorType::EightBytes(v) => w.write(&[
                (*v >> 56) as u8,
                (*v >> 48) as u8,
                (*v >> 40) as u8,
                (*v >> 32) as u8,
                (*v >> 24) as u8,
                (*v >> 16) as u8,
                (*v >> 8) as u8,
                *v as u8,
            ]),
            MinorType::Indefinite() => Ok(()),
        }
    }
}
