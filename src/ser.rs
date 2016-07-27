//! CBOR serializisation.

use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser::{self, Serialize};

use {Error, Result};

/// A structure for serializing Rust values into CBOR.
pub struct Serializer<W: Write> {
    writer: W,
    packed: bool,
}

impl<W: Write> Serializer<W> {
    /// Creates a new CBOR serializer.
    pub fn new(writer: W) -> Serializer<W> {
        Serializer {
            writer: writer,
            packed: false,
        }
    }
    
    /// Creates a new packed CBOR serializer.
    ///
    /// In packed mode all struct field names and enum variant names
    /// are not serialized instead the index position of the field
    /// is used.
    pub fn packed(writer: W) -> Serializer<W> {
        Serializer {
            writer: writer,
            packed: true
        }
    }
    
    /// Writes the CBOR self-describe tag to the stream.
    ///
    /// Tagging allows a decoder to distinguish different file formats
    /// based on their content without other information.
    pub fn self_describe(&mut self) -> Result<()> {
        try!(self.writer.write_u8(6 << 5 | 25));
        try!(self.writer.write_u16::<BigEndian>(55799));
        Ok(())
    }
    
    #[inline]
    fn write_type_u8(&mut self, major: u8, additional: u8) -> Result<()> {
        if additional > 23 {
            try!(self.writer.write_u8(major << 5 | 24));
            try!(self.writer.write_u8(additional));
        } else {
            try!(self.writer.write_u8(major << 5 | additional));
        }
        Ok(())
    }
    
    #[inline]
    fn write_type_u16(&mut self, major: u8, additional: u16) -> Result<()> {
        if additional > ::std::u8::MAX as u16 {
            try!(self.writer.write_u8(major << 5 | 25));
            try!(self.writer.write_u16::<BigEndian>(additional));
        } else {
            try!(self.write_type_u8(major, additional as u8));
        }
        Ok(())
    }
    
    #[inline]
    fn write_type_u32(&mut self, major: u8, additional: u32) -> Result<()> {
        if additional > ::std::u16::MAX as u32 {
            try!(self.writer.write_u8(major << 5 | 26));
            try!(self.writer.write_u32::<BigEndian>(additional));
        } else {
            try!(self.write_type_u16(major, additional as u16));
        }
        Ok(())
    }

    #[inline]
    fn write_type_u64(&mut self, major: u8, additional: u64) -> Result<()> {
        if additional > ::std::u32::MAX as u64 {
            try!(self.writer.write_u8(major << 5 | 27));
            try!(self.writer.write_u64::<BigEndian>(additional));
        } else {
            try!(self.write_type_u32(major, additional as u32));
        }
        Ok(())
    }
    
    #[inline]
    fn write_collection_start(&mut self, major: u8, len: Option<usize>) -> Result<CollectionState> {
        if let Some(len) = len {
            try!(self.write_type_u64(major, len as u64));
            Ok(CollectionState::Fixed)
        } else {
            try!(self.writer.write_u8(major << 5 | 31));
            Ok(CollectionState::Indefinite)
        }
    }

    #[inline]
    fn write_collection_end(&mut self, state: CollectionState) -> Result<()> {
        match state {
            CollectionState::Fixed => Ok(()),
            CollectionState::Indefinite => self.writer.write_u8(0xff).map_err(From::from),
        }
    }
}

/// A collection like an array or map may have a fixed or indefinite length.
pub enum CollectionState {
    /// Fixed collections end after all elements were serialized.
    Fixed,
    /// Indefinite collections are terminated by a stop code after the last element.
    Indefinite,
}

/// Structs are record types. They are serialized as key value pairs.
///
/// The keys are either strings or indexes denoting the position in the struct.
/// In `packed` serialization indexes are used to preserve space.
///
/// To keep track of the current index a counter is stored in this state.
pub struct StructState {
    counter: usize
}

impl<W: Write> ser::Serializer for Serializer<W> {
    type Error = Error;
    type SeqState = CollectionState;
    type TupleState = ();
    type TupleStructState = ();
    type TupleVariantState = ();
    type MapState = CollectionState;
    type StructState = StructState;
    type StructVariantState = StructState;
    
    #[inline]
    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.writer.write_u8(
            match v {
                false => 7 << 5 | 20,
                true => 7 << 5 | 21,
            }).map_err(From::from)
    }
    
    #[inline]
    fn serialize_isize(&mut self, v: isize) -> Result<()> {
        self.serialize_i64(v as i64)
    }
    
    #[inline]
    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        if v < 0 {
            self.write_type_u8(1, -(v + 1) as u8)
        } else {
            self.serialize_u8(v as u8)
        }
    }
    
    #[inline]
    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        if v < 0 {
            self.write_type_u16(1, -(v + 1) as u16)
        } else {
            self.serialize_u16(v as u16)
        }
    }
    
    #[inline]
    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        if v < 0 {
            self.write_type_u32(1, -(v + 1) as u32)
        } else {
            self.serialize_u32(v as u32)
        }
    }
    
    #[inline]
    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        if v < 0 {
            self.write_type_u64(1, -(v + 1) as u64)
        } else {
            self.serialize_u64(v as u64)
        }
    }
    
    #[inline]
    fn serialize_usize(&mut self, v: usize) -> Result<()> {
        self.serialize_u64(v as u64)
    }
    
    #[inline]
    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.write_type_u8(0, v)
    }
    
    #[inline]
    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.write_type_u16(0, v)
    }
    
    #[inline]
    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.write_type_u32(0, v)
    }
    
    #[inline]
    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.write_type_u64(0, v)
    }
    
    #[inline]
    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        // TODO: Encode to f16
        if v.is_infinite() && v.is_sign_positive() {
            self.writer.write_all(&[0xf9, 0x7c, 0x00])
        } else if v.is_infinite() && v.is_sign_negative() {
            self.writer.write_all(&[0xf9, 0xfc, 0x00])
        } else if v.is_nan() {
            self.writer.write_all(&[0xf9, 0x7e, 0x00])
        } else {
            self.writer
                .write_u8(7 << 5 | 26)
                .and_then(|()| self.writer.write_f32::<BigEndian>(v))
        }.map_err(From::from)
    }
    
    #[inline]
    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        // TODO: Encode to f16
        if v.is_infinite() && v.is_sign_positive() {
            self.writer.write_all(&[0xf9, 0x7c, 0x00])
        } else if v.is_infinite() && v.is_sign_negative() {
            self.writer.write_all(&[0xf9, 0xfc, 0x00])
        } else if v.is_nan() {
            self.writer.write_all(&[0xf9, 0x7e, 0x00])
        } else if v as f32 as f64 == v {
            self.writer
                .write_u8(7 << 5 | 26)
                .and_then(|()| self.writer.write_f32::<BigEndian>(v as f32))
        } else {
            self.writer
                .write_u8(7 << 5 | 27)
                .and_then(|()| self.writer.write_f64::<BigEndian>(v))
        }.map_err(From::from)
    }
    
    #[inline]
    fn serialize_char(&mut self, v: char) -> Result<()> {
        // TODO: Avoid allocation. rust-lang/rust#27784
        let mut s = String::new();
        s.push(v);
        self.serialize_str(s.as_str())
    }
    
    #[inline]
    fn serialize_str(&mut self, value: &str) -> Result<()> {
        try!(self.write_type_u64(3, value.len() as u64));
        self.writer.write_all(value.as_bytes()).map_err(From::from)
    }
    
    #[inline]
    fn serialize_bytes(&mut self, value: &[u8]) -> Result<()> {
        try!(self.write_type_u64(2, value.len() as u64));
        self.writer.write_all(value).map_err(From::from)
    }
    
    #[inline]
    fn serialize_unit(&mut self) -> Result<()> {
        self.writer.write_u8(7 << 5 | 22).map_err(From::from)
    }
    
    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }
    
    #[inline]
    fn serialize_unit_variant(&mut self,
                              _name: &'static str,
                              variant_index: usize,
                              variant: &'static str) -> Result<()> {
        if !self.packed {
            self.serialize_str(variant)
        } else {
            self.serialize_usize(variant_index)
        }
    }
    
    #[inline]
    fn serialize_newtype_struct<T: Serialize>(&mut self, 
                                              _name: &'static str,
                                              value: T) -> Result<()> {
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_newtype_variant<T: Serialize>(&mut self,
                                               name: &'static str,
                                               variant_index: usize,
                                               variant: &'static str,
                                               value: T) -> Result<()> {
        try!(self.writer.write_u8(4 << 5 | 2));
        try!(self.serialize_unit_variant(name, variant_index, variant));
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        self.serialize_unit()
    }
    
    #[inline]
    fn serialize_some<T: Serialize>(&mut self, value: T) -> Result<()> {
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<CollectionState> {
        self.write_collection_start(4, len)
    }
    
    #[inline]
    fn serialize_seq_elt<T: Serialize>(&mut self,
                                       _state: &mut Self::SeqState,
                                       value: T) -> Result<()> {
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_seq_end(&mut self, state: CollectionState) -> Result<()> {
        self.write_collection_end(state)
    }
    
    #[inline]
    fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<CollectionState> {
        self.serialize_seq(Some(size))
    }
    
    #[inline]
    fn serialize_tuple(&mut self, len: usize) -> Result<()> {
        self.write_type_u64(4, len as u64)
    }

    #[inline]
    fn serialize_tuple_elt<T: Serialize>(&mut self,
                                         _state: &mut (),
                                         value: T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, _state: ()) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_tuple_struct(&mut self, _name: &'static str, len: usize) -> Result<()> {
        self.serialize_tuple(len)
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: Serialize>(&mut self,
                                         _state: &mut (),
                                         value: T) -> Result<()> {
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_tuple_struct_end(&mut self, _state: ()) -> Result<()> {
        Ok(())
    }
    
    #[inline]
    fn serialize_tuple_variant(&mut self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize) -> Result<()> {
        try!(self.write_type_u64(4, (len + 1) as u64));
        self.serialize_unit_variant(name, variant_index, variant)
    }
    
    #[inline]
    fn serialize_tuple_variant_elt<T: Serialize>(&mut self, _state: &mut (), value: T) -> Result<()> {
        value.serialize(self)
    }
    
    #[inline]
    fn serialize_tuple_variant_end(&mut self, _state: ()) -> Result<()> {
        Ok(())
    }
    
    #[inline]
    fn serialize_map(&mut self, len: Option<usize>) -> Result<Self::MapState> {
        self.write_collection_start(5, len)
    }
    
    #[inline]
    fn serialize_map_elt<K: Serialize, V: Serialize>(&mut self,
                                                     _state: &mut CollectionState,
                                                     key: K,
                                                     value: V) -> Result<()> {
        key.serialize(self).and_then(|()| value.serialize(self))
    }
    
    #[inline]
    fn serialize_map_end(&mut self, state: CollectionState) -> Result<()> {
        self.write_collection_end(state)
    }
    
    #[inline]
    fn serialize_struct(&mut self, _name: &'static str, len: usize) -> Result<StructState> {
        try!(self.write_type_u64(5, len as u64));
        Ok(StructState {
            counter: 0
        })
    }
    
    #[inline]
    fn serialize_struct_elt<V: Serialize>(&mut self,
                                          state: &mut StructState,
                                          key: &'static str,
                                          value: V) -> Result<()> {
        if !self.packed {
            try!(self.serialize_str(key));
        } else {
            try!(self.serialize_usize(state.counter));
        }
        try!(value.serialize(self));
        state.counter += 1;
        Ok(())
    }
    
    #[inline]
    fn serialize_struct_end(&mut self, _state: StructState) -> Result<()> {
        Ok(())
    }
    
    #[inline]
    fn serialize_struct_variant(&mut self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize) -> Result<StructState> {
        try!(self.writer.write_u8(4 << 5 | 2));
        try!(self.serialize_unit_variant(name, variant_index, variant));
        try!(self.write_type_u64(5, len as u64));
        Ok(StructState {
            counter: 0
        })
    }

    #[inline]
    fn serialize_struct_variant_elt<V: Serialize>(&mut self,
                                                  state: &mut StructState,
                                                  key: &'static str,
                                                  value: V) -> Result<()> {
        if !self.packed {
            try!(self.serialize_str(key));
        } else {
            try!(self.serialize_usize(state.counter));
        }
        try!(value.serialize(self));
        state.counter += 1;
        Ok(())
    }
    
    #[inline]
    fn serialize_struct_variant_end(&mut self, _state: StructState) -> Result<()> {
        Ok(())
    }
}

/// Serializes a value to a writer.
pub fn to_writer<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    value.serialize(&mut Serializer::new(&mut writer))
}

/// Serializes a value to a writer and add a CBOR self-describe tag.
pub fn to_writer_sd<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::new(&mut writer);
    try!(ser.self_describe());
    value.serialize(&mut ser)
}

/// Serializes a value without names to a writer.
pub fn to_writer_packed<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    value.serialize(&mut Serializer::packed(&mut writer))
}

/// Serializes a value without names to a writer and add a CBOR self-describe tag.
pub fn to_writer_packed_sd<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::packed(&mut writer);
    try!(ser.self_describe());
    value.serialize(&mut ser)
}

/// Serializes a value to a vector.
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    try!(to_writer(&mut vec, value));
    Ok(vec)
}

/// Serializes a value to a vector and add a CBOR self-describe tag.
pub fn to_vec_sd<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    try!(to_writer_sd(&mut vec, value));
    Ok(vec)
}

/// Serializes a value without names to a vector.
pub fn to_vec_packed<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    try!(to_writer_packed(&mut vec, value));
    Ok(vec)
}
/// Serializes a value without names to a vector and add a CBOR self-describe tag.
pub fn to_vec_packed_sd<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    try!(to_writer_packed_sd(&mut vec, value));
    Ok(vec)
}
