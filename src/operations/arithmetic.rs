use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Addition<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Addition<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Addition<'a> {
    Addition { instruction }
  }
}

impl<'a> Operation for Addition<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = self.instruction.address.value() as usize;

    let word = computer.memory[address];
    let word = word.apply_field_spec(self.instruction.modification);
    let mut value = word.value();

    if self.instruction.operation == mix::op_codes::SUB {
      value *= -1;
    }

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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let tests = [
      (
        mix::Word {
          bytes: [19, 18, 1, 2, 22], // +[1234][1][150],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [1, 36, 5, 0, 50], // +[100][5][50]
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::ADD,
        },
        mix::Word {
          bytes: [20, 54, 6, 3, 8], // +[1334][6][200]
          sign: mix::Sign::Positive,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::ADD,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::ADD,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [63, 63, 63, 63, 63],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::ADD,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        true,
      ),
    ];

    for (prev_acc, prev_mem, instruction, expected_acc, expected_ov) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = *prev_acc;
      computer.memory[1000] = *prev_mem;

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
      assert_eq!(computer.overflow, *expected_ov);
    }
  }

  #[test]
  fn test_sub() {
    let tests = [
      (
        mix::Word {
          bytes: [19, 18, 0, 0, 9], // -[1234][0][0][9]
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [31, 16, 2, 22, 0], // -[2000][150][0]
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::SUB,
        },
        mix::Word {
          bytes: [11, 62, 2, 21, 55], // +[766][149][?]
          sign: mix::Sign::Positive,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::SUB,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::SUB,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [63, 63, 63, 63, 63],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 1],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::SUB,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        true,
      ),
    ];

    for (prev_acc, prev_mem, instruction, expected_acc, expected_ov) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = *prev_acc;
      computer.memory[1000] = *prev_mem;

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
      assert_eq!(computer.overflow, *expected_ov);
    }
  }
}