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
}