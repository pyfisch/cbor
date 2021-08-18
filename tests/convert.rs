//! This set of tests confirms that the `From` conversions and `to_value()`
//! conversions produce the same output.

#![cfg(feature = "std")]

use serde_cbor::{value::to_value, Value};
use std::collections::BTreeMap;

#[test]
fn bool() {
    assert_eq!(Value::from(true), to_value(true).unwrap());
    assert_eq!(Value::from(false), to_value(false).unwrap());
}

#[test]
fn integer() {
    assert_eq!(Value::from(127u8), to_value(127u8).unwrap());
    assert_eq!(Value::from(127u16), to_value(127u16).unwrap());
    assert_eq!(Value::from(127u32), to_value(127u32).unwrap());
    assert_eq!(Value::from(127u64), to_value(127u64).unwrap());
    assert_eq!(Value::from(127i8), to_value(127i8).unwrap());
    assert_eq!(Value::from(127i16), to_value(127i16).unwrap());
    assert_eq!(Value::from(127i32), to_value(127i32).unwrap());
    assert_eq!(Value::from(127i64), to_value(127i64).unwrap());
}

#[test]
fn float() {
    assert_eq!(Value::from(7.8f32), to_value(7.8f32).unwrap());
    assert_eq!(Value::from(7.8f64), to_value(7.8f64).unwrap());
}

#[test]
fn bytes() {
    let bytes = vec![0u8, 1u8, 2u8];

    assert_eq!(Value::from(bytes.clone()), to_value(bytes).unwrap());
}

#[test]
fn string() {
    let string = "foo".to_owned();

    assert_eq!(Value::from(string.clone()), to_value(string).unwrap());
}

#[test]
fn array() {
    let array: Vec<Value> = vec![1.into(), 2.into(), 3.into()];

    assert_eq!(Value::from(array.clone()), to_value(array).unwrap());
}

#[test]
fn map() {
    let mut map = BTreeMap::new();

    map.insert(Value::from(1), Value::from(2));

    assert_eq!(Value::from(map.clone()), to_value(map).unwrap());
}
