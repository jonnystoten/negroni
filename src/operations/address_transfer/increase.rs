use crate::computer::Computer;

use crate::mix;
use crate::operations::Operation;

pub struct Increase<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Increase<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Increase<'a> {
    Increase { instruction }
  }
}

impl<'a> Operation for Increase<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let mut value = computer.get_indexed_address_value(self.instruction);

    if self.instruction.modification == 1 {
      value *= -1;
    }

    // TODO: find a way to reduce the duplication here
    // TODO: maybe, if it's possible to assign to a dynamic field of a struct,
    // TODO: we can decide which register to read/write as runtime?
    match self.instruction.operation {
      mix::op_codes::INCA => {
        let sum = computer.accumulator.value() + value;

        let mut result = if mix::Word::fits_in_word(sum) {
          mix::Word::from_value(sum)
        } else {
          computer.overflow = true;
          mix::Word::from_value_with_overflow(sum)
        };

        if sum == 0 {
          // keep the sign unchanged (pg. 131)
          result.sign = computer.accumulator.sign;
        }

        computer.accumulator = result;
      }
      mix::op_codes::INCX => {
        let sum = computer.extension.read().value() + value;

        let mut result = if mix::Word::fits_in_word(sum) {
          mix::Word::from_value(sum)
        } else {
          computer.overflow = true;
          mix::Word::from_value_with_overflow(sum)
        };

        if sum == 0 {
          // keep the sign unchanged (pg. 131)
          result.sign = computer.extension.read().sign;
        }

        computer.extension.write(result);
      }
      mix::op_codes::INC1...mix::op_codes::INC6 => {
        let index = (self.instruction.operation - mix::op_codes::INC1) as usize;
        let i = computer.indexes[index];
        let sum = i.value() + value;
        computer.indexes[index] = mix::Address::from_value(sum);
      }
      _ => panic!("unknown op code"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_inca() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INCA,
        },
        mix::Word::from_value(3000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INCA,
        },
        mix::Word::from_value(-1000),
      ),
    ];

    // TODO: tests for overflow

    for (instruction, expected_acc) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = mix::Word::from_value(1000);

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc)
    }
  }

  #[test]
  fn test_incx() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INCX,
        },
        mix::Word::from_value(3000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INCX,
        },
        mix::Word::from_value(-1000),
      ),
    ];

    // TODO: tests for overflow

    for (instruction, expected_ext) in &tests {
      let mut computer = Computer::new();
      computer.extension.write(mix::Word::from_value(1000));

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension.read(), *expected_ext)
    }
  }

  #[test]
  fn test_inci() {
    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INC1,
        },
        mix::Address::from_value(3000),
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::INC2,
        },
        mix::Address::from_value(-1000),
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::from_value(100),
          index_specification: 1,
          modification: 0,
          operation: mix::op_codes::INC3,
        },
        mix::Address::from_value(1100),
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::from_value(750),
          index_specification: 2,
          modification: 0,
          operation: mix::op_codes::INC4,
        },
        mix::Address::from_value(1750),
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 0,
          operation: mix::op_codes::INC5,
        },
        mix::Address::from_value(1000),
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::from_value(-1000),
          index_specification: 2,
          modification: 0,
          operation: mix::op_codes::INC6,
        },
        mix::Address::zero(),
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      let mut computer = Computer::new();
      computer.indexes[(index - 1) as usize] = mix::Address::from_value(1000);

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg)
    }
  }

  #[test]
  fn test_deca() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DECA,
        },
        mix::Word::from_value(-1000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DECA,
        },
        mix::Word::from_value(3000),
      ),
    ];

    // TODO: tests for overflow

    for (instruction, expected_acc) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = mix::Word::from_value(1000);

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc)
    }
  }

  #[test]
  fn test_decx() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DECX,
        },
        mix::Word::from_value(-1000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DECX,
        },
        mix::Word::from_value(3000),
      ),
    ];

    // TODO: tests for overflow

    for (instruction, expected_ext) in &tests {
      let mut computer = Computer::new();
      computer.extension.write(mix::Word::from_value(1000));

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension.read(), *expected_ext)
    }
  }

  #[test]
  fn test_deci() {
    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DEC1,
        },
        mix::Address::from_value(-1000),
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::DEC2,
        },
        mix::Address::from_value(3000),
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::from_value(100),
          index_specification: 1,
          modification: 1,
          operation: mix::op_codes::DEC3,
        },
        mix::Address::from_value(900),
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::from_value(750),
          index_specification: 2,
          modification: 1,
          operation: mix::op_codes::DEC4,
        },
        mix::Address::from_value(250),
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 1,
          operation: mix::op_codes::DEC5,
        },
        mix::Address::from_value(1000),
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::from_value(1000),
          index_specification: 2,
          modification: 1,
          operation: mix::op_codes::DEC6,
        },
        mix::Address::zero(),
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      let mut computer = Computer::new();
      computer.indexes[(index - 1) as usize] = mix::Address::from_value(1000);

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg)
    }
  }
}
