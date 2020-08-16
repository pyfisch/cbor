use crate::serialize::owned::OwnedValue;

pub fn bytes(bytes: &[u8]) -> OwnedValue {
    OwnedValue::from_byte_string(bytes)
}

pub fn indefinite_bytes(chunks: &[&[u8]]) -> OwnedValue {
    OwnedValue::from_indefinite_byte_string(chunks)
}
