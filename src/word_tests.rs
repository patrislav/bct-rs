use crate::word::*;

fn assert_word_from_i32(number: i32, hi_expected: i16, lo_expected: i16) {
  let word = Word::from(number);
  let hi_actual = word.hi_tryte().to_integer();
  let lo_actual = word.lo_tryte().to_integer();
  assert_eq!(hi_actual, hi_expected);
  assert_eq!(lo_actual, lo_expected);
}

#[test]
fn word_positive_from_i32_9841() {
  assert_word_from_i32(9841, 13, 364); // 000111 111111
}

#[test]
fn word_negative_from_i32_9841() {
  assert_word_from_i32(-9841, -13, -364); // 000TTT TTTTTT
}
