#[derive(Debug, Copy, Clone)]
pub enum Sign {
  Positive,
  Negative,
}

#[derive(Debug, Copy, Clone)]
pub struct Word {
  pub sign: Sign,
  pub bytes: [u8; 5],
}