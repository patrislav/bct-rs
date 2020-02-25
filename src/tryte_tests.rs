use crate::tryte::*;

fn _print_all_trytes() {
  for num in -364..364 {
    let a = num / 27;
    let b = num % 27;
    println!("{} = {} r {}", num, a, b)
  }
}

fn assert_tryte_from_i16(number: i16, hi: u8, lo: u8) {
  let tryte = Tryte::from(number);
  println!(" -- LO --\nExpected: {:06b}\n     Got: {:06b}", lo, tryte.lo_bct());
  assert_eq!(tryte.lo_bct(), lo);
  println!(" -- HI --\nExpected: {:06b}\n     Got: {:06b}", hi, tryte.hi_bct());
  assert_eq!(tryte.hi_bct(), hi);
}

fn assert_tryte_into_i16(number: i16, hi: u8, lo: u8) {
  let tryte = Tryte::from_bct_trybbles(hi, lo);
  assert_eq!(tryte.to_integer(), number);
}

#[test]
fn tryte_positive_from_i16_3() {
  assert_tryte_from_i16(3, 0b_00_00_00, 0b_00_01_00); // 000010
}
#[test]
fn tryte_positive_from_i16_15() {
  assert_tryte_from_i16(15, 0b_00_00_01, 0b_10_10_00); // 001TT0
}
#[test]
fn tryte_positive_from_i16_100() {
  assert_tryte_from_i16(100, 0b_00_01_01, 0b_10_00_01); //011T01
}
#[test]
fn tryte_positive_from_i16_187() {
  assert_tryte_from_i16(187, 0b_01_10_01, 0b_00_10_01); // 1T10T1
}
#[test]
fn tryte_positive_from_i16_299() {
  assert_tryte_from_i16(299, 0b_01_01_10, 0b_00_01_10); // 11T01T
}

#[test]
fn tryte_negative_from_i16_3() {
  assert_tryte_from_i16(-3, 0b_00_00_00, 0b_00_10_00); // 0000T0
}
#[test]
fn tryte_negative_from_i16_15() {
  assert_tryte_from_i16(-15, 0b_00_00_10, 0b_01_01_00); // 00T110
}
#[test]
fn tryte_negative_from_i16_100() {
  assert_tryte_from_i16(-100, 0b_00_10_10, 0b_01_00_10); //0TT10T
}
#[test]
fn tryte_negative_from_i16_187() {
  assert_tryte_from_i16(-187, 0b_10_01_10, 0b_00_01_10); // T1T01T
}
#[test]
fn tryte_negative_from_i16_299() {
  assert_tryte_from_i16(-299, 0b_10_10_01, 0b_00_10_01); // TT10T1
}

#[test]
fn tryte_positive_into_i16_3() {
  assert_tryte_into_i16(3, 0b_00_00_00, 0b_00_01_00); // 000010
}
#[test]
fn tryte_positive_into_i16_15() {
  assert_tryte_into_i16(15, 0b_00_00_01, 0b_10_10_00); // 001TT0
}
#[test]
fn tryte_positive_into_i16_100() {
  assert_tryte_into_i16(100, 0b_00_01_01, 0b_10_00_01); //011T01
}
#[test]
fn tryte_positive_into_i16_187() {
  assert_tryte_into_i16(187, 0b_01_10_01, 0b_00_10_01); // 1T10T1
}
#[test]
fn tryte_positive_into_i16_299() {
  assert_tryte_into_i16(299, 0b_01_01_10, 0b_00_01_10); // 11T01T
}

#[test]
fn tryte_negative_into_i16_3() {
  assert_tryte_into_i16(-3, 0b_00_00_00, 0b_00_10_00); // 0000T0
}
#[test]
fn tryte_negative_into_i16_15() {
  assert_tryte_into_i16(-15, 0b_00_00_10, 0b_01_01_00); // 00T110
}
#[test]
fn tryte_negative_into_i16_100() {
  assert_tryte_into_i16(-100, 0b_00_10_10, 0b_01_00_10); //0TT10T
}
#[test]
fn tryte_negative_into_i16_187() {
  assert_tryte_into_i16(-187, 0b_10_01_10, 0b_00_01_10); // T1T01T
}
#[test]
fn tryte_negative_into_i16_299() {
  assert_tryte_into_i16(-299, 0b_10_10_01, 0b_00_10_01); // TT10T1
}

#[test]
fn test_as_ternary_string() {
  assert_eq!(Tryte::from(-3).as_ternary_string().unwrap(), "0000T0");
  assert_eq!(Tryte::from(187).as_ternary_string().unwrap(), "1T10T1");
  assert_eq!(Tryte::from(-299).as_ternary_string().unwrap(), "TT10T1");
}

#[test]
fn test_as_nonary_string() {
  assert_eq!(Tryte::from(-3).as_nonary_string().unwrap(), "00C");
  assert_eq!(Tryte::from(187).as_nonary_string().unwrap(), "23B");
  assert_eq!(Tryte::from(-299).as_nonary_string().unwrap(), "D3B");
}
