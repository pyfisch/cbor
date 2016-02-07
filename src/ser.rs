//! CBOR serializisation.

use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser::{self, Serialize, SeqVisitor, MapVisitor};

use super::error::{Error, Result};

/// A structure for serializing Rust values into CBOR.
pub struct Serializer<W: Write> {
    writer: W,
}

impl<W: Write> Serializer<W> {
    /// Creates a new CBOR serializer.
    #[inline]
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer: writer }
    }

    #[inline]
    fn compact_type(&mut self, major_type: u8, v: u64) -> Result<()> {
        if v <= 23 {
            self.writer.write_u8(major_type << 5 | v as u8)
        } else if v <= ::std::u8::MAX as u64 {
            self.writer
                .write_u8(major_type << 5 | 0x18)
                .and_then(|()| self.writer.write_u8(v as u8))
        } else if v <= ::std::u16::MAX as u64 {
            self.writer
                .write_u8(major_type << 5 | 0x19)
                .and_then(|()| self.writer.write_u16::<BigEndian>(v as u16))
        } else if v <= ::std::u32::MAX as u64 {
            self.writer
                .write_u8(major_type << 5 | 0x1a)
                .and_then(|()| self.writer.write_u32::<BigEndian>(v as u32))
        } else {
            self.writer
                .write_u8(major_type << 5 | 0x1b)
                .and_then(|()| self.writer.write_u64::<BigEndian>(v))
        }
        .map_err(From::from)
    }
}

impl<W: Write> ser::Serializer for Serializer<W> {
    type Error = Error;

    #[inline]
    fn visit_bool(&mut self, v: bool) -> Result<()> {
        self.writer
            .write_u8(if v {
                0xf5
            } else {
                0xf4
            })
            .map_err(From::from)
    }
    #[inline]
    fn visit_i64(&mut self, v: i64) -> Result<()> {
        if v >= 0 {
            self.visit_u64(v as u64)
        } else {
            self.compact_type(1, (-v) as u64 - 1)
        }
    }
    #[inline]
    fn visit_u64(&mut self, v: u64) -> Result<()> {
        self.compact_type(0, v)
    }
    #[inline]
    fn visit_f64(&mut self, v: f64) -> Result<()> {
        // TODO: Encode to f16
        if v.is_infinite() && v.is_sign_positive() {
            self.writer.write_all(&[0xf9, 0x7c, 0x00]).map_err(From::from)
        } else if v.is_infinite() && v.is_sign_negative() {
            self.writer.write_all(&[0xf9, 0xfc, 0x00]).map_err(From::from)
        } else if v.is_nan() {
            self.writer.write_all(&[0xf9, 0x7e, 0x00]).map_err(From::from)
        } else if v as f32 as f64 == v {
            self.writer
                .write_u8(0xfa)
                .and_then(|()| self.writer.write_f32::<BigEndian>(v as f32))
                .map_err(From::from)
        } else {
            self.writer
                .write_u8(0xfb)
                .and_then(|()| self.writer.write_f64::<BigEndian>(v))
                .map_err(From::from)
        }
    }
    #[inline]
    fn visit_str(&mut self, value: &str) -> Result<()> {
        self.compact_type(3, value.len() as u64)
            .and_then(|()| self.writer.write_all(value.as_bytes()).map_err(From::from))
    }
    #[inline]
    fn visit_unit(&mut self) -> Result<()> {
        self.writer.write_u8(0xf6).map_err(From::from)
    }
    #[inline]
    fn visit_none(&mut self) -> Result<()> {
        self.visit_unit()
    }
    #[inline]
    fn visit_some<V>(&mut self, value: V) -> Result<()>
        where V: Serialize
    {
        value.serialize(self)
    }
    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<()>
        where V: SeqVisitor
    {
        if let Some(len) = visitor.len() {
            try!(self.compact_type(4, len as u64));
            while let Some(()) = try!(visitor.visit(self)) {
            }
            Ok(())
        } else {
            try!(self.writer.write_u8(0x9f));
            while let Some(()) = try!(visitor.visit(self)) {
            }
            self.writer.write_u8(0xff).map_err(From::from)
        }
    }
    #[inline]
    fn visit_seq_elt<T>(&mut self, value: T) -> Result<()>
        where T: Serialize
    {
        value.serialize(self)
    }
    #[inline]
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<()>
        where V: MapVisitor
    {
        if let Some(len) = visitor.len() {
            try!(self.compact_type(5, len as u64));
            while let Some(()) = try!(visitor.visit(self)) {
            }
            Ok(())
        } else {
            try!(self.writer.write_u8(0xbf));
            while let Some(()) = try!(visitor.visit(self)) {
            }
            self.writer.write_u8(0xff).map_err(From::from)
        }
    }
    #[inline]
    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> Result<()>
        where K: Serialize,
              V: Serialize
    {
        key.serialize(self).and_then(|()| value.serialize(self))
    }

    #[inline]
    fn format() -> &'static str {
        "cbor"
    }
}

/// Encodes the specified struct into a writer.
#[inline]
pub fn to_writer<W: Write, T: ser::Serialize>(writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser).map_err(From::from)
}

/// Encodes the specified struct into a writer with a leading self-describe tag.
#[inline]
pub fn to_writer_sd<W: Write, T: ser::Serialize>(writer: &mut W, value: &T) -> Result<()> {
    try!(writer.write_all(&[0xd9, 0xd9, 0xf7]));
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser).map_err(From::from)
}

/// Encodes the specified struct into a `Vec<u8>`.
#[inline]
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::new();
    try!(to_writer(&mut writer, value));
    Ok(writer)
}

/// Encodes the specified struct into a `Vec<u8>` with a leading self-describe tag.
#[inline]
pub fn to_vec_sd<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::new();
    try!(to_writer_sd(&mut writer, value));
    Ok(writer)
}
