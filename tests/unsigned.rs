//! Tests for saturating conversions with unsigned integer source types.

use saturating_cast::SaturatingCast;

use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

#[test]
fn u8_lossy() {
    // impl_uint_clamp_to_max_bound!(u8 => i8);

    // i8
    assert_eq!(0_i8, u8::MIN.saturating_cast());
    assert_eq!(i8::MAX, u8::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_u8 << 7).saturating_cast());
}

#[test]
fn u16_lossy() {
    // impl_uint_clamp_to_max_bound!(u16 => u8);

    // u8
    assert_eq!(u8::MIN, u16::MIN.saturating_cast());
    assert_eq!(u8::MAX, u16::MAX.saturating_cast());
    assert_eq!(u8::MAX, (1_u16 << 8).saturating_cast());

    // impl_uint_clamp_to_max_bound!(u16 => i8, i16, isize);

    // i8
    assert_eq!(0_i8, u16::MIN.saturating_cast());
    assert_eq!(i8::MAX, u16::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_u16 << 7).saturating_cast());

    // i16
    assert_eq!(0_i16, u16::MIN.saturating_cast());
    assert_eq!(i16::MAX, u16::MAX.saturating_cast());
    assert_eq!(i16::MAX, (1_u16 << 15).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // isize
    {
        assert_eq!(0_isize, u16::MIN.saturating_cast());
        assert_eq!(isize::try_from(u16::MAX).unwrap(), u16::MAX.saturating_cast());
    }
}

#[test]
fn u32_lossy() {
    // impl_uint_clamp_to_max_bound!(u32 => u8, u16, usize);

    // u8
    assert_eq!(u8::MIN, u32::MIN.saturating_cast());
    assert_eq!(u8::MAX, u32::MAX.saturating_cast());
    assert_eq!(u8::MAX, (1_u32 << 8).saturating_cast());

    // u16
    assert_eq!(u16::MIN, u32::MIN.saturating_cast());
    assert_eq!(u16::MAX, u32::MAX.saturating_cast());
    assert_eq!(u16::MAX, (1_u32 << 16).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // usize
    {
        assert_eq!(usize::MIN, u32::MIN.saturating_cast());
        assert_eq!(usize::try_from(u32::MAX).unwrap(), u32::MAX.saturating_cast());
    }

    // impl_uint_clamp_to_max_bound!(u32 => i8, i16, i32, isize);

    // i8
    assert_eq!(0_i8, u32::MIN.saturating_cast());
    assert_eq!(i8::MAX, u32::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_u32 << 7).saturating_cast());

    // i16
    assert_eq!(0_i16, u32::MIN.saturating_cast());
    assert_eq!(i16::MAX, u32::MAX.saturating_cast());
    assert_eq!(i16::MAX, (1_u32 << 15).saturating_cast());

    // i32
    assert_eq!(0_i32, u32::MIN.saturating_cast());
    assert_eq!(i32::MAX, u32::MAX.saturating_cast());
    assert_eq!(i32::MAX, (1_u32 << 31).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // isize
    {
        assert_eq!(0_isize, u32::MIN.saturating_cast());
        assert_eq!(isize::try_from(u32::MAX).unwrap(), u32::MAX.saturating_cast());
    }
}

#[test]
fn u64_lossy() {
    // impl_uint_clamp_to_max_bound!(u64 => u8, u16, u32, usize);

    // u8
    assert_eq!(u8::MIN, u64::MIN.saturating_cast());
    assert_eq!(u8::MAX, u64::MAX.saturating_cast());
    assert_eq!(u8::MAX, (1_u64 << 8).saturating_cast());

    // u16
    assert_eq!(u16::MIN, u64::MIN.saturating_cast());
    assert_eq!(u16::MAX, u64::MAX.saturating_cast());
    assert_eq!(u16::MAX, (1_u64 << 16).saturating_cast());

    // u32
    assert_eq!(u32::MIN, u64::MIN.saturating_cast());
    assert_eq!(u32::MAX, u64::MAX.saturating_cast());
    assert_eq!(u32::MAX, (1_u64 << 32).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // usize
    {
        assert_eq!(usize::MIN, u64::MIN.saturating_cast());
        assert_eq!(usize::MAX, u64::MAX.saturating_cast());
    }

    // impl_uint_clamp_to_max_bound!(u64 => i8, i16, i32, i64, isize);

    // i8
    assert_eq!(0_i8, u64::MIN.saturating_cast());
    assert_eq!(i8::MAX, u64::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_u64 << 7).saturating_cast());

    // i16
    assert_eq!(0_i16, u64::MIN.saturating_cast());
    assert_eq!(i16::MAX, u64::MAX.saturating_cast());
    assert_eq!(i16::MAX, (1_u64 << 15).saturating_cast());

    // i32
    assert_eq!(0_i32, u64::MIN.saturating_cast());
    assert_eq!(i32::MAX, u64::MAX.saturating_cast());
    assert_eq!(i32::MAX, (1_u64 << 31).saturating_cast());

    // i64
    assert_eq!(0_i64, u64::MIN.saturating_cast());
    assert_eq!(i64::MAX, u64::MAX.saturating_cast());
    assert_eq!(i64::MAX, (1_u64 << 63).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // isize
    {
        assert_eq!(0_isize, u64::MIN.saturating_cast());
        assert_eq!(isize::MAX, u64::MAX.saturating_cast());
        assert_eq!(isize::MAX, (1_u64 << 63).saturating_cast());
    }
}

#[test]
fn u128_lossy() {
    // impl_uint_clamp_to_max_bound!(u128 => u8, u16, u32, u64, usize);

    // u8
    assert_eq!(u8::MIN, u128::MIN.saturating_cast());
    assert_eq!(u8::MAX, u128::MAX.saturating_cast());
    assert_eq!(u8::MAX, (1_u128 << 8).saturating_cast());

    // u16
    assert_eq!(u16::MIN, u128::MIN.saturating_cast());
    assert_eq!(u16::MAX, u128::MAX.saturating_cast());
    assert_eq!(u16::MAX, (1_u128 << 16).saturating_cast());

    // u32
    assert_eq!(u32::MIN, u128::MIN.saturating_cast());
    assert_eq!(u32::MAX, u128::MAX.saturating_cast());
    assert_eq!(u32::MAX, (1_u128 << 32).saturating_cast());

    // u64
    assert_eq!(u64::MIN, u128::MIN.saturating_cast());
    assert_eq!(u64::MAX, u128::MAX.saturating_cast());
    assert_eq!(u64::MAX, (1_u128 << 64).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // usize
    {
        assert_eq!(usize::MIN, u128::MIN.saturating_cast());
        assert_eq!(usize::MAX, u128::MAX.saturating_cast());
        assert_eq!(usize::MAX, (1_u128 << 64).saturating_cast());
    }

    // impl_uint_clamp_to_max_bound!(u128 => i8, i16, i32, i64, i128, isize);

    // i8
    assert_eq!(0_i8, u128::MIN.saturating_cast());
    assert_eq!(i8::MAX, u128::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_u128 << 7).saturating_cast());

    // i16
    assert_eq!(0_i16, u128::MIN.saturating_cast());
    assert_eq!(i16::MAX, u128::MAX.saturating_cast());
    assert_eq!(i16::MAX, (1_u128 << 15).saturating_cast());

    // i32
    assert_eq!(0_i32, u128::MIN.saturating_cast());
    assert_eq!(i32::MAX, u128::MAX.saturating_cast());
    assert_eq!(i32::MAX, (1_u128 << 31).saturating_cast());

    // i64
    assert_eq!(0_i64, u128::MIN.saturating_cast());
    assert_eq!(i64::MAX, u128::MAX.saturating_cast());
    assert_eq!(i64::MAX, (1_u128 << 63).saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // isize
    {
        assert_eq!(0_isize, u128::MIN.saturating_cast());
        assert_eq!(isize::MAX, u128::MAX.saturating_cast());
        assert_eq!(isize::MAX, (1_u128 << 63).saturating_cast());
    }
}

#[cfg(target_pointer_width = "64")]
#[test]
fn usize_lossy() {
    // impl_uint_clamp_to_max_bound!(usize => u8, u16, u32, u64, u128);

    // u8
    assert_eq!(u8::MIN, usize::MIN.saturating_cast());
    assert_eq!(u8::MAX, usize::MAX.saturating_cast());
    assert_eq!(u8::MAX, (1_usize << 8).saturating_cast());

    // u16
    assert_eq!(u16::MIN, usize::MIN.saturating_cast());
    assert_eq!(u16::MAX, usize::MAX.saturating_cast());
    assert_eq!(u16::MAX, (1_usize << 16).saturating_cast());

    // u32
    assert_eq!(u32::MIN, usize::MIN.saturating_cast());
    assert_eq!(u32::MAX, usize::MAX.saturating_cast());
    assert_eq!(u32::MAX, (1_usize << 32).saturating_cast());

    // u64
    assert_eq!(u64::MIN, usize::MIN.saturating_cast());
    assert_eq!(u64::MAX, usize::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, usize::MIN.saturating_cast());
    assert_eq!(u128::try_from(usize::MAX).unwrap(), usize::MAX.saturating_cast());

    // impl_uint_clamp_to_max_bound!(usize => i8, i16, i32, i64, i128, isize);

    // i8
    assert_eq!(0_i8, usize::MIN.saturating_cast());
    assert_eq!(i8::MAX, usize::MAX.saturating_cast());
    assert_eq!(i8::MAX, (1_usize << 7).saturating_cast());

    // i16
    assert_eq!(0_i16, usize::MIN.saturating_cast());
    assert_eq!(i16::MAX, usize::MAX.saturating_cast());
    assert_eq!(i16::MAX, (1_usize << 15).saturating_cast());

    // i32
    assert_eq!(0_i32, usize::MIN.saturating_cast());
    assert_eq!(i32::MAX, usize::MAX.saturating_cast());
    assert_eq!(i32::MAX, (1_usize << 31).saturating_cast());

    // i64
    assert_eq!(0_i64, usize::MIN.saturating_cast());
    assert_eq!(i64::MAX, usize::MAX.saturating_cast());
    assert_eq!(i64::MAX, (1_usize << 63).saturating_cast());

    // i128
    assert_eq!(0_i128, usize::MIN.saturating_cast());
    assert_eq!(i128::try_from(usize::MAX).unwrap(), usize::MAX.saturating_cast());

    // isize
    assert_eq!(0_isize, usize::MIN.saturating_cast());
    assert_eq!(isize::MAX, usize::MAX.saturating_cast());
    assert_eq!(isize::MAX, (1_usize << 63).saturating_cast());
}
