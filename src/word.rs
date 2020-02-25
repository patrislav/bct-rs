use crate::tryte::Tryte;

#[derive(Debug)]
pub struct Word {
  hi: Tryte,
  lo: Tryte,
}

impl Clone for Word {
  fn clone(&self) -> Self {
    Word {
      hi: self.hi.clone(),
      lo: self.lo.clone(),
    }
  }
}

impl Word {
  pub fn from_trytes(hi: Tryte, lo: Tryte) -> Self {
    Word { hi, lo }
  }

  pub fn hi_tryte(&self) -> &Tryte {
    &self.hi
  }

  pub fn lo_tryte(&self) -> &Tryte {
    &self.lo
  }
}

const SHIFT: i16 = 729;
const HALF_SHIFT: i16 = 364;

impl Into<i32> for Word {
  fn into(self) -> i32 {
    let low: i16 = self.lo.into();
    let high: i16 = self.hi.into();
    (high as i32 * SHIFT as i32) + low as i32
  }
}

impl From<i32> for Word {
  fn from(number: i32) -> Self {
    let res = (number / SHIFT as i32) as i16;
    let rem = (number % SHIFT as i32) as i16;
    let (high, low) = if rem > HALF_SHIFT {
      (res + 1, rem - SHIFT)
    } else if rem < -HALF_SHIFT {
      (res - 1, rem + SHIFT)
    } else {
      (res, rem)
    };
    Word {
      hi: Tryte::from(high),
      lo: Tryte::from(low),
    }
  }
}
