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
    let word = word.apply_field_spec(self.instruction.modification);

    computer.accumulator = word;
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
}
