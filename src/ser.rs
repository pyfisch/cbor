use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser::{self, Serialize, SeqVisitor, MapVisitor};

use super::error::{Error, Result};

pub struct Serializer<W: Write, F=CompactFormatter> {
    writer: W,
    formatter: F,
}

impl <W:Write>Serializer<W> {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer {
            writer: writer,
            formatter: CompactFormatter,
        }
    }
}

impl <W: Write> ser::Serializer for Serializer<W> {
    type Error = Error;
    fn visit_bool(&mut self, v: bool) -> Result<()> {
        self.writer.write_u8(if v {
            0xf5
        } else {
            0xf4
        }).map_err(From::from)
    }
    fn visit_i64(&mut self, v: i64) -> Result<()> {
        if v >= 0 {
            self.visit_u64(v as u64)
        } else {
            self.formatter.visit_type(&mut self.writer, 1, (-v) as u64 - 1)
        }
    }
    fn visit_u64(&mut self, v: u64) -> Result<()> {
        self.formatter.visit_type(&mut self.writer, 0, v)
    }
    fn visit_f64(&mut self, v: f64) -> Result<()> {
        self.writer.write_u8(0xfb)
            .and_then(|()| self.writer.write_f64::<BigEndian>(v)).map_err(From::from)
    }
    fn visit_str(&mut self, value: &str) -> Result<()> {
        self.formatter.visit_type(&mut self.writer, 3, value.len() as u64)
            .and_then(|()| self.writer.write_all(value.as_bytes()).map_err(From::from))
    }
    fn visit_unit(&mut self) -> Result<()> {
        self.writer.write_u8(0xf6).map_err(From::from)
    }
    fn visit_none(&mut self) -> Result<()> {
        self.visit_unit()
    }
    fn visit_some<V>(&mut self, value: V) -> Result<()> where V: Serialize {
        value.serialize(self)
    }
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<()> where V: SeqVisitor {
        if let Some(len) = visitor.len() {
            try!(self.formatter.visit_type(&mut self.writer, 4, len as u64));
            while let Some(()) = try!(visitor.visit(self)) { }
            Ok(())
        } else {
            try!(self.writer.write_u8(0x9f));
            while let Some(()) = try!(visitor.visit(self)) { }
            self.writer.write_u8(0xff).map_err(From::from)
        }
    }
    fn visit_seq_elt<T>(&mut self, value: T) -> Result<()> where T: Serialize {
        value.serialize(self)
    }
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<()> where V: MapVisitor {
        if let Some(len) = visitor.len() {
            try!(self.formatter.visit_type(&mut self.writer, 5, len as u64));
            while let Some(()) = try!(visitor.visit(self)) { }
            Ok(())
        } else {
            try!(self.writer.write_u8(0xbf));
            while let Some(()) = try!(visitor.visit(self)) { }
            self.writer.write_u8(0xff).map_err(From::from)
        }
    }
    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> Result<()> where K: Serialize, V: Serialize {
        key.serialize(self).and_then(|()| value.serialize(self))
    }

    fn format() -> &'static str {
        "cbor"
    }
}

pub trait Formatter<W: Write> {
    fn visit_type(&mut self, writer: &mut W, major_type: u8, v: u64) -> Result<()>;
    fn visit_float(&mut self, writer: &mut W, v: f64) -> Result<()>;
}

pub struct CompactFormatter;

impl <W: Write>Formatter<W> for CompactFormatter {
    fn visit_type(&mut self, writer: &mut W, major_type: u8, v: u64) -> Result<()> {
        compact_type(writer, major_type, v)
    }

    fn visit_float(&mut self, writer: &mut W, v: f64) -> Result<()> {
        compact_float(writer, v)
    }
}

fn compact_type<W: Write>(writer: &mut W, major_type: u8, v: u64) -> Result<()> {
    if v <= 23 {
        writer.write_u8(major_type << 5 | v as u8)
    } else if v <= ::std::u8::MAX as u64 {
        writer.write_u8(major_type << 5 | 0x18)
            .and_then(|()| writer.write_u8(v as u8))
    } else if v <= ::std::u16::MAX as u64 {
        writer.write_u8(major_type << 5 | 0x19)
            .and_then(|()| writer.write_u16::<BigEndian>(v as u16))
    } else if v <= ::std::u32::MAX as u64 {
        writer.write_u8(major_type << 5 | 0x1a)
            .and_then(|()| writer.write_u32::<BigEndian>(v as u32))
    } else {
        writer.write_u8(major_type << 5 | 0x1b)
            .and_then(|()| writer.write_u64::<BigEndian>(v))
    }.map_err(From::from)
}

#[allow(float_cmp)]
fn compact_float<W: Write>(writer: &mut W, v: f64) -> Result<()> {
    // TODO: Encode to f16
    if v.is_infinite() && v.is_sign_positive() {
        writer.write_all(&[0xf9, 0x7c, 0x00]).map_err(From::from)
    } else if v.is_infinite() && v.is_sign_negative() {
        writer.write_all(&[0xf9, 0xfc, 0x00]).map_err(From::from)
    } else if v.is_nan() {
        writer.write_all(&[0xf9, 0x7e, 0x00]).map_err(From::from)
    } else if v as f32 as f64 == v {
        writer.write_f32::<BigEndian>(v as f32).map_err(From::from)
    } else {
        writer.write_f64::<BigEndian>(v).map_err(From::from)
    }
}

#[inline]
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
    where W: Write,
          T: ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser).map_err(From::from)
}

#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>> where T: Serialize {
    let mut writer = Vec::new();
    try!(to_writer(&mut writer, value));
    Ok(writer)
}
