use std::fmt;

use super::{Address, Sign, Word};

impl fmt::Debug for Word {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} [{}]",
      fmt_signed_bytes(&self.sign, &self.bytes, self.value()),
      fmt_instruction(self)
    )
  }
}

impl fmt::Debug for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      fmt_signed_bytes(&self.sign, &self.bytes, self.value())
    )
  }
}

fn fmt_signed_bytes(sign: &Sign, bytes: &[u8], value: isize) -> String {
  format!("{} {} ({:010})", fmt_sign(sign), fmt_bytes(bytes), value)
}

fn fmt_instruction(word: &Word) -> String {
  let address = &word.bytes[..2];
  let address_val = address[0] as isize * 64 + address[1] as isize;
  format!(
    "{} {:04} {}",
    fmt_sign(&word.sign),
    address_val,
    fmt_bytes(&word.bytes[2..])
  )
}

fn fmt_bytes(bytes: &[u8]) -> String {
  let mut result = String::new();
  for byte in bytes {
    result.push_str(&format!("{:02} ", byte));
  }
  result
}

fn fmt_sign(sign: &Sign) -> &'static str {
  match sign {
    Sign::Positive => "+",
    Sign::Negative => "-",
  }
}