use crate::deserialize::peek::peek;
use crate::encoding::{MajorType, MinorType};
use crate::serialize::values::Value;
use crate::serialize::WriteTo;

pub fn tag(bytes: &[u8]) -> Option<Value> {
    let major = MajorType::from(bytes);

    match major {
        MajorType::Tag(minor) => {
            let tag = match minor {
                MinorType::SameByte(x) => x as u64,
                MinorType::OneByte(x) => x as u64,
                MinorType::TwoBytes(x) => x as u64,
                MinorType::FourBytes(x) => x as u64,
                MinorType::EightBytes(x) => x as u64,
                _ => return None,
            };
            let offset = major.len() as usize;
            let rest = &bytes[offset..];
            peek(rest).map(|v| Value::from_tag(tag, v))
        }
        _ => None,
    }
}
