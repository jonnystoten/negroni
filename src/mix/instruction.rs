use super::{Address, Word};

use crate::mix::op_codes;
use crate::operations;

#[derive(Debug)]
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
      op_codes::ADD | op_codes::SUB => Box::new(operations::Addition::new(self)),
      op_codes::MUL => Box::new(operations::Multiplication::new(self)),
      op_codes::DIV => Box::new(operations::Division::new(self)),
      op_codes::LDA...op_codes::LDXN => Box::new(operations::Load::new(self)),
      op_codes::STA...op_codes::STZ => Box::new(operations::Store::new(self)),
      op_codes::ENTA...op_codes::ENTX => match self.modification {
        0 | 1 => Box::new(operations::Increase::new(self)),
        2 | 3 => Box::new(operations::Enter::new(self)),
        _ => panic!(
          "unknown modification for address transfer: {}",
          self.modification
        ),
      },

      _ => panic!("unknown opcode {}", self.operation),
    }
  }
}
