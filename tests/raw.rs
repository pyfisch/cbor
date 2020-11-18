#[macro_use]
extern crate serde_derive;

#[cfg(feature = "std")]
mod std_tests {
    use serde_cbor::value::RawValue;

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum Test {
        Known(u32),
        Unknown(Box<RawValue>),
    }

    #[test]
    fn test() {
        let test = Test::Known(1337);
        let known_bytes = serde_cbor::to_vec(&test)
            .expect("serialization failed");

        let test = Test::Unknown(known_bytes.as_slice().into());
        let unknown_bytes = serde_cbor::to_vec(&test)
            .expect("serialization failed");

        assert_eq!(known_bytes, unknown_bytes);
    }
}
