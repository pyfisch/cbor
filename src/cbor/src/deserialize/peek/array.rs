use crate::deserialize::peek::peek;
use crate::encoding::{MajorType, MinorType};
use crate::serialize::owned::OwnedValue;
use crate::serialize::WriteTo;

pub fn array(bytes: &[u8]) -> Option<OwnedValue> {
    let major = MajorType::from(bytes);

    match major {
        MajorType::Array(minor) => {
            let len = match minor {
                MinorType::SameByte(x) => x as usize,
                MinorType::OneByte(x) => x as usize,
                MinorType::TwoBytes(x) => x as usize,
                MinorType::FourBytes(x) => x as usize,
                MinorType::EightBytes(x) => x as usize,
                _ => return None,
            };

            let mut v = Vec::new();
            let mut offset = major.len() as usize;
            for _ in 0..len {
                // Overflow?
                if offset >= bytes.len() {
                    return None;
                }

                let subvalue = peek(&bytes[offset..]);
                match subvalue {
                    Some(s) => {
                        offset += s.len();
                        v.push(s);
                    }
                    None => {
                        eprintln!("1");
                        return None;
                    }
                }
            }
            Some(OwnedValue::from_array(v))
        }
        _ => None,
    }
}

pub fn indefinite_array(_bytes: &[u8]) -> Option<OwnedValue> {
    None
}
