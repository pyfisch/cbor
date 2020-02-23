#![cfg(feature = "std")]
use crate::serialize::values;

fn value_of(c: u8) -> u8 {
    match c {
        b'A'...b'F' => (c - b'A' + 10),
        b'a'...b'f' => (c - b'a' + 10),
        b'0'...b'9' => (c - b'0'),
        c => panic!("Invalid hex value {}", c),
    }
}

fn check<T: AsRef<[u8]>>(value: values::Value, hex: T) {
    let hex = hex.as_ref();

    // If you mess this up, you don't deserve tests.
    assert_eq!(hex.len() % 2, 0);

    let bytes: Vec<u8> = hex
        .chunks(2)
        .enumerate()
        .map(|(_, pair)| value_of(pair[0]) << 4 | value_of(pair[1]))
        .collect();

    assert_eq!(value.to_bytes(), bytes, "{:?}", value);
}

#[test]
fn usmall() {
    check(values::usmall(0), "00");
    check(values::usmall(9), "09");
    check(values::usmall(22), "16");
    check(values::usmall(23), "17");
    check(values::usmall(24), "17");
    check(values::usmall(90), "17");
}

#[test]
fn u8() {
    check(values::u8(0), "1800");
    check(values::u8(90), "185a");
    check(values::u8(255), "18ff");
}

#[test]
fn u16() {
    check(values::u16(0), "190000");
    check(values::u16(90), "19005a");
    check(values::u16(9000), "192328");
}

#[test]
fn u32() {
    check(values::u32(0), "1a00000000");
    check(values::u32(90), "1a0000005a");
    check(values::u32(900000), "1a000dbba0");
}

#[test]
fn u64() {
    check(values::u64(0), "1b0000000000000000");
    check(values::u64(90), "1b000000000000005a");
    check(values::u64(900000), "1b00000000000dbba0");
    check(values::u64(90000000000), "1b00000014f46b0400");
}

#[test]
fn ismall() {
    check(values::ismall(0), "00");
    check(values::ismall(-9), "28");
    check(values::ismall(-23), "36");
    check(values::ismall(-24), "37");
    check(values::ismall(-25), "37");
    check(values::ismall(-90), "37");
    check(values::ismall(9), "09");
    check(values::ismall(22), "16");
    check(values::ismall(23), "17");
    check(values::ismall(24), "17");
    check(values::ismall(90), "17");
}

#[test]
fn i8() {
    check(values::i8(0), "1800");
    check(values::i8(90), "185a");
    check(values::i8(127), "187f");
    check(values::i8(0), "1800");
    check(values::i8(-1), "3800");
    check(values::i8(-90), "3859");
    check(values::i8(-128), "387f");
}

#[test]
fn i16() {
    check(values::i16(0), "190000");
    check(values::i16(90), "19005a");
    check(values::i16(9000), "192328");
    check(values::i16(-1), "390000");
    check(values::i16(-90), "390059");
    check(values::i16(-9000), "392327");
}
