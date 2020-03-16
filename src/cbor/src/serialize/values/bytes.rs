use crate::serialize::values::Value;

pub fn bytes(bytes: &[u8]) -> Value {
    Value::from_byte_string(bytes)
}

pub fn indefinite_bytes<'a>(chunks: &'a [&'a [u8]]) -> Value<'a> {
    Value::from_indefinite_byte_string(chunks)
}
