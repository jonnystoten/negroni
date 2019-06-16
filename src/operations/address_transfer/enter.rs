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

    if self.instruction.modification == 3 {
      word = word.toggle_sign();
    }

    match self.instruction.operation {
      mix::op_codes::ENTA => {
        computer.accumulator = word;
      }
      mix::op_codes::ENTX => {
        computer.extension = word;
      }
      mix::op_codes::ENT1...mix::op_codes::ENT6 => {
        let index = (self.instruction.operation - mix::op_codes::ENT1) as usize;
        computer.indexes[index] = word.cast_to_address();
      }
      _ => panic!("unknown enter operation {}", self.instruction.operation),
    }
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
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(-2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::from_value(2100),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTA,
        },
        mix::Word::negative_zero(),
      ),
    ];

    for (instruction, expected_acc) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc)
    }
  }

  #[test]
  fn test_entx() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::from_value(2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::from_value(-2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::from_value(2100),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENTX,
        },
        mix::Word::negative_zero(),
      ),
    ];

    for (instruction, expected_ext) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension, *expected_ext)
    }
  }

  #[test]
  fn test_enti() {
    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENT1,
        },
        mix::Address::from_value(2000),
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 2,
          operation: mix::op_codes::ENT2,
        },
        mix::Address::from_value(-2000),
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 2,
          operation: mix::op_codes::ENT3,
        },
        mix::Address::from_value(2100),
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENT4,
        },
        mix::Address::zero(),
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 2,
          operation: mix::op_codes::ENT5,
        },
        mix::Address::zero(),
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 2,
          operation: mix::op_codes::ENT6,
        },
        mix::Address::negative_zero(),
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg)
    }
  }

  #[test]
  fn test_enna() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::from_value(-2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::from_value(2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::from_value(-2100),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 3,
          modification: 3,
          operation: mix::op_codes::ENNA,
        },
        mix::Word::zero(),
      ),
    ];

    for (instruction, expected_acc) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc)
    }
  }

  #[test]
  fn test_ennx() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::from_value(-2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::from_value(2000),
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::from_value(-2100),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::negative_zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::zero(),
      ),
      (
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 3,
          modification: 3,
          operation: mix::op_codes::ENNX,
        },
        mix::Word::zero(),
      ),
    ];

    for (instruction, expected_ext) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension, *expected_ext)
    }
  }

  #[test]
  fn test_enni() {
    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENN1,
        },
        mix::Address::from_value(-2000),
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::from_value(-2000),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::ENN2,
        },
        mix::Address::from_value(2000),
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 1,
          modification: 3,
          operation: mix::op_codes::ENN3,
        },
        mix::Address::from_value(-2100),
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENN4,
        },
        mix::Address::negative_zero(),
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::zero(),
          index_specification: 3,
          modification: 3,
          operation: mix::op_codes::ENN5,
        },
        mix::Address::negative_zero(),
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::negative_zero(),
          index_specification: 2,
          modification: 3,
          operation: mix::op_codes::ENN6,
        },
        mix::Address::zero(),
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      let mut computer = Computer::new();
      computer.indexes[0] = mix::Address::from_value(100);
      computer.indexes[1] = mix::Address::zero();
      computer.indexes[2] = mix::Address::negative_zero();

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg)
    }
  }
}
