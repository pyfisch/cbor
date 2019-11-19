use serde::de::{Deserializer, Error};
use serde::ser::{Serialize, Serializer};

/// signals that a newtype is from a CBOR tag
pub const CBOR_NEWTYPE_NAME: &str = "__cbor_tag";

/// extensions for all serde serializers
pub trait SerializerExt: Serializer {
    /// basically serialize_newtype_struct with a cbor tag value
    fn serialize_cbor_tagged<T: Serialize>(
        self,
        tag: u64,
        value: &T,
    ) -> core::result::Result<Self::Ok, Self::Error> {
        tag_access::set_tag(Some(tag));
        let r = self.serialize_newtype_struct(CBOR_NEWTYPE_NAME, value);
        tag_access::set_tag(None);
        r
    }
}

impl<S: Serializer> SerializerExt for S {}

/// extensions for all serde deserializers
pub trait DeserializerExt<'de>: Deserializer<'de> {
    /// expect the given cbor tag
    fn expect_cbor_tag(&self, tag: u64) -> Result<(), Self::Error> {
        match tag_access::get_tag() {
            Some(t) if t == tag => Ok(()),
            Some(_) => Err(Self::Error::custom("unexpected cbor tag")),
            None => Err(Self::Error::custom("missing cbor tag")),
        }
    }
}

impl<'de, D: Deserializer<'de>> DeserializerExt<'de> for D {}

#[cfg(tags)]
pub mod tag_access {
    use std::cell::RefCell;
    thread_local!(static CBOR_TAG: RefCell<Option<u64>> = RefCell::new(None));

    pub fn set_tag(value: Option<u64>) {
        CBOR_TAG.with(|f| {
            *f.borrow_mut() = value;
        });
    }

    pub fn get_tag() -> Option<u64> {
        CBOR_TAG.with(|f| *f.borrow())
    }

    #[cfg(test)]
    mod tests {
        use crate::*;

        fn decode_hex(s: &str) -> std::result::Result<Vec<u8>, std::num::ParseIntError> {
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
                .collect()
        }

        // get bytes from http://cbor.me/ trees
        fn get_bytes(example: &str) -> std::result::Result<Vec<u8>, std::num::ParseIntError> {
            let hex = example
                .split("\n")
                .flat_map(|line| line.split("#").take(1))
                .collect::<Vec<&str>>()
                .join("")
                .replace(" ", "");
            decode_hex(&hex)
        }

        #[test]
        fn tagged_cbor_roundtrip() {
            let data = r#"
C1                   # tag(1)
   82                # array(2)
      C2             # tag(2)
         63          # text(3)
            666F6F   # "foo"
      C3             # tag(3)
         A1          # map(1)
            C4       # tag(4)
               61    # text(1)
                  61 # "a"
            C5       # tag(5)
               61    # text(1)
                  62 # "b"
            "#;
            let bytes1 = get_bytes(&data).unwrap();
            let value1: Value = from_slice(&bytes1).unwrap();
            let bytes2 = to_vec(&value1).unwrap();
            let value2: Value = from_slice(&bytes2).unwrap();
            assert_eq!(bytes1, bytes2);
            assert_eq!(value1, value2);
            // println!("{:?}\n{:?}\n{:?}\n{:?}\n", bytes1, value1, bytes2, value2);
        }
    }
}

#[cfg(not(tags))]
pub mod tag_access {

    pub fn set_tag(_: Option<u64>) {}

    pub fn get_tag() -> Option<u64> {
        None
    }
}
