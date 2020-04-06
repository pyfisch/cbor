use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;

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

make_peek_u_fn!(usmall, u8, MajorType::UnsignedInteger, MinorType::SameByte);
make_peek_u_fn!(u8, u8, MajorType::UnsignedInteger, MinorType::OneByte);
make_peek_u_fn!(u16, u16, MajorType::UnsignedInteger, MinorType::TwoBytes);
make_peek_u_fn!(u32, u32, MajorType::UnsignedInteger, MinorType::FourBytes);
make_peek_u_fn!(u64, u64, MajorType::UnsignedInteger, MinorType::EightBytes);

pub fn uint(bytes: &[u8]) -> Option<u64> {
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

make_peek_u_fn! { negative_usmall, u8, MajorType::NegativeInteger, minor, MinorType::SameByte, value }
make_peek_u_fn! { negative_u8, u8, MajorType::NegativeInteger, MinorType::OneByte }
make_peek_u_fn! { negative_u16, u16, MajorType::NegativeInteger, MinorType::TwoBytes }
make_peek_u_fn! { negative_u32, u32, MajorType::NegativeInteger, MinorType::FourBytes }
make_peek_u_fn! { negative_u64, u64, MajorType::NegativeInteger, MinorType::EightBytes }

pub fn negative_uint(bytes: &[u8]) -> Option<u64> {
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

make_peek_i_fn!(ismall, i8, negative_usmall, usmall);
make_peek_i_fn!(i8, i8, negative_u8, u8);
make_peek_i_fn!(i16, i16, negative_u16, u16);
make_peek_i_fn!(i32, i32, negative_u32, u32);
make_peek_i_fn!(i64, i64, negative_u64, u64);

pub fn int(bytes: &[u8]) -> Option<i64> {
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
