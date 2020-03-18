pub mod values;

mod write;
use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;
use crate::serialize::values::Value;
pub use write::{Write, WriteError};

// Define a function to forward a value to the writer.
macro_rules! writer_method (
    ($name: ident) => {
        pub fn $name(&mut self) -> Result<&mut Self, WriteError> {
            values::$name().write_to(self.writer)?;
            Ok(self)
        }
    };
    ($name: ident, $t1: ty) => {
        pub fn $name(&mut self, v1: $t1) -> Result<&mut Self, WriteError> {
            values::$name(v1).write_to(self.writer)?;
            Ok(self)
        }
    };
    ($name: ident, $t1: ty, $t2: ty) => {
        pub fn $name(&mut self, v1: $t1, v2: $t2) -> Result<&mut Self, WriteError> {
            values::$name(v1, v2).write_to(self.writer)?;
            Ok(self)
        }
    };
);

/// A Dumb serializer for CBOR. Takes values and send them automatically to the writer without
/// validation. This means it is possible to create invalid CBOR values from this serializer.
/// For example, you could create a map with half a value pair. Or an indefinite text made
/// of integers (which is illegal CBOR).
///
/// Use this class with caution!
pub struct Serializer<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(writer: &'_ mut W) -> Serializer<'_, W> {
        Serializer { writer }
    }

    writer_method!(usmall, u8);
    writer_method!(u8, u8);
    writer_method!(u16, u16);
    writer_method!(u32, u32);
    writer_method!(u64, u64);
    writer_method!(ismall, i8);
    writer_method!(i8, i8);
    writer_method!(i16, i16);
    writer_method!(i32, i32);
    writer_method!(i64, i64);

    writer_method!(tag, u64, &Value<'a>);

    writer_method!(text, &str);
    writer_method!(bytes, &[u8]);
    writer_method!(array, &[Value<'a>]);
    writer_method!(map, &[(Value<'a>, Value<'a>)]);

    writer_method!(bool, bool);
    writer_method!(r#true);
    writer_method!(r#false);
    writer_method!(null);
    writer_method!(undefined);

    #[cfg(feature = "half")]
    writer_method!(f16, half::f16);
    writer_method!(f32, f32);
    writer_method!(f64, f64);

    /// Write a break for indefinite length bytestring, string, arrays or maps.
    pub fn r#break(&mut self) -> Result<&mut Self, WriteError> {
        MajorType::Break().write_to(self.writer)?;
        Ok(self)
    }

    /// Start an array of length [length]. You will then have to serialize [length] elements,
    /// otherwise an invalid CBOR is produced.
    pub fn raw_array(&mut self, length: usize) -> Result<&mut Self, WriteError> {
        MajorType::Array(MinorType::size(length)).write_to(self.writer)?;
        Ok(self)
    }

    /// Start a map of length [length]. You will then have to serialize [length * 2] elements,
    /// each a key-value pair, otherwise an invalid CBOR is produced.
    pub fn raw_map(&mut self, length: usize) -> Result<&mut Self, WriteError> {
        MajorType::Map(MinorType::size(length)).write_to(self.writer)?;
        Ok(self)
    }

    pub fn raw_tag(&mut self, tag: u64) -> Result<&mut Self, WriteError> {
        MajorType::Tag(MinorType::u64(tag)).write_to(self.writer)?;
        Ok(self)
    }

    pub fn indefinite_bytes(&mut self) -> Result<&mut Self, WriteError> {
        MajorType::ByteString(MinorType::Indefinite()).write_to(self.writer)?;

        Ok(self)
    }

    pub fn indefinite_text(&mut self) -> Result<&mut Self, WriteError> {
        MajorType::Text(MinorType::Indefinite()).write_to(self.writer)?;
        Ok(self)
    }

    pub fn indefinite_array(&mut self) -> Result<&mut Self, WriteError> {
        MajorType::Array(MinorType::Indefinite()).write_to(self.writer)?;
        Ok(self)
    }

    pub fn indefinite_map(&mut self) -> Result<&mut Self, WriteError> {
        MajorType::Map(MinorType::Indefinite()).write_to(self.writer)?;
        Ok(self)
    }
}
