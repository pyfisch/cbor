//! Encoding and decoding for tokio

use crate::error::Category;
use crate::Error;

use std::marker::PhantomData;

use bytes::{buf::ext::BufMutExt, buf::Buf, BytesMut};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_util::codec;

/// A `tokio_util::codec::Encoder` for CBOR frames
pub struct Encoder<T: Serialize>(PhantomData<T>);

impl<T: Serialize> Default for Encoder<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Serialize> codec::Encoder<&T> for Encoder<T> {
    type Error = Error;

    fn encode(&mut self, item: &T, dst: &mut BytesMut) -> Result<(), Error> {
        crate::to_writer(dst.writer(), item)
    }
}

/// A `tokio_util::codec::Decoder` for CBOR frames
pub struct Decoder<T: DeserializeOwned>(PhantomData<T>);

impl<T: DeserializeOwned> Default for Decoder<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: DeserializeOwned> codec::Decoder for Decoder<T> {
    type Item = T;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Error> {
        let mut bytes: &[u8] = src.as_ref();
        let starting = bytes.len();

        let item: T = match crate::from_reader(&mut bytes) {
            Err(e) if e.classify() == Category::Eof => return Ok(None),
            Ok(v) => v,
            e => e?,
        };

        let ending = bytes.len();
        src.advance(starting - ending);
        Ok(Some(item))
    }
}

/// A Codec for CBOR frames
pub struct Codec<T: Serialize, U: DeserializeOwned>(Encoder<T>, Decoder<U>);

impl<T: Serialize, U: DeserializeOwned> Default for Codec<T, U> {
    fn default() -> Self {
        Codec(Encoder::default(), Decoder::default())
    }
}

impl<T: Serialize, U: DeserializeOwned> codec::Encoder<&T> for Codec<T, U> {
    type Error = Error;

    #[inline]
    fn encode(&mut self, item: &T, dst: &mut BytesMut) -> Result<(), Error> {
        self.0.encode(item, dst)
    }
}

impl<T: Serialize, U: DeserializeOwned> codec::Decoder for Codec<T, U> {
    type Item = U;
    type Error = Error;

    #[inline]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Error> {
        self.1.decode(src)
    }
}
