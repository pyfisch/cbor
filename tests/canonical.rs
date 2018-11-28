extern crate serde_cbor;
use serde_cbor::ObjectKey;

#[test]
fn integer_canonical_sort_order() {
    let expected = [
        0, 23, 24, 255, 256, 65535, 65536, 4294967295,
        -1, -24, -25, -256, -257, -65536, -65537, -4294967296,
    ].into_iter().map(|i| ObjectKey::Integer(*i)).collect::<Vec<_>>();

    let mut sorted = expected.clone();
    sorted.sort();

    assert_eq!(expected, sorted);
}

#[test]
fn string_canonical_sort_order() {
    let expected = [
        "", "a", "b", "aa",
    ].into_iter().map(|s| ObjectKey::String(s.to_string())).collect::<Vec<_>>();

    let mut sorted = expected.clone();
    sorted.sort();

    assert_eq!(expected, sorted);
}

#[test]
fn bytes_canonical_sort_order() {
    let expected = vec![
        vec![], vec![0u8], vec![1u8], vec![0u8, 0u8],
    ].into_iter().map(|v| ObjectKey::Bytes(v)).collect::<Vec<_>>();

    let mut sorted = expected.clone();
    sorted.sort();

    assert_eq!(expected, sorted);
}

#[test]
fn simple_data_canonical_sort_order() {
    let expected = vec![
        ObjectKey::Bool(false), ObjectKey::Bool(true), ObjectKey::Null
    ];

    let mut sorted = expected.clone();
    sorted.sort();

    assert_eq!(expected, sorted);
}

#[test]
fn major_type_canonical_sort_order() {
    let expected = vec![
        ObjectKey::Integer(0),
        ObjectKey::Integer(-1),
        ObjectKey::Bytes(vec![]),
        ObjectKey::String("".to_string()),
        ObjectKey::Null,
    ].into_iter().collect::<Vec<_>>();

    let mut sorted = expected.clone();
    sorted.sort();

    assert_eq!(expected, sorted);
}
