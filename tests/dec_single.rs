//! [DecQuad] smoke tests.
//!
//! The purpose of smoke tests is to verify if bindings compile properly.

use dec_number_sys::*;

macro_rules! c {
  () => {
    &mut dec_context_32()
  };
}

macro_rules! n {
  ($s:expr) => {
    dec_single_from_string(stringify!($s), c!())
  };
}

macro_rules! s {
  ($v:expr) => {
    dec_single_to_string(&$v)
  };
}

/*

#[test]
fn test_dec_single_abs() {
  assert_eq!("5.751", s!(dec_single_abs(&n!(-5.751), c!())));
}

#[test]
fn test_dec_single_add() {
  assert_eq!("9.05", s!(dec_single_add(&n!(5.75), &n!(3.3), c!())));
}

#[test]
fn test_dec_single_compare() {
  assert_eq!("0", s!(dec_single_compare(&n!(1), &n!(1), c!())));
  assert_eq!("1", s!(dec_single_compare(&n!(2), &n!(1), c!())));
  assert_eq!("-1", s!(dec_single_compare(&n!(1), &n!(2), c!())));
  assert_eq!("NaN", s!(dec_single_compare(&n!(1), &n!(NaN), c!())));
  assert_eq!("NaN", s!(dec_single_compare(&n!(NaN), &n!(1), c!())));
}

#[test]
fn test_dec_single_divide() {
  assert_eq!("0.25", s!(dec_single_divide(&n!(1), &n!(4), c!())));
}

#[test]
#[rustfmt::skip]
fn test_dec_single_from_bcd() {
  assert_eq!("2366920938463463374607431768211455", s!(dec_single_from_bcd(&bcd_quad(u128::MAX), 0, false)));
  assert_eq!("18446744073709551615", s!(dec_single_from_bcd(&bcd_quad(u64::MAX.into()), 0, false)));
  assert_eq!("-18446744073709551615", s!(dec_single_from_bcd(&bcd_quad(u64::MAX.into()), 0, true)));
  assert_eq!("9223372036854775807", s!(dec_single_from_bcd(&bcd_quad(i64::MAX.unsigned_abs().into()), 0, false)));
  assert_eq!("-9223372036854775808", s!(dec_single_from_bcd(&bcd_quad(i64::MIN.unsigned_abs().into()), 0, true)));
  assert_eq!("1844674407.3709551615", s!(dec_single_from_bcd(&bcd_quad(u64::MAX.into()), -10, false)));
}

#[test]
fn test_dec_single_from_i32() {
  assert_eq!("-12", s!(dec_single_from_i32(-12)));
}

#[test]
fn test_dec_single_from_u32() {
  assert_eq!("12", s!(dec_single_from_u32(12)));
}

#[test]
fn test_dec_single_is_finite() {
  assert!(dec_single_is_finite(&n!(1)));
  assert!(!dec_single_is_finite(&n!(NaN)));
}

#[test]
fn test_dec_single_is_integer() {
  assert!(dec_single_is_integer(&n!(1)));
  assert!(!dec_single_is_integer(&n!(1.1)));
}

#[test]
fn test_dec_single_is_negative() {
  assert!(dec_single_is_negative(&n!(-1)));
  assert!(!dec_single_is_negative(&n!(1)));
  assert!(!dec_single_is_negative(&n!(NaN)));
}

#[test]
fn test_dec_single_is_positive() {
  assert!(dec_single_is_positive(&n!(1)));
  assert!(!dec_single_is_positive(&n!(-1)));
  assert!(!dec_single_is_positive(&n!(NaN)));
}

#[test]
fn test_dec_single_is_zero() {
  assert!(dec_single_is_zero(&n!(0)));
  assert!(!dec_single_is_zero(&n!(1)));
  assert!(!dec_single_is_zero(&n!(NaN)));
}

#[test]
fn test_dec_single_minus() {
  assert_eq!("0", s!(dec_single_minus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_single_minus(&n!(0), c!())));
  assert_eq!("1.1", s!(dec_single_minus(&n!(-1.1), c!())));
  assert_eq!("-1.1", s!(dec_single_minus(&n!(1.1), c!())));
}

#[test]
fn test_dec_single_multiply() {
  assert_eq!("4.4", s!(dec_single_multiply(&n!(1.1), &n!(4), c!())));
}

#[test]
fn test_dec_single_plus() {
  assert_eq!("0", s!(dec_single_plus(&n!(-0), c!())));
  assert_eq!("0", s!(dec_single_plus(&n!(0), c!())));
  assert_eq!("-1.1", s!(dec_single_plus(&n!(-1.1), c!())));
  assert_eq!("1.1", s!(dec_single_plus(&n!(1.1), c!())));
}

#[test]
fn test_dec_single_quantize() {
  assert_eq!("123.5", s!(dec_single_quantize(&n!(123.456), &n!(1.0), c!())));
}

#[test]
fn test_dec_single_reduce() {
  assert_eq!("1.2345678E+11", s!(dec_single_reduce(&n!(12345678E+4), c!())));
}

#[test]
fn test_dec_single_remainder() {
  assert_eq!("1", s!(dec_single_remainder(&n!(10), &n!(3), c!())));
}

#[test]
fn test_dec_single_scale_b() {
  assert_eq!("1234.5678", s!(dec_single_scale_b(&n!(12345678), &n!(-4), c!())));
}

#[test]
fn test_dec_single_subtract() {
  assert_eq!("1.000", s!(dec_single_subtract(&n!(1.123), &n!(0.123), c!())));
}

#[test]
fn test_dec_single_to_integral_value() {
  assert_eq!(
    "-0",
    s!(dec_single_to_integral_value(&n!(-0.75), c!(), DEC_ROUND_CEILING))
  );
}

*/

#[test]
#[rustfmt::skip]
fn test_dec_single_constants() {
  // zero
  assert_eq!("[22 50 00 00]", format!("{:?}", n!(0)));
  assert_eq!("[22 50 00 00]", format!("{:?}", DEC_SINGLE_ZERO));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_dec_single_copy_clone() {
  let n1 = n!(1.2345);
  let n2 = n1;
  let n3 = n1.clone();
  assert_eq!("1.2345", s!(n2));
  assert_eq!("1.2345", s!(n3));
}

#[test]
fn test_dec_single_zero() {
  let mut n = n!(-0.75);
  dec_single_zero(&mut n);
  assert_eq!("0", s!(n));
}
