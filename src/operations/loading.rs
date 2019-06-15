use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Load<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Load<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Load<'a> {
    Load { instruction }
  }
}

impl<'a> Operation for Load<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = self.instruction.address.value() as usize;

    let word = computer.memory[address];
    let mut word = word.apply_field_spec(self.instruction.modification);

    if mix::op_codes::LDAN <= self.instruction.operation
      && self.instruction.operation <= mix::op_codes::LDXN
    {
      word = word.toggle_sign();
    }

    match self.instruction.operation {
      mix::op_codes::LDA | mix::op_codes::LDAN => {
        computer.accumulator = word;
      }
      mix::op_codes::LDX | mix::op_codes::LDXN => {
        computer.extension = word;
      }
      mix::op_codes::LD1...mix::op_codes::LD6 => {
        let index = (self.instruction.operation - mix::op_codes::LD1) as usize;
        computer.indexes[index] = word.cast_to_address();
      }
      mix::op_codes::LD1N...mix::op_codes::LD6N => {
        let index = (self.instruction.operation - mix::op_codes::LD1N) as usize;
        computer.indexes[index] = word.cast_to_address();
      }
      _ => panic!("unknown load operation {}", self.instruction.operation),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lda() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [1, 14, 3, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [0, 0, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [0, 0, 1, 14, 3],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 5],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 1),
          operation: mix::op_codes::LDA,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_acc) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
    }
  }

  #[test]
  fn test_ldx() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [1, 14, 3, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [0, 0, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [0, 0, 1, 14, 3],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 5],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 1),
          operation: mix::op_codes::LDX,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_ext) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension, *expected_ext);
    }
  }

  #[test]
  fn test_ldi() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [0, 0, 0, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LD1,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LD2,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LD3,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LD4,
        },
        mix::Address {
          bytes: [0, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LD5,
        },
        mix::Address {
          bytes: [0, 5],
          sign: mix::Sign::Positive,
        },
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LD6,
        },
        mix::Address {
          bytes: [0, 0],
          sign: mix::Sign::Negative,
        },
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg);
    }
  }

  #[test]
  fn test_ldan() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [1, 14, 3, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [0, 0, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [0, 0, 1, 14, 3],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 1),
          operation: mix::op_codes::LDAN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Negative,
        },
      ),
    ];

    for (instruction, expected_acc) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
    }
  }

  #[test]
  fn test_ldxn() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [1, 14, 3, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [1, 14, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [0, 0, 3, 5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [0, 0, 1, 14, 3],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 1),
          operation: mix::op_codes::LDXN,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Negative,
        },
      ),
    ];

    for (instruction, expected_ext) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.extension, *expected_ext);
    }
  }

  #[test]
  fn test_ldin() {
    let mut computer = Computer::new();
    computer.memory[2000] = mix::Word {
      bytes: [0, 0, 0, 5, 4],
      sign: mix::Sign::Negative,
    };

    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::LD1N,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Positive,
        },
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::LD2N,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(3, 5),
          operation: mix::op_codes::LD3N,
        },
        mix::Address {
          bytes: [5, 4],
          sign: mix::Sign::Negative,
        },
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 3),
          operation: mix::op_codes::LD4N,
        },
        mix::Address {
          bytes: [0, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(4, 4),
          operation: mix::op_codes::LD5N,
        },
        mix::Address {
          bytes: [0, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::new(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 0),
          operation: mix::op_codes::LD6N,
        },
        mix::Address {
          bytes: [0, 0],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (index, instruction, expected_reg) in &tests {
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.indexes[(index - 1) as usize], *expected_reg);
    }
  }
}
