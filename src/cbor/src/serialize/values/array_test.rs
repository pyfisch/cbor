#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn array_small() {
    let values = [
        values::usmall(0),
        values::u8(1),
        values::u16(2),
        values::u32(3),
        values::u64(4),
    ];

    assert_value(
        values::array(&values),
        "850018011900021a000000031b0000000000000004",
    );
}

#[test]
fn array_recurse() {
    let inner = [
        values::usmall(0),
        values::u8(1),
        values::u16(2),
        values::u32(3),
        values::u64(4),
    ];
    let values = [values::array(&inner)];

    assert_value(
        values::array(&values),
        "81850018011900021a000000031b0000000000000004",
    );
}
