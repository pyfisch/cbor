use crate::deserialize::peek;
use crate::serialize::values;
use crate::test_utils::assert_peek_simple;

proptest::proptest! {
    #[test]
    fn usmall(u in 0u8..23u8) {
        assert_peek_simple(u, values::usmall, peek::usmall);
    }

    #[test]
    fn u8(u: u8) {
        assert_peek_simple(u, values::u8, peek::u8);
    }

    #[test]
    fn u16(u: u16) {
        assert_peek_simple(u, values::u16, peek::u16);
    }

    #[test]
    fn u32(u: u32) {
        assert_peek_simple(u, values::u32, peek::u32);
    }

    #[test]
    fn u64(u: u64) {
        assert_peek_simple(u, values::u64, peek::u64);
    }

    #[test]
    fn uint(u: u64) {
        assert_peek_simple(u, values::uint, peek::uint);
    }

    #[test]
    fn negative_usmall(u in 0u8..23u8) {
        assert_peek_simple(u, values::negative_usmall, peek::negative_usmall);
    }

    #[test]
    fn negative_u8(u: u8) {
        assert_peek_simple(u, values::negative_u8, peek::negative_u8);
    }

    #[test]
    fn negative_u16(u: u16) {
        assert_peek_simple(u, values::negative_u16, peek::negative_u16);
    }

    #[test]
    fn negative_u32(u: u32) {
        assert_peek_simple(u, values::negative_u32, peek::negative_u32);
    }

    #[test]
    fn negative_u64(u: u64) {
        assert_peek_simple(u, values::negative_u64, peek::negative_u64);
    }

    #[test]
    fn negative_uint(u: u64) {
        assert_peek_simple(u, values::negative_uint, peek::negative_uint);
    }

    #[test]
    fn ismall(u in -24i8..23i8) {
        assert_peek_simple(u, values::ismall, peek::ismall);
    }

    #[test]
    fn i8(u: i8) {
        assert_peek_simple(u, values::i8, peek::i8);
    }

    #[test]
    fn i16(u: i16) {
        assert_peek_simple(u, values::i16, peek::i16);
    }

    #[test]
    fn i32(u: i32) {
        assert_peek_simple(u, values::i32, peek::i32);
    }

    #[test]
    fn i64(u: i64) {
        assert_peek_simple(u, values::i64, peek::i64);
    }

    #[test]
    fn int(u: i64) {
        assert_peek_simple(u, values::int, peek::int);
    }
}
