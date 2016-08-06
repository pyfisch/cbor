// Do not try to use features on stable.
#![cfg_attr(feature="unstable", feature(custom_derive, plugin))]
// Do not try to load the plugin if run on stable.
#![cfg_attr(feature="unstable", plugin(serde_macros))]

extern crate serde;
extern crate serde_cbor;

use std::u8;

use serde::bytes::ByteBuf;

use serde_cbor::{to_vec, from_slice};

fn to_binary(s: &'static str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    let mut b = Vec::with_capacity(s.len() / 2);
    for i in 0..s.len() / 2 {
        b.push(u8::from_str_radix(&s[i * 2..(i + 1) * 2], 16).unwrap());
    }
    b
}

macro_rules! testcase {
    ($name:ident, f64, $expr:expr, $s:expr) => {
        #[test]
        fn $name() {
            let expr: f64 = $expr;
            let serialized = to_binary($s);
            assert_eq!(to_vec(&expr).unwrap(), serialized);
            let parsed: f64 = from_slice(&serialized[..]).unwrap();
            if !expr.is_nan() {
                assert_eq!(expr, parsed);
            } else {
                assert!(parsed.is_nan())
            }
        }
    };
    ($name:ident, $ty:ty, $expr:expr, $s:expr) => {
        #[test]
        fn $name() {
            let expr: $ty = $expr;
            let serialized = to_binary($s);
            println!("Testing serialization");
            assert_eq!(to_vec(&expr).unwrap(), serialized);
            let parsed: $ty = from_slice(&serialized[..]).unwrap();
            println!("Testing deserialization");
            assert_eq!(parsed, expr);
        }
    }
}
            

testcase!(test_bool_false, bool, false, "f4");
testcase!(test_bool_true, bool, true, "f5");
testcase!(test_isize_neg_256, isize, -256, "38ff");
testcase!(test_isize_neg_257, isize, -257, "390100");
testcase!(test_isize_255, isize, 255, "18ff");
testcase!(test_i8_5, i8, 5, "05");
testcase!(test_i8_23, i8, 23, "17");
testcase!(test_i8_24, i8, 24, "1818");
testcase!(test_i8_neg_128, i8, -128, "387f");
testcase!(test_u32_98745874, u32, 98745874, "1a05e2be12");
testcase!(test_f32_1234_point_5, f32, 1234.5, "fa449a5000");
testcase!(test_f64_12345_point_6, f64, 12345.6, "fb40c81ccccccccccd");
testcase!(test_f64_nan, f64, ::std::f64::NAN, "f97e00");
testcase!(test_f64_infinity, f64, ::std::f64::INFINITY, "f97c00");
testcase!(test_f64_neg_infinity, f64, -::std::f64::INFINITY, "f9fc00");
testcase!(test_char_null, char, '\x00', "6100");
testcase!(test_char_broken_heart, char, '💔', "64f09f9294");
testcase!(test_str_pangram_de, String, "aâø↓é".to_owned(), "6a61c3a2c3b8e28693c3a9");
testcase!(test_bytes, ByteBuf, b"\x00\xab".to_vec().into(), "4200ab");
testcase!(test_unit, (), (), "f6");

#[cfg(feature="unstable")]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct UnitStruct;
#[cfg(feature="unstable")]
testcase!(test_unit_struct, UnitStruct, UnitStruct, "f6");

#[cfg(feature="unstable")]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct NewtypeStruct(bool);
#[cfg(feature="unstable")]
testcase!(test_newtype_struct, NewtypeStruct, NewtypeStruct(true), "f5");

testcase!(test_option_none, Option<u8>, None, "f6");
testcase!(test_option_some, Option<u8>, Some(42), "182a");
