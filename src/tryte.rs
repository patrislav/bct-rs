use crate::translate::{
  bct_trybble_to_i8,
  i8_trybble_to_bct,
  bct_trybble_to_ternary_string,
  i16_tryte_to_nonary_string,
};

#[derive(Debug)]
pub struct Tryte {
  hi: u8,
  lo: u8,
}

impl Clone for Tryte {
  fn clone(&self) -> Self {
    Tryte {
      hi: self.hi,
      lo: self.lo,
    }
  }
}

impl Tryte {
  /// Convert two BCT trybbles (3-trit numbers) to a Tryte (6 trits)
  ///
  /// ```
  /// # use bct::Tryte;
  /// let hi = 0b_00_01_10; // 01T
  /// let lo = 0b_10_00_01; // T01
  /// let tryte = Tryte::from_bct_trybbles(hi, lo);
  /// assert_eq!(tryte.to_integer(), 46);
  /// ```
  pub fn from_bct_trybbles(hi: u8, lo: u8) -> Self {
    Tryte { hi, lo }
  }

  /// Returns a value of the Tryte's high trybble (high 3 trits)
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46); // 01TT01
  /// assert_eq!(tryte.hi_value(), 2); // 01T => 2
  /// ```
  pub fn hi_value(&self) -> i8 {
    bct_trybble_to_i8(self.hi)
  }

  /// Returns a value of the Tryte's low trybble (low 3 trits)
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46); // 01TT01
  /// assert_eq!(tryte.lo_value(), -8); // T01 => -8
  /// ```
  pub fn lo_value(&self) -> i8 {
    bct_trybble_to_i8(self.lo)
  }

  /// Returns the BCT representation of the Tryte's high trybble (high 3 trits)
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46); // 01TT01
  /// assert_eq!(tryte.hi_bct(), 0b_00_01_10); // 01T
  /// ```
  pub fn hi_bct(&self) -> u8 {
    self.hi
  }

  /// Returns the BCT representation of the Tryte's low trybble (low 3 trits)
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46); // 01TT01
  /// assert_eq!(tryte.lo_bct(), 0b_10_00_01); // T01
  /// ```
  pub fn lo_bct(&self) -> u8 {
    self.lo
  }

  /// Returns a value of the Tryte
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46); // 01TT01
  /// assert_eq!(tryte.to_integer(), 46);
  /// ```
  pub fn to_integer(&self) -> i16 {
    let low = bct_trybble_to_i8(self.lo) as i16;
    let high = bct_trybble_to_i8(self.hi) as i16;
    (high * 27) + low
  }

  /// Returns ternary representation of a tryte
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46);
  /// assert_eq!(tryte.as_ternary_string().unwrap(), "01TT01");
  /// ```
  pub fn as_ternary_string(&self) -> Option<String> {
    let hi_opt = bct_trybble_to_ternary_string(self.hi);
    let lo_opt = bct_trybble_to_ternary_string(self.lo);
    match (hi_opt, lo_opt) {
      (Some(hi), Some(lo)) => Some([hi, lo].concat()),
      _ => None,
    }
  }

  /// Returns nonary representation of a tryte
  ///
  /// ```
  /// # use bct::Tryte;
  /// let tryte = Tryte::from(46);
  /// assert_eq!(tryte.as_nonary_string().unwrap(), "1D1");
  /// ```
  pub fn as_nonary_string(&self) -> Option<String> {
    i16_tryte_to_nonary_string(self.to_integer())
  }

  /// Adds two trytes together and returns the result and a carry
  ///
  /// ```
  /// # use bct::Tryte;
  /// let first = Tryte::from(46);
  /// let second = Tryte::from(54);
  /// let (result, _) = Tryte::add(&first, &second);
  /// assert_eq!(result.to_integer(), 100);
  /// ```
  pub fn add(first: &Tryte, second: &Tryte) -> (Tryte, i8) {
    let result = first.to_integer() + second.to_integer();
    let tryte = Tryte::from(result);
    let carry = 0;
    (tryte, carry)
  }

  /// Subtracts second from first and returns the result and a carry
  ///
  /// ```
  /// # use bct::Tryte;
  /// let first = Tryte::from(46);
  /// let second = Tryte::from(54);
  /// let (result, _) = Tryte::sub(&first, &second);
  /// assert_eq!(result.to_integer(), -8);
  /// ```
  pub fn sub(first: &Tryte, second: &Tryte) -> (Tryte, i8) {
    let result = first.to_integer() - second.to_integer();
    let tryte = Tryte::from(result);
    let carry = 0;
    (tryte, carry)
  }
}

impl Into<i16> for Tryte {
  fn into(self) -> i16 {
    self.to_integer()
  }
}

impl From<i16> for Tryte {
  fn from(number: i16) -> Self {
    let res = (number / 27) as i8;
    let rem = (number % 27) as i8;
    let (high, low) = if rem > 13 {
      (res + 1, rem - 27)
    } else if rem < -13 {
      (res - 1, rem + 27)
    } else {
      (res, rem)
    };
    Tryte {
      hi: i8_trybble_to_bct(high),
      lo: i8_trybble_to_bct(low),
    }
  }
}
