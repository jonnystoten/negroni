mod address;
mod instruction;
mod word;
mod debugging;

#[allow(dead_code)]
pub mod op_codes;
pub mod char_codes;
pub use address::Address;
pub use instruction::Instruction;
pub use word::{Sign, Word};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Comparison {
  Less,
  Equal,
  Greater,
}

pub fn field_spec(left: u8, right: u8) -> u8 {
  left * 8 + right
}

pub fn decode_field_spec(spec: u8) -> (u8, u8) {
  (spec / 8, spec % 8)
}
