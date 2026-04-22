// If `src` can be promoted to `$dst`, then it must be Ok to cast `dst` back to
// `$src`
macro_rules! promote_and_back {
    ($($src:ident => $($dst:ident),+);+;) => {
        mod demoting_to {
            $(
                mod $src {
                    mod from {
                        use crate::From;

                        $(
                            quickcheck! {
                                fn $dst(src: $src) -> bool {
                                    $src::cast($dst::cast(src)).is_ok()
                                }
                            }
                         )+
                    }
                }
             )+
        }
    }
}

#[cfg(target_pointer_width = "32")]
promote_and_back! {
    i8    => f32, f64,     i16, i32, isize, i64, i128                            ;
    i16   => f32, f64,          i32, isize, i64, i128                            ;
    i32   =>      f64,                      i64, i128                            ;
    isize =>      f64,                      i64, i128                            ;
    i64   =>                                     i128                            ;
    u8    => f32, f64,     i16, i32, isize, i64, i128, u16, u32, usize, u64, u128;
    u16   => f32, f64,          i32, isize, i64, i128,      u32, usize, u64, u128;
    u32   =>      f64,                      i64, i128,                  u64, u128;
    usize =>      f64,                      i64, i128,                  u64, u128;
    u64   =>                                     i128,                       u128;
}

#[cfg(target_pointer_width = "64")]
promote_and_back! {
    i8    => f32, f64,     i16, i32, i64, isize, i128                            ;
    i16   => f32, f64,          i32, i64, isize, i128                            ;
    i32   =>      f64,               i64, isize, i128                            ;
    i64   =>                                     i128                            ;
    isize =>                                     i128                            ;
    u8    => f32, f64,     i16, i32, i64, isize, i128, u16, u32, u64, usize, u128;
    u16   => f32, f64,          i32, i64, isize, i128,      u32, u64, usize, u128;
    u32   =>      f64,               i64, isize, i128,           u64, usize, u128;
    u64   =>                                     i128,                       u128;
    usize =>                                     i128,                       u128;
}

// If it's Ok to cast `src` to `$dst`, it must also be Ok to cast `dst` back to
// `$src`
macro_rules! symmetric_cast_between {
    ($($src:ident => $($dst:ident),+);+;) => {
        mod symmetric_cast_between {
            $(
                mod $src {
                    mod and {
                        use quickcheck::TestResult;

                        use crate::From;

                        $(
                            quickcheck! {
                                fn $dst(src: $src) -> TestResult {
                                    if let Ok(dst) = $dst::cast(src) {
                                        TestResult::from_bool(
                                            $src::cast(dst).is_ok())
                                    } else {
                                        TestResult::discard()
                                    }
                                }
                            }
                         )+
                    }
                }
             )+
        }
    }
}

#[cfg(target_pointer_width = "32")]
symmetric_cast_between! {
    u8    =>           i8                      ;
    u16   =>           i8, i16                 ;
    u32   =>           i8, i16, i32            ;
    usize =>           i8, i16, i32            ;
    u64   =>           i8, i16, i32, i64, isize;
}

#[cfg(target_pointer_width = "64")]
symmetric_cast_between! {
    u8    =>           i8                            ;
    u16   =>           i8, i16                       ;
    u32   =>           i8, i16, i32                  ;
    u64   =>           i8, i16, i32, i64, isize      ;
    usize =>           i8, i16, i32, i64, isize      ;
    u128  =>           i8, i16, i32, i64, isize, i128;
}

macro_rules! from_float {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            mod $src {
                mod inf {
                    mod to {
                        use crate::{Error, From};

                        $(
                            #[test]
                            fn $dst() {
                                let _0: $src = 0.;
                                let _1: $src = 1.;
                                let inf = _1 / _0;
                                let neg_inf = -_1 / _0;

                                assert_eq!($dst::cast(inf),
                                           Err(Error::Infinite));
                                assert_eq!($dst::cast(neg_inf),
                                           Err(Error::Infinite));
                            }
                         )+
                    }
                }

                mod nan {
                    mod to {
                        use crate::{Error, From};

                        $(
                            #[test]
                            fn $dst() {
                                let _0: $src = 0.;
                                let nan = _0 / _0;

                                assert_eq!($dst::cast(nan),
                                           Err(Error::NaN));
                            }
                         )+
                    }
                }
            }
         )+
    }
}

from_float! {
    f32 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
    f64 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
}

#[test]
fn test_fl_conversion() {
    use crate::u128;
    assert_eq!(u128(42.0f32), Ok(42));
}

#[test]
fn gh16() {
    assert_eq!(super::u64(-0.01_f64), Ok(0));
    assert_eq!(super::u64(-0.99_f32), Ok(0));

    assert_eq!(super::u32(-0.99_f64), Ok(0));
    assert_eq!(super::u32(-0.01_f32), Ok(0));

    assert_eq!(super::u64(0.01_f64), Ok(0));
    assert_eq!(super::u64(0.99_f32), Ok(0));

    assert_eq!(super::u32(0.99_f64), Ok(0));
    assert_eq!(super::u32(0.01_f32), Ok(0));
}

#[test]
fn gh15() {
    assert_eq!(super::u32(32_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u32(32_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u64(64_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u64(64_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u8(8_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u8(8_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u16(16_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u16(16_f64.exp2()), Err(super::Error::Overflow));
}

#[test]
fn gh23_lossless_integer_max_min_to_float() {
    // f32::MANTISSA_DIGITS = 24
    assert_eq!(Ok(u8::MAX), super::u8(255f32));
    assert_eq!(Ok(u16::MAX), super::u16(65_535f32));

    // f64::MANTISSA_DIGITS = 53
    assert_eq!(Ok(u8::MAX), super::u8(255f64));
    assert_eq!(Ok(u16::MAX), super::u16(65_535f64));
    assert_eq!(Ok(u32::MAX), super::u32(4_294_967_295f64));

    // also check negative values (not part of the original bug)
    assert_eq!(Ok(i8::MIN), super::i8(-128f32));
    assert_eq!(Ok(i16::MIN), super::i16(-32_768f32));

    assert_eq!(Ok(i8::MIN), super::i8(-128f64));
    assert_eq!(Ok(i16::MIN), super::i16(-32_768f64));
    assert_eq!(Ok(i32::MIN), super::i32(-2_147_483_648f64));
}

// --- Exhaustive tests for all monomorphizations ---

// half_promotion: every signed -> unsigned pair where negative triggers Underflow
#[test]
fn test_half_promotion_underflow_exhaustive() {
    use super::Error;

    // i8 -> unsigned
    assert_eq!(super::u8(-1i8), Err(Error::Underflow));
    assert_eq!(super::u16(-1i8), Err(Error::Underflow));
    assert_eq!(super::u32(-1i8), Err(Error::Underflow));
    assert_eq!(super::usize(-1i8), Err(Error::Underflow));
    assert_eq!(super::u64(-1i8), Err(Error::Underflow));
    assert_eq!(super::u128(-1i8), Err(Error::Underflow));

    // i16 -> unsigned
    assert_eq!(super::u16(-1i16), Err(Error::Underflow));
    assert_eq!(super::u32(-1i16), Err(Error::Underflow));
    assert_eq!(super::usize(-1i16), Err(Error::Underflow));
    assert_eq!(super::u64(-1i16), Err(Error::Underflow));
    assert_eq!(super::u128(-1i16), Err(Error::Underflow));

    // i32 -> unsigned
    assert_eq!(super::u32(-1i32), Err(Error::Underflow));
    assert_eq!(super::usize(-1i32), Err(Error::Underflow));
    assert_eq!(super::u64(-1i32), Err(Error::Underflow));
    assert_eq!(super::u128(-1i32), Err(Error::Underflow));

    // i64 -> unsigned
    assert_eq!(super::u64(-1i64), Err(Error::Underflow));
    assert_eq!(super::usize(-1i64), Err(Error::Underflow));
    assert_eq!(super::u128(-1i64), Err(Error::Underflow));

    // isize -> unsigned
    assert_eq!(super::u64(-1isize), Err(Error::Underflow));
    assert_eq!(super::usize(-1isize), Err(Error::Underflow));
    assert_eq!(super::u128(-1isize), Err(Error::Underflow));

    // i128 -> unsigned
    assert_eq!(super::u128(-1i128), Err(Error::Underflow));
}

// from_unsigned: every unsigned -> smaller type pair where overflow triggers
#[test]
fn test_from_unsigned_overflow_exhaustive() {
    use super::Error;

    // u8 -> i8
    assert_eq!(super::i8(u8::MAX), Err(Error::Overflow));

    // u16 -> i8, i16, u8
    assert_eq!(super::i8(u16::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(u16::MAX), Err(Error::Overflow));
    assert_eq!(super::u8(u16::MAX), Err(Error::Overflow));

    // u32 -> i8, i16, i32, u8, u16
    assert_eq!(super::i8(u32::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(u32::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(u32::MAX), Err(Error::Overflow));
    assert_eq!(super::u8(u32::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(u32::MAX), Err(Error::Overflow));

    // usize -> i8, i16, i32, u8, u16, u32
    assert_eq!(super::i8(usize::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(usize::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(usize::MAX), Err(Error::Overflow));
    assert_eq!(super::u8(usize::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(usize::MAX), Err(Error::Overflow));
    assert_eq!(super::u32(usize::MAX), Err(Error::Overflow));

    // u64 -> i8, i16, i32, i64, isize, u8, u16, u32
    assert_eq!(super::i8(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::i64(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::isize(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::u8(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(u64::MAX), Err(Error::Overflow));
    assert_eq!(super::u32(u64::MAX), Err(Error::Overflow));

    // u128 -> i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize
    assert_eq!(super::i8(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::i64(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::i128(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::isize(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::u8(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::u32(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::u64(u128::MAX), Err(Error::Overflow));
    assert_eq!(super::usize(u128::MAX), Err(Error::Overflow));
}

// from_signed: every signed -> smaller signed/unsigned pair, both underflow and overflow
#[test]
fn test_from_signed_boundary_exhaustive() {
    use super::Error;

    // i16 -> i8, u8
    assert_eq!(super::i8(i8::MAX as i16 + 1), Err(Error::Overflow));
    assert_eq!(super::i8(i8::MIN as i16 - 1), Err(Error::Underflow));
    assert_eq!(super::u8(i16::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(u8::MAX as i16 + 1), Err(Error::Overflow));

    // i32 -> i8, i16, u8, u16
    assert_eq!(super::i8(i8::MAX as i32 + 1), Err(Error::Overflow));
    assert_eq!(super::i8(i8::MIN as i32 - 1), Err(Error::Underflow));
    assert_eq!(super::i16(i16::MAX as i32 + 1), Err(Error::Overflow));
    assert_eq!(super::i16(i16::MIN as i32 - 1), Err(Error::Underflow));
    assert_eq!(super::u8(i32::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(u8::MAX as i32 + 1), Err(Error::Overflow));
    assert_eq!(super::u16(i32::MIN), Err(Error::Underflow));
    assert_eq!(super::u16(u16::MAX as i32 + 1), Err(Error::Overflow));

    // isize -> i8, i16, i32, u8, u16, u32
    assert_eq!(super::i8(i8::MAX as isize + 1), Err(Error::Overflow));
    assert_eq!(super::i8(i8::MIN as isize - 1), Err(Error::Underflow));
    assert_eq!(super::i16(i16::MAX as isize + 1), Err(Error::Overflow));
    assert_eq!(super::i16(i16::MIN as isize - 1), Err(Error::Underflow));
    assert_eq!(super::i32(i32::MAX as isize + 1), Err(Error::Overflow));
    assert_eq!(super::i32(i32::MIN as isize - 1), Err(Error::Underflow));
    assert_eq!(super::u8(isize::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(u8::MAX as isize + 1), Err(Error::Overflow));
    assert_eq!(super::u16(isize::MIN), Err(Error::Underflow));
    assert_eq!(super::u16(u16::MAX as isize + 1), Err(Error::Overflow));
    assert_eq!(super::u32(isize::MIN), Err(Error::Underflow));
    assert_eq!(super::u32(u32::MAX as isize + 1), Err(Error::Overflow));

    // i64 -> i8, i16, i32, isize, u8, u16, u32, usize
    assert_eq!(super::i8(i8::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::i8(i8::MIN as i64 - 1), Err(Error::Underflow));
    assert_eq!(super::i16(i16::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::i16(i16::MIN as i64 - 1), Err(Error::Underflow));
    assert_eq!(super::i32(i32::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::i32(i32::MIN as i64 - 1), Err(Error::Underflow));
    assert_eq!(super::u8(i64::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(u8::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::u16(i64::MIN), Err(Error::Underflow));
    assert_eq!(super::u16(u16::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::u32(i64::MIN), Err(Error::Underflow));
    assert_eq!(super::u32(u32::MAX as i64 + 1), Err(Error::Overflow));
    assert_eq!(super::usize(i64::MIN), Err(Error::Underflow));

    // i128 -> i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
    assert_eq!(super::i8(i8::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::i8(i8::MIN as i128 - 1), Err(Error::Underflow));
    assert_eq!(super::i16(i16::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::i16(i16::MIN as i128 - 1), Err(Error::Underflow));
    assert_eq!(super::i32(i32::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::i32(i32::MIN as i128 - 1), Err(Error::Underflow));
    assert_eq!(super::i64(i64::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::i64(i64::MIN as i128 - 1), Err(Error::Underflow));
    assert_eq!(super::isize(i128::MAX), Err(Error::Overflow));
    assert_eq!(super::isize(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::u8(u8::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::u16(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::u16(u16::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::u32(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::u32(u32::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::u64(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::u64(u64::MAX as i128 + 1), Err(Error::Overflow));
    assert_eq!(super::usize(i128::MIN), Err(Error::Underflow));
    assert_eq!(super::usize(usize::MAX as i128 + 1), Err(Error::Overflow));
}

// from_float: every float -> integer pair, overflow + underflow for both signed and unsigned
#[test]
fn test_from_float_overflow_exhaustive() {
    use super::Error;

    // f32 -> signed overflow
    assert_eq!(super::i8(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::i64(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::isize(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::i128(f32::MAX), Err(Error::Overflow));

    // f64 -> signed overflow
    assert_eq!(super::i8(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::i16(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::i32(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::i64(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::isize(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::i128(f64::MAX), Err(Error::Overflow));

    // f32 -> unsigned overflow
    assert_eq!(super::u8(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::u32(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::u64(f32::MAX), Err(Error::Overflow));
    assert_eq!(super::usize(f32::MAX), Err(Error::Overflow));

    // f64 -> unsigned overflow
    assert_eq!(super::u8(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::u16(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::u32(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::u64(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::usize(f64::MAX), Err(Error::Overflow));
    assert_eq!(super::u128(f64::MAX), Err(Error::Overflow));
}

#[test]
fn test_from_float_underflow_signed_exhaustive() {
    use super::Error;

    // f32 -> signed underflow
    assert_eq!(super::i8(-f32::MAX), Err(Error::Underflow));
    assert_eq!(super::i16(-f32::MAX), Err(Error::Underflow));
    assert_eq!(super::i32(-f32::MAX), Err(Error::Underflow));
    assert_eq!(super::i64(-f32::MAX), Err(Error::Underflow));
    assert_eq!(super::isize(-f32::MAX), Err(Error::Underflow));
    assert_eq!(super::i128(-f32::MAX), Err(Error::Underflow));

    // f64 -> signed underflow
    assert_eq!(super::i8(-f64::MAX), Err(Error::Underflow));
    assert_eq!(super::i16(-f64::MAX), Err(Error::Underflow));
    assert_eq!(super::i32(-f64::MAX), Err(Error::Underflow));
    assert_eq!(super::i64(-f64::MAX), Err(Error::Underflow));
    assert_eq!(super::isize(-f64::MAX), Err(Error::Underflow));
    assert_eq!(super::i128(-f64::MAX), Err(Error::Underflow));
}

#[test]
fn test_from_float_underflow_unsigned_exhaustive() {
    use super::Error;

    // f32 -> unsigned underflow (src <= -1.0)
    assert_eq!(super::u8(-1.0f32), Err(Error::Underflow));
    assert_eq!(super::u16(-1.0f32), Err(Error::Underflow));
    assert_eq!(super::u32(-1.0f32), Err(Error::Underflow));
    assert_eq!(super::u64(-1.0f32), Err(Error::Underflow));
    assert_eq!(super::usize(-1.0f32), Err(Error::Underflow));
    assert_eq!(super::u128(-1.0f32), Err(Error::Underflow));

    // f64 -> unsigned underflow (src <= -1.0)
    assert_eq!(super::u8(-1.0f64), Err(Error::Underflow));
    assert_eq!(super::u16(-1.0f64), Err(Error::Underflow));
    assert_eq!(super::u32(-1.0f64), Err(Error::Underflow));
    assert_eq!(super::u64(-1.0f64), Err(Error::Underflow));
    assert_eq!(super::usize(-1.0f64), Err(Error::Underflow));
    assert_eq!(super::u128(-1.0f64), Err(Error::Underflow));
}

// from_float: verify normal (ok) path for all pairs
#[test]
fn test_from_float_ok_exhaustive() {
    // f32 -> all integer types, normal value
    assert_eq!(super::i8(42.0f32), Ok(42i8));
    assert_eq!(super::i16(42.0f32), Ok(42i16));
    assert_eq!(super::i32(42.0f32), Ok(42i32));
    assert_eq!(super::i64(42.0f32), Ok(42i64));
    assert_eq!(super::isize(42.0f32), Ok(42isize));
    assert_eq!(super::i128(42.0f32), Ok(42i128));
    assert_eq!(super::u8(42.0f32), Ok(42u8));
    assert_eq!(super::u16(42.0f32), Ok(42u16));
    assert_eq!(super::u32(42.0f32), Ok(42u32));
    assert_eq!(super::u64(42.0f32), Ok(42u64));
    assert_eq!(super::usize(42.0f32), Ok(42usize));
    assert_eq!(super::u128(42.0f32), Ok(42u128));

    // f64 -> all integer types, normal value
    assert_eq!(super::i8(42.0f64), Ok(42i8));
    assert_eq!(super::i16(42.0f64), Ok(42i16));
    assert_eq!(super::i32(42.0f64), Ok(42i32));
    assert_eq!(super::i64(42.0f64), Ok(42i64));
    assert_eq!(super::isize(42.0f64), Ok(42isize));
    assert_eq!(super::i128(42.0f64), Ok(42i128));
    assert_eq!(super::u8(42.0f64), Ok(42u8));
    assert_eq!(super::u16(42.0f64), Ok(42u16));
    assert_eq!(super::u32(42.0f64), Ok(42u32));
    assert_eq!(super::u64(42.0f64), Ok(42u64));
    assert_eq!(super::usize(42.0f64), Ok(42usize));
    assert_eq!(super::u128(42.0f64), Ok(42u128));
}

// from_float: unsigned truncation in (-1, 0) -> Ok(0)
#[test]
fn test_from_float_unsigned_truncation_exhaustive() {
    // f32 -> unsigned, value in (-1, 0) truncates to 0
    assert_eq!(super::u8(-0.5f32), Ok(0u8));
    assert_eq!(super::u16(-0.5f32), Ok(0u16));
    assert_eq!(super::u32(-0.5f32), Ok(0u32));
    assert_eq!(super::u64(-0.5f32), Ok(0u64));
    assert_eq!(super::usize(-0.5f32), Ok(0usize));
    assert_eq!(super::u128(-0.5f32), Ok(0u128));

    // f64 -> unsigned, value in (-1, 0) truncates to 0
    assert_eq!(super::u8(-0.5f64), Ok(0u8));
    assert_eq!(super::u16(-0.5f64), Ok(0u16));
    assert_eq!(super::u32(-0.5f64), Ok(0u32));
    assert_eq!(super::u64(-0.5f64), Ok(0u64));
    assert_eq!(super::usize(-0.5f64), Ok(0usize));
    assert_eq!(super::u128(-0.5f64), Ok(0u128));
}

// f64 -> f32 (manual impl)
#[test]
fn test_f64_to_f32() {
    use super::Error;

    assert!(super::f32(f64::NAN).unwrap().is_nan());
    assert_eq!(super::f32(f64::INFINITY), Ok(f32::INFINITY));
    assert_eq!(super::f32(f64::NEG_INFINITY), Ok(f32::NEG_INFINITY));
    assert_eq!(super::f32(0.0f64), Ok(0.0f32));
    assert_eq!(super::f32(1.0f64), Ok(1.0f32));
    assert_eq!(super::f32(-1.0f64), Ok(-1.0f32));
    assert_eq!(super::f32(f32::MAX as f64 * 2.0), Err(Error::Overflow));
    assert_eq!(super::f32(f32::MIN as f64 * 2.0), Err(Error::Underflow));
}
