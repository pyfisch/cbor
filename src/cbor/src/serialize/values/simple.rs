use crate::encoding::major_type::MajorType;
use crate::serialize::values::Value;

pub fn r#true() -> Value<'static> {
    MajorType::True.into()
}

pub fn r#false() -> Value<'static> {
    MajorType::False.into()
}

pub fn null() -> Value<'static> {
    MajorType::Null.into()
}

pub fn undefined() -> Value<'static> {
    MajorType::Undefined.into()
}

#[cfg(feature = "half")]
pub fn half_float(float: half::f16) -> Value<'static> {
    MajorType::HalfFloat(float.to_bits()).into()
}

#[cfg(feature = "half")]
pub fn half_float_from_f32(float: f32) -> Value<'static> {
    MajorType::HalfFloat(half::f16::from_f32(float).to_bits()).into()
}

pub fn half_float_from_u16(float: u16) -> Value<'static> {
    MajorType::HalfFloat(float).into()
}

pub fn float(f: f32) -> Value<'static> {
    MajorType::SingleFloat(f).into()
}

pub fn double_float(f: f64) -> Value<'static> {
    MajorType::DoubleFloat(f).into()
}
