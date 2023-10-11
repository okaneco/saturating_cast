//! Tests for saturating conversions with signed integer source types.

use saturating_cast::SaturatingCast;

use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

#[test]
fn i8_lossy() {
    // impl_int_clamp_to_zero_with_larger_uint_max!(i8 => u8, u16, u32, u64, u128, usize);

    // u8
    assert_eq!(u8::MIN, i8::MIN.saturating_cast());
    assert_eq!(u8::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());

    // u16
    assert_eq!(u16::MIN, i8::MIN.saturating_cast());
    assert_eq!(u16::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());

    // u32
    assert_eq!(u32::MIN, i8::MIN.saturating_cast());
    assert_eq!(u32::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());

    // u64
    assert_eq!(u64::MIN, i8::MIN.saturating_cast());
    assert_eq!(u64::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, i8::MIN.saturating_cast());
    assert_eq!(u128::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());

    // usize
    assert_eq!(usize::MIN, i8::MIN.saturating_cast());
    assert_eq!(usize::try_from(i8::MAX).unwrap(), i8::MAX.saturating_cast());
}

#[test]
fn i16_lossy() {
    // impl_int_clamp_to_smaller!(i16 => i8);

    // i8
    assert_eq!(i8::MIN, i16::MIN.saturating_cast());
    assert_eq!(i8::MAX, i16::MAX.saturating_cast());

    // impl_int_clamp_to_zero_with_larger_uint_max!(i16 => u16, u32, u64, u128, usize);

    // u16
    assert_eq!(u16::MIN, i16::MIN.saturating_cast());
    assert_eq!(u16::try_from(i16::MAX).unwrap(), i16::MAX.saturating_cast());

    // u32
    assert_eq!(u32::MIN, i16::MIN.saturating_cast());
    assert_eq!(u32::try_from(i16::MAX).unwrap(), i16::MAX.saturating_cast());

    // u64
    assert_eq!(u64::MIN, i16::MIN.saturating_cast());
    assert_eq!(u64::try_from(i16::MAX).unwrap(), i16::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, i16::MIN.saturating_cast());
    assert_eq!(u128::try_from(i16::MAX).unwrap(), i16::MAX.saturating_cast());

    // usize
    assert_eq!(usize::MIN, i16::MIN.saturating_cast());
    assert_eq!(usize::try_from(i16::MAX).unwrap(), i16::MAX.saturating_cast());

    // impl_int_clamp_to_smaller!(i16 => u8);

    // u8
    assert_eq!(u8::MIN, i16::MIN.saturating_cast());
    assert_eq!(u8::MAX, i16::MAX.saturating_cast());
}

#[test]
fn i32_lossy() {
    // impl_int_clamp_to_smaller!(i32 => i8, i16);

    // i8
    assert_eq!(i8::MIN, i32::MIN.saturating_cast());
    assert_eq!(i8::MAX, i32::MAX.saturating_cast());

    // i16
    assert_eq!(i16::MIN, i32::MIN.saturating_cast());
    assert_eq!(i16::MAX, i32::MAX.saturating_cast());

    // impl_int_clamp_to_zero_with_larger_uint_max!(i32 => u32, u64, u128);

    // u32
    assert_eq!(u32::MIN, i32::MIN.saturating_cast());
    assert_eq!(u32::try_from(i32::MAX).unwrap(), i32::MAX.saturating_cast());

    // u64
    assert_eq!(u64::MIN, i32::MIN.saturating_cast());
    assert_eq!(u64::try_from(i32::MAX).unwrap(), i32::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, i32::MIN.saturating_cast());
    assert_eq!(u128::try_from(i32::MAX).unwrap(), i32::MAX.saturating_cast());

    // impl_int_clamp_to_smaller!(i32 => u8, u16);

    // u8
    assert_eq!(u8::MIN, i32::MIN.saturating_cast());
    assert_eq!(u8::MAX, i32::MAX.saturating_cast());

    // u16
    assert_eq!(u16::MIN, i32::MIN.saturating_cast());
    assert_eq!(u16::MAX, i32::MAX.saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // impl_int_isize_usize!(i32);
    {
        // isize
        assert_eq!(isize::try_from(i32::MIN).unwrap(), i32::MIN.saturating_cast());
        assert_eq!(isize::try_from(i32::MAX).unwrap(), i32::MAX.saturating_cast());

        // usize
        assert_eq!(usize::MIN, i32::MIN.saturating_cast());
        assert_eq!(usize::try_from(i32::MAX).unwrap(), i32::MAX.saturating_cast());
    }
}

#[test]
fn i64_lossy() {
    // impl_int_clamp_to_smaller!(i64 => i8, i16, i32);

    // i8
    assert_eq!(i8::MIN, i64::MIN.saturating_cast());
    assert_eq!(i8::MAX, i64::MAX.saturating_cast());

    // i16
    assert_eq!(i16::MIN, i64::MIN.saturating_cast());
    assert_eq!(i16::MAX, i64::MAX.saturating_cast());

    // i32
    assert_eq!(i32::MIN, i64::MIN.saturating_cast());
    assert_eq!(i32::MAX, i64::MAX.saturating_cast());

    // impl_int_clamp_to_zero_with_larger_uint_max!(i64 => u64, u128);

    // u64
    assert_eq!(u64::MIN, i64::MIN.saturating_cast());
    assert_eq!(u64::try_from(i64::MAX).unwrap(), i64::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, i64::MIN.saturating_cast());
    assert_eq!(u128::try_from(i64::MAX).unwrap(), i64::MAX.saturating_cast());

    // impl_int_clamp_to_smaller!(i64 => u8, u16, u32);

    // u8
    assert_eq!(u8::MIN, i64::MIN.saturating_cast());
    assert_eq!(u8::MAX, i64::MAX.saturating_cast());

    // u16
    assert_eq!(u16::MIN, i64::MIN.saturating_cast());
    assert_eq!(u16::MAX, i64::MAX.saturating_cast());

    // u32
    assert_eq!(u32::MIN, i64::MIN.saturating_cast());
    assert_eq!(u32::MAX, i64::MAX.saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // impl_int_isize_usize!(i64);
    {
        // isize
        assert_eq!(isize::try_from(i64::MIN).unwrap(), i64::MIN.saturating_cast());
        assert_eq!(isize::try_from(i64::MAX).unwrap(), i64::MAX.saturating_cast());

        // usize
        assert_eq!(usize::MIN, i64::MIN.saturating_cast());
        assert_eq!(usize::try_from(i64::MAX).unwrap(), i64::MAX.saturating_cast());
    }
}

#[test]
fn i128_lossy() {
    // impl_int_clamp_to_smaller!(i128 => i8, i16, i32, i64);

    // i8
    assert_eq!(i8::MIN, i128::MIN.saturating_cast());
    assert_eq!(i8::MAX, i128::MAX.saturating_cast());

    // i16
    assert_eq!(i16::MIN, i128::MIN.saturating_cast());
    assert_eq!(i16::MAX, i128::MAX.saturating_cast());

    // i32
    assert_eq!(i32::MIN, i128::MIN.saturating_cast());
    assert_eq!(i32::MAX, i128::MAX.saturating_cast());

    // i64
    assert_eq!(i64::MIN, i128::MIN.saturating_cast());
    assert_eq!(i64::MAX, i128::MAX.saturating_cast());

    // impl_int_clamp_to_zero_with_larger_uint_max!(i128 => u128);

    // u128
    assert_eq!(u128::MIN, i128::MIN.saturating_cast());
    assert_eq!(u128::try_from(i128::MAX).unwrap(), i128::MAX.saturating_cast());

    // impl_int_clamp_to_smaller!(i128 => u8, u16, u32, u64);

    // u8
    assert_eq!(u8::MIN, i128::MIN.saturating_cast());
    assert_eq!(u8::MAX, i128::MAX.saturating_cast());

    // u16
    assert_eq!(u16::MIN, i128::MIN.saturating_cast());
    assert_eq!(u16::MAX, i128::MAX.saturating_cast());

    // u32
    assert_eq!(u32::MIN, i128::MIN.saturating_cast());
    assert_eq!(u32::MAX, i128::MAX.saturating_cast());

    // u64
    assert_eq!(u64::MIN, i128::MIN.saturating_cast());
    assert_eq!(u64::MAX, i128::MAX.saturating_cast());

    #[cfg(target_pointer_width = "64")]
    // impl_int_isize_usize!(i128);
    {
        // isize
        assert_eq!(isize::MIN, i128::MIN.saturating_cast());
        assert_eq!(isize::MAX, i128::MAX.saturating_cast());

        // usize
        assert_eq!(usize::MIN, i128::MIN.saturating_cast());
        assert_eq!(usize::MAX, i128::MAX.saturating_cast());
    }
}

#[cfg(target_pointer_width = "64")]
#[test]
fn isize_lossy() {
    // impl_int_clamp_to_smaller!(isize => i8, i16);

    // i8
    assert_eq!(i8::MIN, isize::MIN.saturating_cast());
    assert_eq!(i8::MAX, isize::MAX.saturating_cast());

    // i16
    assert_eq!(i16::MIN, isize::MIN.saturating_cast());
    assert_eq!(i16::MAX, isize::MAX.saturating_cast());

    // impl_isize_casts!(i32, i64, i128);

    // i32
    assert_eq!(i32::MIN, isize::MIN.saturating_cast());
    assert_eq!(i32::MAX, isize::MAX.saturating_cast());

    // i64
    assert_eq!(i64::MIN, isize::MIN.saturating_cast());
    assert_eq!(i64::MAX, isize::MAX.saturating_cast());

    // i128
    assert_eq!(i128::try_from(isize::MIN).unwrap(), isize::MIN.saturating_cast());
    assert_eq!(i128::try_from(isize::MAX).unwrap(), isize::MAX.saturating_cast());

    // impl_int_clamp_to_zero_with_larger_uint_max!(isize => u64, u128, usize);

    // u64
    assert_eq!(u64::MIN, isize::MIN.saturating_cast());
    assert_eq!(u64::try_from(isize::MAX).unwrap(), isize::MAX.saturating_cast());

    // u128
    assert_eq!(u128::MIN, isize::MIN.saturating_cast());
    assert_eq!(u128::try_from(isize::MAX).unwrap(), isize::MAX.saturating_cast());

    // usize
    assert_eq!(usize::MIN, isize::MIN.saturating_cast());
    assert_eq!(usize::try_from(isize::MAX).unwrap(), isize::MAX.saturating_cast());

    // impl_int_clamp_to_smaller!(isize => u8, u16);

    // u8
    assert_eq!(u8::MIN, isize::MIN.saturating_cast());
    assert_eq!(u8::MAX, isize::MAX.saturating_cast());

    // u16
    assert_eq!(u16::MIN, isize::MIN.saturating_cast());
    assert_eq!(u16::MAX, isize::MAX.saturating_cast());

    // impl_isize_casts!(u32);

    // u32
    assert_eq!(u32::MIN, isize::MIN.saturating_cast());
    assert_eq!(u32::MAX, isize::MAX.saturating_cast());
}
