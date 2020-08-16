#![cfg(feature = "std")]
use crate::serialize::owned;
use crate::test_utils::assert_value_owned;

#[test]
fn map_small() {
    let pairs = [
        owned::key_value(owned::u8(1), owned::u8(2)),
        owned::key_value(owned::u8(3), owned::u16(4)),
        owned::key_value(owned::u8(5), owned::u32(6)),
    ];

    assert_value_owned(owned::map(&pairs), "a318011802180319000418051a00000006");
}

#[test]
fn indefinite_map_small() {
    let pairs = [
        owned::key_value(owned::u8(1), owned::u8(2)),
        owned::key_value(owned::u8(3), owned::u16(4)),
        owned::key_value(owned::u8(5), owned::u32(6)),
    ];

    assert_value_owned(
        owned::indefinite_map(&pairs),
        "bf18011802180319000418051a00000006ff",
    );
}
