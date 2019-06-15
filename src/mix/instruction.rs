use super::{Address, Word};

use crate::mix::op_codes;
use crate::operations;

pub struct Instruction {
  pub operation: u8,
  pub modification: u8,
  pub address: Address,
  pub index_specification: u8,
}

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
      op_codes::LDA...op_codes::LDXN => Box::new(operations::Load::new(self)),
      op_codes::STA...op_codes::STX => Box::new(operations::Store::new(self)),
      op_codes::ENTA => Box::new(operations::AddressTransfer::new(self)),
      _ => panic!("unknown opcode {}", self.operation),
    }
  }
}
