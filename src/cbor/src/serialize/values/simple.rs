use crate::encoding::major_type::MajorType;
use crate::serialize::values::Value;

pub fn r#true() -> Value<'static> {
    Value::simple(MajorType::True())
}

pub fn r#false() -> Value<'static> {
    Value::simple(MajorType::False())
}

pub fn null() -> Value<'static> {
    Value::simple(MajorType::Null())
}

pub fn undefined() -> Value<'static> {
    Value::simple(MajorType::Undefined())
}

#[cfg(feature = "half")]
pub fn half_float(float: half::f16) -> Value<'static> {
    Value::simple(MajorType::HalfFloat(float.to_bits()))
}

#[cfg(feature = "half")]
pub fn half_float_from_f32(float: f32) -> Value<'static> {
    Value::simple(MajorType::HalfFloat(half::f16::from_f32(float).to_bits()))
}

pub fn half_float_from_u16(float: u16) -> Value<'static> {
    Value::simple(MajorType::HalfFloat(float))
}

pub fn float(f: f32) -> Value<'static> {
    Value::simple(MajorType::SingleFloat(f.to_bits()))
}

pub fn double_float(f: f64) -> Value<'static> {
    Value::simple(MajorType::DoubleFloat(f.to_bits()))
}
