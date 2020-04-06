#![cfg(test)]

use crate::serialize::{Serializer, Write, WriteError};
use crate::test_utils::hex_decode;

// This skips the Write trait and just implement its own vector iterator.
// The Write trait has error handling, and we really don't need that here,
// so this is simpler.
struct Writer {
    pub vector: Vec<u8>,
}

impl Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, WriteError> {
        self.vector.extend_from_slice(bytes);
        Ok(bytes.len())
    }
}

#[test]
fn serialize_text() {
    let mut w = Writer { vector: Vec::new() };
    Serializer::new(&mut w).text("Hello World").unwrap();
    assert_eq!(w.vector, hex_decode("6b48656c6c6f20576f726c64"));
}

#[test]
fn serialize_indefinite_text() {
    let mut w = Writer { vector: Vec::new() };
    Serializer::new(&mut w)
        .indefinite_text()
        .unwrap()
        .text("Hello")
        .unwrap()
        .text(" ")
        .unwrap()
        .text("World")
        .unwrap()
        .r#break()
        .unwrap();

    assert_eq!(w.vector, hex_decode("7f6548656c6c6f612065576f726c64ff"));
}
