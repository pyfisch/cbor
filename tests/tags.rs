#[cfg(feature = "tags")]
mod tags_tests {
    use serde_bytes;
    use serde_cbor::value::Value;
    use serde_cbor::{from_slice, to_vec};
    use serde_derive::{Deserialize, Serialize};

    fn serialize_and_compare<T: serde::Serialize>(value: T, expected: &[u8]) {
        assert_eq!(to_vec(&value).unwrap(), expected);
    }

    #[test]
    fn test_tags_inline() {
        let value = Value::Tag(1, Box::new(Value::Bool(true)));
        serialize_and_compare(value, &[0xc1, 0xf5]);
    }

    #[test]
    fn test_tags_u8() {
        let value = Value::Tag(50, Box::new(Value::Bool(true)));
        serialize_and_compare(value, &[0xd8, 0x32, 0xf5]);
    }

    #[test]
    fn test_tags_u16() {
        let value = Value::Tag(600, Box::new(Value::Bool(true)));
        serialize_and_compare(value, &[0xd9, 0x02, 0x58, 0xf5]);
    }

    #[test]
    fn test_tags_u32() {
        let value = Value::Tag(70_000, Box::new(Value::Bool(true)));
        serialize_and_compare(value, &[0xda, 0x00, 0x01, 0x11, 0x70, 0xf5]);
    }

    #[test]
    fn test_tags_u64() {
        let value = Value::Tag(8_000_000_000, Box::new(Value::Bool(true)));
        serialize_and_compare(
            value,
            &[0xdb, 0x00, 0x00, 0x00, 0x01, 0xDC, 0xD6, 0x50, 0x00, 0xf5],
        );
    }

    #[test]
    fn test_tags_null() {
        let value = Value::Tag(40, Box::new(Value::Null));
        serialize_and_compare(value, &[0xd8, 0x28, 0xf6]);
    }

    #[test]
    fn test_tags_bool() {
        let value = Value::Tag(40, Box::new(Value::Bool(false)));
        serialize_and_compare(value, &[0xd8, 0x28, 0xf4]);
    }

    #[test]
    fn test_tags_integer() {
        let value = Value::Tag(40, Box::new(Value::Integer(12345)));
        serialize_and_compare(value, &[0xd8, 0x28, 0x19, 0x30, 0x39]);
    }

    #[test]
    fn test_tags_float() {
        let value = Value::Tag(40, Box::new(Value::Float(-5.5)));
        serialize_and_compare(value, &[0xd8, 0x28, 0xF9, 0xC5, 0x80]);
    }

    #[test]
    fn test_tags_bytes() {
        let value = Value::Tag(40, Box::new(Value::Bytes(vec![3, 4, 5])));
        serialize_and_compare(value, &[0xd8, 0x28, 0x43, 0x03, 0x04, 0x05]);
    }

    #[test]
    fn test_tags_text() {
        let value = Value::Tag(40, Box::new(Value::Text("yay".to_string())));
        serialize_and_compare(value, &[0xd8, 0x28, 0x63, 0x79, 0x61, 0x79]);
    }

    #[test]
    fn test_tags_array() {
        let value = Value::Tag(
            40,
            Box::new(Value::Array(vec![Value::Bool(true), Value::Integer(7)])),
        );
        serialize_and_compare(value, &[0xd8, 0x28, 0x82, 0xf5, 0x07]);
    }

    #[test]
    fn test_tags_map() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);

        let value = Value::Tag(40, Box::new(serde_cbor::value::to_value(map).unwrap()));
        serialize_and_compare(
            value,
            &[
                0xd8, 0x28, 0xa2, 0x63, 0x62, 0x61, 0x72, 0x02, 0x63, 0x66, 0x6f, 0x6f, 0x01,
            ],
        );
    }

    #[test]
    fn test_tags_tag() {
        let value = Value::Tag(40, Box::new(Value::Tag(54321, Box::new(Value::Null))));
        serialize_and_compare(value, &[0xd8, 0x28, 0xd9, 0xd4, 0x31, 0xf6]);
    }

    #[test]
    fn test_tags_derive_struct() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "_TagStruct")]
        struct MyType((u64, Value));

        let value = MyType((42, Value::Bytes(vec![1, 2, 3])));
        serialize_and_compare(value, &[0xd8, 0x2a, 0x43, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_tag_decode() {
        let tag_encoded = [0xd8, 0x2a, 0x43, 0x01, 0x02, 0x03];
        let tag_decoded = serde_cbor::de::from_slice::<Value>(&tag_encoded).unwrap();
        assert_eq!(
            tag_decoded,
            Value::Tag(42, Box::new(Value::Bytes(vec![1, 2, 3])))
        );
    }

    #[test]
    fn test_tags_roundtrip() {
        let tag_value = Value::Tag(42, Box::new(Value::Bytes(vec![1, 2, 3])));
        let tag_encoded = to_vec(&tag_value).unwrap();
        assert_eq!(tag_encoded, [0xd8, 0x2a, 0x43, 0x01, 0x02, 0x03]);

        let tag_decoded = from_slice::<Value>(&tag_encoded).unwrap();
        assert_eq!(tag_decoded, tag_value);
    }

    #[test]
    fn test_tags_custom_type() {
        #[derive(Debug, PartialEq)]
        struct Cid(Vec<u8>);

        impl serde::Serialize for Cid {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                let tag = 42u64;
                let value = serde_bytes::ByteBuf::from(&self.0[..]);
                s.serialize_newtype_struct(serde_cbor::CBOR_TAG_STRUCT_NAME, &(tag, value))
            }
        }

        struct CidVisitor;

        impl<'de> serde::de::Visitor<'de> for CidVisitor {
            type Value = Cid;

            fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "a sequence of tag and value")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple(2, self)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // First element of the tuple is the tag
                let tag: u64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                // Second element of the tuple is the value
                let value: Value = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                match (tag, value) {
                    // Only return the value if tag and value type match
                    (42, Value::Bytes(bytes)) => Ok(Cid(bytes)),
                    _ => {
                        let error = format!("tag: {:?}", tag);
                        let unexpected = serde::de::Unexpected::Other(&error);
                        Err(serde::de::Error::invalid_value(unexpected, &self))
                    }
                }
            }
        }

        impl<'de> serde::de::Deserialize<'de> for Cid {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let visitor = CidVisitor;
                deserializer.deserialize_newtype_struct(serde_cbor::CBOR_TAG_STRUCT_NAME, visitor)
            }
        }

        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct MyStruct {
            cid: Cid,
            data: bool,
        }

        // Tests with just the custom type

        let cid = Cid(vec![1, 2, 3]);
        let cid_encoded = to_vec(&cid).unwrap();
        assert_eq!(cid_encoded, [0xd8, 0x2a, 0x43, 0x01, 0x02, 0x03]);

        let cid_decoded_as_cid: Cid = from_slice(&cid_encoded).unwrap();
        assert_eq!(cid_decoded_as_cid, cid);

        let cid_decoded_as_value: Value = from_slice(&cid_encoded).unwrap();
        assert_eq!(
            cid_decoded_as_value,
            Value::Tag(42, Box::new(Value::Bytes(vec![1, 2, 3])))
        );

        // Tests with the Type nested in a struct

        let mystruct = MyStruct { cid, data: true };
        let mystruct_encoded = to_vec(&mystruct).unwrap();
        assert_eq!(
            mystruct_encoded,
            [
                0xa2, 0x63, 0x63, 0x69, 0x64, 0xd8, 0x2a, 0x43, 0x1, 0x2, 0x3, 0x64, 0x64, 0x61,
                0x74, 0x61, 0xf5
            ]
        );

        let mystruct_decoded_as_mystruct: MyStruct = from_slice(&mystruct_encoded).unwrap();
        assert_eq!(mystruct_decoded_as_mystruct, mystruct);

        let mystruct_decoded_as_value: Value = from_slice(&mystruct_encoded).unwrap();
        let mut expected_map = std::collections::BTreeMap::new();
        expected_map.insert(
            Value::Text("cid".to_string()),
            Value::Tag(42, Box::new(Value::Bytes(vec![1, 2, 3]))),
        );
        expected_map.insert(Value::Text("data".to_string()), Value::Bool(true));
        assert_eq!(mystruct_decoded_as_value, Value::Map(expected_map));
    }
}
