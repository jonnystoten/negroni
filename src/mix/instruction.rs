use super::{Address, Word};
use crate::operations;
use crate::mix::op_codes;

pub struct Instruction {
  pub operation: u8,
  pub modification: u8,
  pub address: Address,
  pub index_specification: u8,
}

// TODO: configure this
const BYTE_SIZE: u8 = 64;

impl Instruction {
  pub fn from_word(word: Word) -> Instruction {
    let bytes = word.bytes;
    Instruction {
      address: Address {
        bytes: [bytes[0], bytes[1]],
        sign: word.sign,
      },
      index_specification: bytes[2],
      modification: bytes[3],
      operation: bytes[4],
    }
  }

  pub fn decode(&self) -> Box<dyn operations::Operation + '_> {
    match self.operation {
      op_codes::LDA => Box::new(operations::Load::new(self)),
      op_codes::ENTA => Box::new(operations::AddressTransfer::new(self)),
      _ => panic!("unknown op code"),
    }
  }
}
