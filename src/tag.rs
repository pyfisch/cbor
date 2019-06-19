use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Wrapper struct to handle encoding Cbor semantic tags.
#[derive(Deserialize)]
pub struct EncodeCborTag<T: Serialize> {
    __cbor_tag_ser_tag: u64,
    __cbor_tag_ser_data: T,
}

impl<T: Serialize> EncodeCborTag<T> {
    /// Constructs a new `EncodeCborTag`, to wrap your type in a tag.
    pub fn new(tag: u64, value: T) -> Self {
        EncodeCborTag {
            __cbor_tag_ser_tag: tag,
            __cbor_tag_ser_data: value,
        }
    }

    /// Returns the tag.
    pub fn tag(&self) -> u64 {
        self.__cbor_tag_ser_tag
    }

    /// Returns the inner value, consuming the wrapper.
    pub fn value(self) -> T {
        self.__cbor_tag_ser_data
    }
}

impl<T: Serialize> Serialize for EncodeCborTag<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EncodeCborTag", 2)?;
        state.serialize_field("__cbor_tag_ser_tag", &self.__cbor_tag_ser_tag)?;
        state.serialize_field("__cbor_tag_ser_data", &self.__cbor_tag_ser_data)?;
        state.end()
    }
}
