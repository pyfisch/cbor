#![cfg(feature="unstable")]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_cbor;
use serde_cbor::{from_slice, to_vec};

#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
enum Enum {
    A,
    B,
}

#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
struct EnumStruct {
    e: Enum,
}

#[test]
fn test_enum() {
    let enum_struct = EnumStruct{ e: Enum::B };
    let re : EnumStruct = from_slice(&to_vec(&enum_struct).unwrap()).unwrap();
    assert_eq!(enum_struct, re);
}


#[repr(u16)]
#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
enum ReprEnum {
    A,  
    B,  
}

#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
struct ReprEnumStruct {
    e: ReprEnum,
}

#[test]
fn test_repr_enum() {
    let repr_enum_struct = ReprEnumStruct { e: ReprEnum::B };
    let re : ReprEnumStruct = from_slice(&to_vec(&repr_enum_struct).unwrap()).unwrap();
    assert_eq!(repr_enum_struct, re);
}


#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
enum DataEnum {
    A(u32),
    B(bool, u8),
    C { x: u8, y: String }
}

#[test]
fn test_data_enum() {
    let data_enum_a = DataEnum::A(4);
    let re_a : DataEnum = from_slice(&to_vec(&data_enum_a).unwrap()).unwrap();
    assert_eq!(data_enum_a, re_a);
    let data_enum_b = DataEnum::B(true, 42);
    let re_b : DataEnum = from_slice(&to_vec(&data_enum_b).unwrap()).unwrap();
    assert_eq!(data_enum_b, re_b);
    let data_enum_c = DataEnum::C { x: 3, y: "foo".to_owned() };
    println!("{:?}", &to_vec(&data_enum_c).unwrap());
    let re_c : DataEnum = from_slice(&to_vec(&data_enum_c).unwrap()).unwrap();
    assert_eq!(data_enum_c, re_c);
}

#[test]
fn test_serialize() {
    assert_eq!(to_vec(&Enum::A).unwrap(), &[97, 65]);
    assert_eq!(to_vec(&Enum::B).unwrap(), &[97, 66]);
    assert_eq!(to_vec(&DataEnum::A(42)).unwrap(), &[130, 97, 65, 24, 42]);
    assert_eq!(to_vec(&DataEnum::B(true, 9)).unwrap(),
               &[131, 97, 66, 245, 9]);
}
