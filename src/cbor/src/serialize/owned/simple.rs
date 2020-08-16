use crate::encoding::major_type::MajorType;
use crate::serialize::owned::OwnedValue;

pub fn bool(v: bool) -> OwnedValue {
    if v {
        r#true()
    } else {
        r#false()
    }
}

pub fn r#true() -> OwnedValue {
    OwnedValue::simple(MajorType::True())
}

pub fn r#false() -> OwnedValue {
    OwnedValue::simple(MajorType::False())
}

pub fn null() -> OwnedValue {
    OwnedValue::simple(MajorType::Null())
}

pub fn undefined() -> OwnedValue {
    OwnedValue::simple(MajorType::Undefined())
}

#[cfg(feature = "half")]
pub fn half_float(float: half::f16) -> OwnedValue {
    OwnedValue::simple(MajorType::HalfFloat(float.to_bits()))
}

#[cfg(feature = "half")]
pub fn f16(float: half::f16) -> OwnedValue {
    OwnedValue::simple(MajorType::HalfFloat(float.to_bits()))
}

#[cfg(feature = "half")]
pub fn half_float_from_f32(float: f32) -> OwnedValue {
    OwnedValue::simple(MajorType::HalfFloat(half::f16::from_f32(float).to_bits()))
}

pub fn half_float_from_u16(float: u16) -> OwnedValue {
    OwnedValue::simple(MajorType::HalfFloat(float))
}

pub fn float(f: f32) -> OwnedValue {
    OwnedValue::simple(MajorType::SingleFloat(f.to_bits()))
}
pub fn f32(f: f32) -> OwnedValue {
    OwnedValue::simple(MajorType::SingleFloat(f.to_bits()))
}

pub fn double_float(f: f64) -> OwnedValue {
    OwnedValue::simple(MajorType::DoubleFloat(f.to_bits()))
}
pub fn f64(f: f64) -> OwnedValue {
    OwnedValue::simple(MajorType::DoubleFloat(f.to_bits()))
}
