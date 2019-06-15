use crate::computer::Computer;

use crate::mix;
use super::Operation;

pub struct AddressTransfer<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> AddressTransfer<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> AddressTransfer<'a> {
    AddressTransfer { instruction }
  }
}

impl<'a> Operation for AddressTransfer<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = self.instruction.address;

    computer.accumulator = mix::Word {
      bytes: [0, 0, 0, address.bytes[0], address.bytes[1]],
      sign: address.sign,
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_enta() {
    let mut computer = Computer::new();
    let instruction = mix::Instruction {
      address: mix::Address::new(2000),
      index_specification: 0,
      modification: 2,
      operation: mix::op_codes::ENTA,
    };

    instruction.decode().execute(&mut computer);

    assert_eq!(computer.accumulator.value(), 2000)
  }

  #[test]
  fn test_enta_neg() {
    let mut computer = Computer::new();
    let instruction = mix::Instruction {
      address: mix::Address::new(-2000),
      index_specification: 0,
      modification: 2,
      operation: mix::op_codes::ENTA,
    };

    instruction.decode().execute(&mut computer);

    assert_eq!(computer.accumulator.value(), -2000)
  }
}
