fn main() {
    #[cfg(feature = "tags")]
    tags_example::main();
    #[cfg(not(feature = "tags"))]
    println!("Run this example with the `--feature tags` flag.");
}

#[cfg(feature = "tags")]
mod tags_example {
    use serde::de::{self, Unexpected};
    use serde::ser;
    use serde_derive::{Deserialize, Serialize};

    use serde_bytes;

    use serde_cbor::value::Value;
    use serde_cbor::{from_slice, to_vec};

    use std::fmt;

    #[derive(Debug, PartialEq)]
    struct Cid(Vec<u8>);

    impl ser::Serialize for Cid {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            let tag = 42u64;
            let value = serde_bytes::ByteBuf::from(&self.0[..]);
            s.serialize_newtype_struct(serde_cbor::CBOR_TAG_STRUCT_NAME, &(tag, value))
        }
    }

    struct CidVisitor;

    impl<'de> de::Visitor<'de> for CidVisitor {
        type Value = Cid;

        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "a sequence of tag and value")
        }

        fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_tuple(2, self)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let tag: u64 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let value: Value = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;

            match (tag, value) {
                // Only return the value if tag and value type match
                (42, Value::Bytes(bytes)) => Ok(Cid(bytes)),
                _ => {
                    let error = format!("tag: {:?}", tag);
                    let unexpected = Unexpected::Other(&error);
                    Err(de::Error::invalid_value(unexpected, &self))
                }
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Cid {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
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

    pub fn main() {
        // Serialize any CBOR tag you like, the tag identifier is an u64 and the value is any of
        // the CBOR values available.
        let tag = Value::Tag(123, Box::new(Value::Text("some value".to_string())));
        println!("Tag: {:?}", tag);
        let tag_encoded = to_vec(&tag).unwrap();
        println!("Encoded tag: {:x?}", tag_encoded);

        // You can also have your own custom tags implemented, that don't even use the CBOR `Value`
        // type. In this example we encode a vector of integers as byte string with tag 42.
        let cid = Cid(vec![1, 2, 3]);
        println!("CID: {:?}", cid);
        let cid_encoded = to_vec(&cid).unwrap();
        println!("Encoded CID: {:x?}", cid_encoded);

        // You can either decode it again as your custom object...
        let cid_decoded_as_cid: Cid = from_slice(&cid_encoded).unwrap();
        println!("Decoded CID as CID: {:?}", cid_decoded_as_cid);
        // ...or as a generic CBOR Value, which will then transform it into a `Tag()`.
        let cid_decoded_as_value: Value = from_slice(&cid_encoded).unwrap();
        println!("Decoded CID as Value: {:?}", cid_decoded_as_value);

        // Your custom object also works if it is nested in a truct
        let mystruct = MyStruct { cid, data: true };
        println!("Custom struct: {:?}", mystruct);
        let mystruct_encoded = to_vec(&mystruct).unwrap();
        println!("Encoded custom struct: {:?}", mystruct_encoded);
        let mystruct_decoded_as_mystruct: MyStruct = from_slice(&mystruct_encoded).unwrap();
        println!("Decoded custom struct: {:?}", mystruct_decoded_as_mystruct);
        let mystruct_decoded_as_value: Value = from_slice(&mystruct_encoded).unwrap();
        println!(
            "Decoded custom struct as Value: {:?}",
            mystruct_decoded_as_value
        );
    }
}
