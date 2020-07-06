#![cfg(feature = "codec")]

use bytes::{BufMut, BytesMut};
use serde_cbor::{codec::Codec, error::Category};
use tokio_util::codec::{Decoder, Encoder};

#[test]
fn decode() {
    let mut codec = Codec::<u8, u8>::default();

    assert_eq!(
        codec
            .decode(&mut b"\xFF"[..].into())
            .err()
            .unwrap()
            .classify(),
        Category::Syntax
    );

    assert_eq!(
        codec
            .decode(&mut b"\x24"[..].into())
            .err()
            .unwrap()
            .classify(),
        Category::Data
    );

    assert_eq!(codec.decode(&mut b"\x07"[..].into()).unwrap().unwrap(), 7);

    let mut buf = BytesMut::with_capacity(2);
    assert_eq!(codec.decode(&mut buf).unwrap(), None);
    buf.put_u8(0x18);
    assert_eq!(codec.decode(&mut buf).unwrap(), None);
    buf.put_u8(0x18);
    assert_eq!(codec.decode(&mut buf).unwrap(), Some(24));
}

#[test]
fn encode() {
    let mut codec = Codec::<u8, u8>::default();
    let mut buf = BytesMut::new();

    codec.encode(&7, &mut buf).unwrap();
    assert_eq!(buf.len(), 1);
    assert_eq!(buf[0], 7);
}
