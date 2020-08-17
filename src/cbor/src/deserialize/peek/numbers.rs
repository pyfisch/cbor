use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;
use crate::serialize::values::Value;

macro_rules! make_peek_u_fn {
    ($fn_name: ident, $return_type: ty, $($test: path, $result: tt),*) => {
        make_peek_u_fn! { $fn_name, $return_type, $($test($result) => $result),* }
    };
    ($fn_name: ident, $return_type: ty, $($test: path),*) => {
        make_peek_u_fn! { $fn_name, $return_type, $($test(x) => x),* }
    };
    ($fn_name: ident, $return_type: ty, $($test: pat => $result: expr),*) => {
        pub fn $fn_name(bytes: &[u8]) -> Option<$return_type> {
            let v = Some(MajorType::from(bytes));
            $(
                let v = match v {
                    Some($test) => Some($result),
                    _ => None,
                };
            )*
            v
        }
    };
}

make_peek_u_fn!(usmall_, u8, MajorType::UnsignedInteger, MinorType::SameByte);
make_peek_u_fn!(u8_, u8, MajorType::UnsignedInteger, MinorType::OneByte);
make_peek_u_fn!(u16_, u16, MajorType::UnsignedInteger, MinorType::TwoBytes);
make_peek_u_fn!(u32_, u32, MajorType::UnsignedInteger, MinorType::FourBytes);
make_peek_u_fn!(u64_, u64, MajorType::UnsignedInteger, MinorType::EightBytes);

pub fn uint_(bytes: &[u8]) -> Option<u64> {
    match MajorType::from(bytes) {
        MajorType::UnsignedInteger(minor) => match minor {
            MinorType::SameByte(value) => Some(value as u64),
            MinorType::OneByte(value) => Some(value as u64),
            MinorType::TwoBytes(value) => Some(value as u64),
            MinorType::FourBytes(value) => Some(value as u64),
            MinorType::EightBytes(value) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

make_peek_u_fn! { negative_usmall_, u8, MajorType::NegativeInteger, minor, MinorType::SameByte, value }
make_peek_u_fn! { negative_u8_, u8, MajorType::NegativeInteger, MinorType::OneByte }
make_peek_u_fn! { negative_u16_, u16, MajorType::NegativeInteger, MinorType::TwoBytes }
make_peek_u_fn! { negative_u32_, u32, MajorType::NegativeInteger, MinorType::FourBytes }
make_peek_u_fn! { negative_u64_, u64, MajorType::NegativeInteger, MinorType::EightBytes }

pub fn negative_uint_(bytes: &[u8]) -> Option<u64> {
    match MajorType::from(bytes) {
        MajorType::NegativeInteger(minor) => match minor {
            MinorType::SameByte(value) => Some(value as u64),
            MinorType::OneByte(value) => Some(value as u64),
            MinorType::TwoBytes(value) => Some(value as u64),
            MinorType::FourBytes(value) => Some(value as u64),
            MinorType::EightBytes(value) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

macro_rules! make_peek_i_fn {
    ($fn_name: ident, $return_type: ty, $fn_negative: expr, $fn_positive: expr) => {
        pub fn $fn_name(bytes: &[u8]) -> Option<$return_type> {
            $fn_negative(bytes)
                .and_then(|v| Some(-(v as $return_type) - 1))
                .or_else(|| $fn_positive(bytes).and_then(|v| Some(v as $return_type)))
        }
    };
}

make_peek_i_fn!(ismall_, i8, negative_usmall_, usmall_);
make_peek_i_fn!(i8_, i8, negative_u8_, u8_);
make_peek_i_fn!(i16_, i16, negative_u16_, u16_);
make_peek_i_fn!(i32_, i32, negative_u32_, u32_);
make_peek_i_fn!(i64_, i64, negative_u64_, u64_);

pub fn int_(bytes: &[u8]) -> Option<i64> {
    match MajorType::from(bytes) {
        MajorType::UnsignedInteger(minor) => match minor {
            MinorType::SameByte(value) => Some(value as i64),
            MinorType::OneByte(value) => Some(value as i64),
            MinorType::TwoBytes(value) => Some(value as i64),
            MinorType::FourBytes(value) => Some(value as i64),
            MinorType::EightBytes(value) => Some(value as i64),
            _ => None,
        },
        MajorType::NegativeInteger(minor) => match minor {
            MinorType::SameByte(value) => Some(-(value as i64) - 1),
            MinorType::OneByte(value) => Some(-(value as i64) - 1),
            MinorType::TwoBytes(value) => Some(-(value as i64) - 1),
            MinorType::FourBytes(value) => Some(-(value as i64) - 1),
            MinorType::EightBytes(value) => Some(-(value as i64) - 1),
            _ => None,
        },
        _ => None,
    }
}

pub fn usmall(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::SameByte(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn u8(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::OneByte(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn u16(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::TwoBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn u32(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::FourBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn u64(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::EightBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn uint(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(_) => Some(Value::simple(major)),
        _ => None,
    }
}

pub fn negative_usmall(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::SameByte(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn negative_u8(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::OneByte(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn negative_u16(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::TwoBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn negative_u32(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::FourBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn negative_u64(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::EightBytes(_)) => Some(Value::simple(major)),
        _ => None,
    }
}
pub fn negative_uint(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(_) => Some(Value::simple(major)),
        _ => None,
    }
}

pub fn ismall(bytes: &[u8]) -> Option<Value<'static>> {
    usmall(bytes).or_else(|| negative_usmall(bytes))
}
pub fn i8(bytes: &[u8]) -> Option<Value<'static>> {
    u8(bytes).or_else(|| negative_u8(bytes))
}
pub fn i16(bytes: &[u8]) -> Option<Value<'static>> {
    u16(bytes).or_else(|| negative_u16(bytes))
}
pub fn i32(bytes: &[u8]) -> Option<Value<'static>> {
    u32(bytes).or_else(|| negative_u32(bytes))
}
pub fn i64(bytes: &[u8]) -> Option<Value<'static>> {
    u64(bytes).or_else(|| negative_u64(bytes))
}
pub fn int(bytes: &[u8]) -> Option<Value<'static>> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(_) => Some(Value::simple(major)),
        MajorType::NegativeInteger(_) => Some(Value::simple(major)),
        _ => None,
    }
}
