#![cfg(all(feature = "std", feature = "half"))]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn simple() {
    let pairs = [
        values::key_value(values::text("F"), values::r#false()),
        values::key_value(values::text("T"), values::r#true()),
        values::key_value(values::text("N"), values::null()),
        values::key_value(values::text("U"), values::undefined()),
    ];

    assert_value(values::map(&pairs), "a46146f46154f5614ef66155f7");
}

#[test]
fn floats() {
    let pairs = [
        values::key_value(values::text("1"), values::half_float_from_f32(1.2)),
        values::key_value(values::text("2"), values::float(4.5)),
        values::key_value(values::text("3"), values::double_float(6.7)),
    ];

    assert_value(
        values::map(&pairs),
        "a36131f93ccd6132fa409000006133fb401acccccccccccd",
    );
}
