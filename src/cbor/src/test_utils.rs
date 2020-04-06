#![cfg(feature = "std")]
use crate::serialize::values::Value;

pub(crate) fn hex_decode<T: AsRef<[u8]>>(hex: T) -> Vec<u8> {
    fn value_of(c: u8) -> u8 {
        match c {
            b'A'...b'F' => (c - b'A' + 10),
            b'a'...b'f' => (c - b'a' + 10),
            b'0'...b'9' => (c - b'0'),
            c => panic!("Invalid hex value {}", c),
        }
    }

    let hex = hex.as_ref();

    // If you mess this up, you don't deserve tests.
    assert_eq!(hex.len() % 2, 0);

    hex.chunks(2)
        .enumerate()
        .map(|(_, pair)| value_of(pair[0]) << 4 | value_of(pair[1]))
        .collect()
}

pub(crate) fn assert_value<T: AsRef<[u8]>>(value: Value, hex: T) {
    let vector = value.to_vec();
    assert_eq!(vector, hex_decode(hex), "{:?}", value);
    assert_eq!(vector.len(), value.len(), "length");
}

pub(crate) fn assert_serialize<'a, DT, T: AsRef<[u8]>, ValueFn, PeekFn>(
    data: DT,
    hex: T,
    value: ValueFn,
    peek: PeekFn,
) where
    DT: std::cmp::PartialEq + std::fmt::Debug + Copy,
    ValueFn: Fn(DT) -> Value<'a>,
    PeekFn: Fn(&[u8]) -> Option<DT>,
{
    let value = value(data);
    let vector = value.to_vec();
    let decode = hex_decode(hex);

    assert_eq!(vector, decode, "serialize missed for {:?}", value);

    // Check deserialization.
    let x: Option<DT> = peek(vector.as_slice());
    assert!(x.is_some());
    assert_eq!(x.unwrap(), data);
}

pub(crate) fn assert_peek_simple<'a, DT, ValueFn, PeekFn>(data: DT, value: ValueFn, peek: PeekFn)
where
    DT: std::cmp::PartialEq + std::fmt::Debug + Copy,
    ValueFn: Fn(DT) -> Value<'a>,
    PeekFn: Fn(&[u8]) -> Option<DT>,
{
    let value = value(data);
    let vector = value.to_vec();

    // Check deserialization.
    let x: Option<DT> = peek(vector.as_slice());
    assert!(x.is_some());
    assert_eq!(x.unwrap(), data);
}
