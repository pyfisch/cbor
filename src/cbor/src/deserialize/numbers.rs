use crate::encoding::major_type::MajorType;
use crate::encoding::minor_type::MinorType;

pub fn is_usmall(bytes: &[u8]) -> bool {
    peek_usmall(bytes).is_some()
}

pub fn peek_usmall(bytes: &[u8]) -> Option<u8> {
    match MajorType::from(bytes) {
        MajorType::UnsignedInteger(minor) => {
            if let MinorType::SameByte(x) = minor {
                Some(x)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn peek_u8(bytes: &[u8]) -> Option<u8> {
    match MajorType::from(bytes) {
        MajorType::UnsignedInteger(minor) => {
            if let MinorType::OneByte(x) = minor {
                Some(x)
            } else {
                None
            }
        }
        _ => None,
    }
}
