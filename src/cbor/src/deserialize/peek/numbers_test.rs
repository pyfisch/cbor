use crate::deserialize::peek;
use crate::serialize::values;
use crate::test_utils::assert_serialize;

#[test]
fn usmall() {
    assert_serialize(8, "08", values::usmall, peek::usmall);
}
