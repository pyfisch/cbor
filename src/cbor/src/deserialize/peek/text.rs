use crate::encoding::{MajorType, MinorType};
use crate::serialize::owned::OwnedValue;
use crate::serialize::WriteTo;

pub fn text(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);

    match major {
        MajorType::Text(minor) => {
            let len = match minor {
                MinorType::SameByte(x) => x as usize,
                MinorType::OneByte(x) => x as usize,
                MinorType::TwoBytes(x) => x as usize,
                MinorType::FourBytes(x) => x as usize,
                MinorType::EightBytes(x) => x as usize,
                _ => return None,
            };

            let offset = major.len() as usize;
            std::str::from_utf8(&bytes[offset..offset + len])
                .ok()
                .map(|s| {
                    // Safe because we know bytes has the same lifetime as OwnedValue.
                    unsafe { OwnedValue::from_text(&*(s as *const str)) }
                })
        }
        _ => None,
    }
}
