use crate::serialize::values;

fn serialize(value: values::Value) -> Vec<u8> {
    let mut v = Vec::new();
    value.write(&mut v).expect("Unexpected io error.");
    v
}

#[test]
fn usmall() -> () {
    assert_eq!(serialize(values::usmall(0)), [0x00]);
    assert_eq!(serialize(values::usmall(9)), [0x09]);
    assert_eq!(serialize(values::usmall(22)), [0x16]);
    assert_eq!(serialize(values::usmall(23)), [0x17]);
    assert_eq!(serialize(values::usmall(24)), [0x17]);
    assert_eq!(serialize(values::usmall(90)), [0x17]);
}

#[test]
fn u8() -> () {
    assert_eq!(serialize(values::u8(0)), [0x18, 0]);
    assert_eq!(serialize(values::u8(90)), [0x18, 0x5a]);
    assert_eq!(serialize(values::u8(255)), [0x18, 0xff]);
}

#[test]
fn u16() -> () {
    assert_eq!(serialize(values::u16(0)), [0x19, 0, 0]);
    assert_eq!(serialize(values::u16(90)), [0x19, 0, 0x5a]);
    assert_eq!(serialize(values::u16(9000)), [0x19, 0x23, 0x28]);
}

#[test]
fn u32() -> () {
    assert_eq!(serialize(values::u32(0)), [0x1a, 0, 0, 0, 0]);
    assert_eq!(serialize(values::u32(90)), [0x1a, 0, 0, 0, 0x5a]);
    assert_eq!(serialize(values::u32(900000)), [0x1a, 0, 0x0d, 0xbb, 0xa0]);
}

#[test]
fn u64() -> () {
    assert_eq!(serialize(values::u64(0)), [0x1b, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        serialize(values::u64(90)),
        [0x1b, 0, 0, 0, 0, 0, 0, 0, 0x5a]
    );
    assert_eq!(
        serialize(values::u64(900000)),
        [0x1b, 0, 0, 0, 0, 0, 0x0d, 0xbb, 0xa0]
    );
    assert_eq!(
        serialize(values::u64(90000000000)),
        [0x1b, 0, 0, 0, 0x14, 0xf4, 0x6b, 0x04, 0]
    );
}
