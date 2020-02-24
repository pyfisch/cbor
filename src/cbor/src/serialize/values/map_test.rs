#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn map_small() {
    let pairs = [
        values::key_value(values::u8(1), values::u8(2)),
        values::key_value(values::u8(3), values::u16(4)),
        values::key_value(values::u8(5), values::u32(6)),
    ];

    assert_value(values::map(&pairs), "a318011802180319000418051a00000006");
}
