#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
  Positive,
  Negative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Word {
  pub sign: Sign,
  pub bytes: [u8; 5],
}

impl Word {
  pub fn new() -> Word {
    Word {
      bytes: [0; 5],
      sign: Sign::Positive,
    }
  }

  pub fn value(&self) -> isize {
    let magnitude = self
      .bytes
      .iter()
      .enumerate()
      .fold(0, |total, (index, byte)| {
        total + (64isize.pow(5 - index as u32 - 1) * *byte as isize)
      });

    let sign = if self.sign == Sign::Positive { 1 } else { -1 };
    magnitude * sign
  }

  pub fn apply_field_spec(&self, spec: u8) -> Word {
    let mut new_word = Word::new();
    new_word.sign = Sign::Positive;

    let (left, right) = super::decode_field_spec(spec);
    let mut left = left;
    if left == 0 {
      new_word.sign = self.sign;
      left = 1;
      if right == 0 {
        return new_word;
      }
    }

    let length = (right - left) + 1;
    let offset = 5 - length;

    for i in left..(right + 1) {
      let value = self.bytes[i as usize - 1];
      new_word.bytes[(i - left + offset) as usize] = value;
    }

    new_word
  }
}
