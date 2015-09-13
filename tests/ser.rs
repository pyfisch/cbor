extern crate serde;
extern crate serde_cbor;

use std::collections::HashMap;

use serde_cbor::{Value, ObjectKey, to_vec, from_slice};

#[test]
fn test_string() {
    let value = Value::String("foobar".to_owned());
    let slice = &[0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72];
    assert_eq!(&to_vec(&value).unwrap()[..], slice);
}

#[test]
fn test_list() {
    let value = Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]);
    let slice = b"\x83\x01\x02\x03";
    assert_eq!(&to_vec(&value).unwrap()[..], slice);
}

#[test]
fn test_object() {
    let mut object = HashMap::new();
    object.insert(ObjectKey::String("a".to_owned()), Value::String("A".to_owned()));
    object.insert(ObjectKey::String("b".to_owned()), Value::String("B".to_owned()));
    object.insert(ObjectKey::String("c".to_owned()), Value::String("C".to_owned()));
    object.insert(ObjectKey::String("d".to_owned()), Value::String("D".to_owned()));
    object.insert(ObjectKey::String("e".to_owned()), Value::String("E".to_owned()));
    let vec = to_vec(&object).unwrap();
    let test_object = from_slice(&vec[..]).unwrap();
    assert_eq!(object, test_object);
}
