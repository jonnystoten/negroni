
use crate::operation::address_transfer::AddressTransfer;
use crate::operation::Operation;
use crate::words;
pub struct Instruction {
  pub operation: u8,
  pub modification: u8,
  pub address: u16,
  pub index_specification: u8,
}

// TODO: configure this
const BYTE_SIZE: u8 = 64;

impl Instruction {
  pub fn from_word(word: words::Word) -> Instruction {
    let bytes = word.bytes;
    Instruction {
      address: ((bytes[0] as u16) * BYTE_SIZE as u16) + bytes[1] as u16,
      index_specification: bytes[2],
      modification: bytes[3],
      operation: bytes[4],
    }
  }

  pub fn decode(&self) -> impl Operation + '_ {
    match self.operation {
      48 => AddressTransfer::new(self),
      _ => panic!("unknown op code"),
    }
  }
}
