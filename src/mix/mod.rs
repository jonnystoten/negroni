mod address;
mod instruction;
mod word;

pub use address::Address;
pub use instruction::Instruction;
pub use word::{Sign, Word};

pub fn field_spec(left: u8, right: u8) -> u8 {
  left * 8 + right
}

pub fn decode_field_spec(spec: u8) -> (u8, u8) {
  (spec / 8, spec % 8)
}

pub mod op_codes {
  pub const LDA: u8 = 8;
  pub const ENTA: u8 = 48;
}
