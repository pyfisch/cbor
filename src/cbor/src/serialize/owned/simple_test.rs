#![cfg(all(feature = "std", feature = "half"))]
use crate::serialize::owned;
use crate::test_utils::assert_value_owned;

#[test]
fn simple() {
    let pairs = [
        owned::key_value(owned::text("F"), owned::r#false()),
        owned::key_value(owned::text("T"), owned::r#true()),
        owned::key_value(owned::text("N"), owned::null()),
        owned::key_value(owned::text("U"), owned::undefined()),
    ];

    assert_value_owned(owned::map(&pairs), "a46146f46154f5614ef66155f7");
}

#[test]
fn floats() {
    let pairs = [
        owned::key_value(owned::text("1"), owned::half_float_from_f32(1.2)),
        owned::key_value(owned::text("2"), owned::float(4.5)),
        owned::key_value(owned::text("3"), owned::double_float(6.7)),
    ];

    assert_value_owned(
        owned::map(&pairs),
        "a36131f93ccd6132fa409000006133fb401acccccccccccd",
    );
}
