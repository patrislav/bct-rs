// This is not be the most efficient way to do it
// TODO: Come up with an algorithm and implement it
// Resource: http://homepage.divms.uiowa.edu/~jones/ternary/bct.shtml

use std::iter;

/// Translate a single BCT trybble (3 trits) into an integer
///
/// ```
/// # use bct::translate::*;
/// let result = bct_trybble_to_i8(0b010001);
/// assert_eq!(result, 10);
/// ```
pub fn bct_trybble_to_i8(bct: u8) -> i8 {
  match bct {
    0b101010 => -13,  // TTT
    0b101000 => -12,  // TT0
    0b101001 => -11,  // TT1
    0b100010 => -10,  // T0T
    0b100000 => -9,   // T00
    0b100001 => -8,   // T01
    0b100110 => -7,   // T1T
    0b100100 => -6,   // T10
    0b100101 => -5,   // T11
    0b001010 => -4,   // 0TT
    0b001000 => -3,   // 0T0
    0b001001 => -2,   // 0T1
    0b000010 => -1,   // 00T
    0b000000 => 0,    // 000
    0b000001 => 1,    // 001
    0b000110 => 2,    // 01T
    0b000100 => 3,    // 010
    0b000101 => 4,    // 011
    0b011010 => 5,    // 1TT
    0b011000 => 6,    // 1T0
    0b011001 => 7,    // 1T1
    0b010010 => 8,    // 10T
    0b010000 => 9,    // 100
    0b010001 => 10,   // 101
    0b010110 => 11,   // 11T
    0b010100 => 12,   // 110
    0b010101 => 13,   // 111
    _ => 0,
  }
}

/// Translate an integer (-13 through 13) to a BCT trybble (3 trits)
///
/// ```
/// # use bct::translate::*;
/// let result = i8_trybble_to_bct(10);
/// assert_eq!(result, 0b010001);
/// ```
pub fn i8_trybble_to_bct(number: i8) -> u8 {
  match number {
    -13 => 0b101010,  // TTT
    -12 => 0b101000,  // TT0
    -11 => 0b101001,  // TT1
    -10 => 0b100010,  // T0T
    -9 => 0b100000,   // T00
    -8 => 0b100001,   // T01
    -7 => 0b100110,   // T1T
    -6 => 0b100100,   // T10
    -5 => 0b100101,   // T11
    -4 => 0b001010,   // 0TT
    -3 => 0b001000,   // 0T0
    -2 => 0b001001,   // 0T1
    -1 => 0b000010,   // 00T
    0 => 0b000000,    // 000
    1 => 0b000001,    // 001
    2 => 0b000110,    // 01T
    3 => 0b000100,    // 010
    4 => 0b000101,    // 011
    5 => 0b011010,    // 1TT
    6 => 0b011000,    // 1T0
    7 => 0b011001,    // 1T1
    8 => 0b010010,    // 10T
    9 => 0b010000,    // 100
    10 => 0b010001,   // 101
    11 => 0b010110,   // 11T
    12 => 0b010100,   // 110
    13 => 0b010101,   // 111
   _ => 0,
  }
}

pub fn bct_trybble_to_ternary_string(bct: u8) -> Option<String> {
  let res = match bct {
    0b101010 => Some("TTT"),
    0b101000 => Some("TT0"),
    0b101001 => Some("TT1"),
    0b100010 => Some("T0T"),
    0b100000 => Some("T00"),
    0b100001 => Some("T01"),
    0b100110 => Some("T1T"),
    0b100100 => Some("T10"),
    0b100101 => Some("T11"),
    0b001010 => Some("0TT"),
    0b001000 => Some("0T0"),
    0b001001 => Some("0T1"),
    0b000010 => Some("00T"),
    0b000000 => Some("000"),
    0b000001 => Some("001"),
    0b000110 => Some("01T"),
    0b000100 => Some("010"),
    0b000101 => Some("011"),
    0b011010 => Some("1TT"),
    0b011000 => Some("1T0"),
    0b011001 => Some("1T1"),
    0b010010 => Some("10T"),
    0b010000 => Some("100"),
    0b010001 => Some("101"),
    0b010110 => Some("11T"),
    0b010100 => Some("110"),
    0b010101 => Some("111"),
    _ => None,
  };
  match res {
    Some(s) => Some(s.to_string()),
    None => None,
  }
}

fn negate_nonary(ns: &str) -> String {
  ns.chars().map(|c| {
    match c {
      'D' => '4',
      'C' => '3',
      'B' => '2',
      'A' => '1',
      '1' => 'A',
      '2' => 'B',
      '3' => 'C',
      '4' => 'D',
      _ => '0',
    }
  }).collect()
}

pub fn i16_tryte_to_nonary_string(number: i16) -> Option<String> {
  if number > 364 { return None }
  if number < -364 { return None }

  let is_negative = number < 0;
  let mut v = Vec::new();
  let mut n = if is_negative { -number } else { number };

  while n > 0 {
    let rem = n % 9;
    n = n / 9;
    if rem > 4 {
      n += 1;
    }

    let out = match rem {
      1 => "1",
      2 => "2",
      3 => "3",
      4 => "4",
      5 => "D",
      6 => "C",
      7 => "B",
      8 => "A",
      _ => "0",
    };
    v.push(out);
  }

  if v.len() < 3 {
    let diff = 3 - v.len();
    v.extend(iter::repeat("0").take(diff));
  }

  v.reverse();
  let result = v.join("");

  let nonary = if is_negative { negate_nonary(&result) } else { result };
  Some(nonary)
}
