#![cfg(feature = "std")]
use crate::serialize::owned;
use crate::test_utils::assert_value_owned;

#[test]
fn array_small() {
    let v = {
        let values = [
            owned::usmall(0),
            owned::u8(1),
            owned::u16(2),
            owned::u32(3),
            owned::u64(4),
        ];

        owned::array(&values)
    };

    assert_value_owned(
        v,
        "850018011900021a000000031b0000000000000004",
    );
}

#[test]
fn array_recurse() {
    let inner = [
        owned::usmall(0),
        owned::u8(1),
        owned::u16(2),
        owned::u32(3),
        owned::u64(4),
    ];
    let values = [owned::array(&inner)];

    assert_value_owned(
        owned::array(&values),
        "81850018011900021a000000031b0000000000000004",
    );
}

#[test]
fn indefinite_array() {
    let values = [
        owned::usmall(1),
        owned::usmall(2),
        owned::usmall(3),
        owned::usmall(4),
        owned::usmall(5),
        owned::usmall(6),
        owned::usmall(7),
        owned::usmall(8),
    ];

    assert_value_owned(owned::indefinite_array(&values), "9f0102030405060708ff");
}
