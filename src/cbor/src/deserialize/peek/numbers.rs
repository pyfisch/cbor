use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;
use crate::serialize::owned::OwnedValue;

pub fn usmall(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::SameByte(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn u8(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::OneByte(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn u16(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::TwoBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn u32(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::FourBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn u64(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(MinorType::EightBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn uint(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(_) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}

pub fn negative_usmall(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::SameByte(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn negative_u8(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::OneByte(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn negative_u16(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::TwoBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn negative_u32(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::FourBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn negative_u64(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(MinorType::EightBytes(_)) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
pub fn negative_uint(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::NegativeInteger(_) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}

pub fn ismall(bytes: &[u8]) -> Option<OwnedValue> {
    usmall(bytes).or_else(|| negative_usmall(bytes))
}
pub fn i8(bytes: &[u8]) -> Option<OwnedValue> {
    u8(bytes).or_else(|| negative_u8(bytes))
}
pub fn i16(bytes: &[u8]) -> Option<OwnedValue> {
    u16(bytes).or_else(|| negative_u16(bytes))
}
pub fn i32(bytes: &[u8]) -> Option<OwnedValue> {
    u32(bytes).or_else(|| negative_u32(bytes))
}
pub fn i64(bytes: &[u8]) -> Option<OwnedValue> {
    u64(bytes).or_else(|| negative_u64(bytes))
}
pub fn int(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);
    match major {
        MajorType::UnsignedInteger(_) => Some(OwnedValue::simple(major)),
        MajorType::NegativeInteger(_) => Some(OwnedValue::simple(major)),
        _ => None,
    }
}
