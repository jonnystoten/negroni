use crate::computer::Computer;


use crate::mix;
use crate::operations::Operation;
pub struct Enter<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Enter<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Enter<'a> {
    Enter { instruction }
  }
}

impl<'a> Operation for Enter<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let value = computer.get_indexed_address_value(self.instruction);
    let mut word = mix::Word::from_value(value);
    if value == 0 {
      word.sign = self.instruction.address.sign;
    }

    computer.accumulator = word;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_enta() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::new(-2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(-2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 1,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(2100),
      ),
      (
        mix::Instruction {
          address: mix::Address::new(0),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(0),
      ),
      (
        mix::Instruction {
          address: mix::Address::new(0),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(0),
      ),
      (
        mix::Instruction {
          address: mix::Address {
            bytes: [0; 2],
            sign: mix::Sign::Negative,
          },
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word {
          bytes: [0; 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address {
            bytes: [0; 2],
            sign: mix::Sign::Negative,
          },
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word {
          bytes: [0; 5],
          sign: mix::Sign::Negative,
        },
      ),
    ];

    for (instruction, expected_acc) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::new(100);
      computer.indexes[1] = mix::Address::new(0);
      computer.indexes[2] = mix::Address {
        bytes: [0; 2],
        sign: mix::Sign::Negative,
      }; // -0

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc)
    }
  }
}
