use std::fmt;

use super::{char_codes, Address, Instruction};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sign {
  Positive,
  Negative,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word {
  pub sign: Sign,
  pub bytes: [u8; 5],
}

impl Word {
  pub fn zero() -> Word {
    Word {
      bytes: [0; 5],
      sign: Sign::Positive,
    }
  }

  pub fn negative_zero() -> Word {
    Word {
      bytes: [0; 5],
      sign: Sign::Negative,
    }
  }

  pub fn from_value(value: isize) -> Word {
    if !Word::fits_in_word(value) {
      panic!("unexpected word overflow! value: {:?}", value);
    }

    Word::from_value_impl(value, false)
  }

  pub fn from_value_with_overflow(value: isize) -> Word {
    if Word::fits_in_word(value) {
      panic!(
        "from_value_with_overflow is only for word overflow! value: {:?}",
        value
      );
    }

    Word::from_value_impl(value, true)
  }

  fn from_value_impl(mut value: isize, allow_overflow: bool) -> Word {
    let mut word = Word::zero();

    if value < 0 {
      word.sign = Sign::Negative;
      value *= -1
    }

    if allow_overflow {
      value = value - 64isize.pow(5);
    }

    for i in 0..5 {
      let max_val = 64isize.pow(4 - i);
      let b = (value / max_val) as u8;
      word.bytes[i as usize] = b;
      value %= max_val
    }

    word
  }

  pub fn from_instruction(instruction: &Instruction) -> Word {
    Word {
      sign: instruction.address.sign,
      bytes: [
        instruction.address.bytes[0],
        instruction.address.bytes[1],
        instruction.index_specification,
        instruction.modification,
        instruction.operation,
      ],
    }
  }

  pub fn from_char_code(char_code: &str) -> Word {
    let mut word = Word::zero();
    for (index, ch) in char_code.chars().enumerate() {
      let byte = char_codes::get_code(&ch);
      word.bytes[index] = byte;
    }

    word
  }

  pub fn to_char_code(&self) -> String {
    let mut result = String::new();
    for byte in self.bytes.iter() {
      let ch = char_codes::get_char(byte);
      result.push(ch);
    }

    result
  }

  pub fn fits_in_word(value: isize) -> bool {
    let max = 64isize.pow(5) - 1;
    value.abs() <= max
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

  pub fn toggle_sign(&self) -> Word {
    Word {
      bytes: self.bytes,
      sign: if self.sign == Sign::Positive {
        Sign::Negative
      } else {
        Sign::Positive
      },
    }
  }

  pub fn apply_field_spec(&self, spec: u8) -> Word {
    let mut new_word = Word::zero();
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

  pub fn cast_to_address(&self) -> Address {
    Address {
      bytes: [self.bytes[3], self.bytes[4]],
      sign: self.sign,
    }
  }
}

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
