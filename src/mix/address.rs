use super::{Word, Sign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
  pub sign: Sign,
  pub bytes: [u8; 2],
}

impl Address {
  pub fn new(value: isize) -> Address {
    let sign = if value.signum() < 0 {
      Sign::Negative
    } else {
      Sign::Positive
    };
    let value = value.abs();

    Address {
      bytes: [(value / 64) as u8, (value % 64) as u8],
      sign,
    }
  }

  pub fn value(&self) -> isize {
    let magnitude = (self.bytes[0] as isize) * 64 + (self.bytes[1] as isize);

    let sign = if self.sign == Sign::Positive { 1 } else { -1 };
    magnitude * sign
  }

  pub fn cast_to_word(&self) -> Word {
    Word {
      bytes: [0, 0, 0, self.bytes[0], self.bytes[1]],
      sign: self.sign,
    }
  }
}
