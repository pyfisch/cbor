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
    assert_eq!(value.to_vec(), hex_decode(hex), "{:?}", value);
}
