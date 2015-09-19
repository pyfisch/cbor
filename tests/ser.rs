extern crate serde;
extern crate serde_cbor;

use std::collections::HashMap;

use serde_cbor::{to_vec, from_slice};

#[test]
fn test_string() {
    let value = "foobar".to_owned();
    assert_eq!(&to_vec(&value).unwrap()[..], b"ffoobar");
}

#[test]
fn test_list() {
    let value = vec![1, 2, 3];
    assert_eq!(&to_vec(&value).unwrap()[..], b"\x83\x01\x02\x03");
}

#[test]
fn test_object() {
    let mut object = HashMap::new();
    object.insert("a".to_owned(), "A".to_owned());
    object.insert("b".to_owned(), "B".to_owned());
    object.insert("c".to_owned(), "C".to_owned());
    object.insert("d".to_owned(), "D".to_owned());
    object.insert("e".to_owned(), "E".to_owned());
    let vec = to_vec(&object).unwrap();
    let test_object = from_slice(&vec[..]).unwrap();
    assert_eq!(object, test_object);
}

#[test]
fn test_float() {
    let vec = to_vec(&12.3f64).unwrap();
    assert_eq!(vec, b"\xfb@(\x99\x99\x99\x99\x99\x9a");
}

#[test]
fn test_infinity() {
    let vec = to_vec(&::std::f64::INFINITY).unwrap();
    assert_eq!(vec, b"\xf9|\x00");
}

#[test]
fn test_i32() {
    let vec = to_vec(&-23567997).unwrap();
    assert_eq!(vec, b"\x3a\x01\x67\x9e\x7c")
}

#[test]
fn test_i8() {
    let vec = to_vec(&-5).unwrap();
    assert_eq!(vec, b"\x24")
}
