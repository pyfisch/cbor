use crate::encoding::major_type::MajorType;
use crate::serialize::owned::OwnedValue;

pub fn bool(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        MajorType::True() => Some(OwnedValue::simple(MajorType::True())),
        MajorType::False() => Some(OwnedValue::simple(MajorType::False())),
        _ => None,
    }
}

pub fn r#true(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        MajorType::True() => Some(OwnedValue::simple(MajorType::True())),
        _ => None,
    }
}

pub fn r#false(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        MajorType::False() => Some(OwnedValue::simple(MajorType::False())),
        _ => None,
    }
}

pub fn null(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        MajorType::Null() => Some(OwnedValue::simple(MajorType::Null())),
        _ => None,
    }
}

pub fn undefined(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        MajorType::Undefined() => Some(OwnedValue::simple(MajorType::Undefined())),
        _ => None,
    }
}

pub fn float(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        #[cfg(feature = "half")]
        x @ MajorType::HalfFloat(_) => Some(OwnedValue::simple(x)),
        x @ MajorType::SingleFloat(_) => Some(OwnedValue::simple(x)),
        x @ MajorType::DoubleFloat(_) => Some(OwnedValue::simple(x)),
        _ => None,
    }
}

#[cfg(feature = "half")]
pub fn f16(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        x @ MajorType::HalfFloat(_) => Some(OwnedValue::simple(x)),
        _ => None,
    }
}

pub fn f32(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        x @ MajorType::SingleFloat(_) => Some(OwnedValue::simple(x)),
        _ => None,
    }
}

pub fn f64(bytes: &[u8]) -> Option<OwnedValue> {
    match MajorType::from(bytes) {
        x @ MajorType::DoubleFloat(_) => Some(OwnedValue::simple(x)),
        _ => None,
    }
}
