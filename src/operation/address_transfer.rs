use crate::computer::Computer;
use crate::instruction::Instruction;
use crate::operation::Operation;
use crate::words::{Word, Sign};

pub struct AddressTransfer<'a> {
  instruction: &'a Instruction
}

impl<'a> AddressTransfer<'a> {
  pub fn new(instruction: &'a Instruction) -> AddressTransfer<'a> {
    AddressTransfer{
      instruction
    }
  }
}

impl<'a> Operation for AddressTransfer<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    computer.accumulator = Word{
      bytes: [0 ,0, 0, (self.instruction.address / 64) as u8, (self.instruction.address % 64) as u8],
      sign: Sign::Negative,
    };
  }
}
