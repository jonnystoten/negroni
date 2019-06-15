mod address;
mod instruction;
mod word;

pub use address::Address;
pub use instruction::Instruction;
pub use word::{Sign, Word};

pub fn field_spec(left: u8, right: u8) -> u8 {
  left * 8 + right
}
