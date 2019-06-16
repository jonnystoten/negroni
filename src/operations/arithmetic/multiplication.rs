use crate::computer::Computer;

use crate::mix;
use crate::operations::Operation;

pub struct Multiplication<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Multiplication<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Multiplication<'a> {
    Multiplication { instruction }
  }
}

impl<'a> Operation for Multiplication<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = self.instruction.address.value() as usize;

    let word = computer.memory[address];
    let word = word.apply_field_spec(self.instruction.modification);

    let sign = if word.sign == computer.accumulator.sign {
      mix::Sign::Positive
    } else {
      mix::Sign::Negative
    };

    let product = computer.accumulator.value() * word.value();
    let acc_result = product / 1073741824; // TODO: base on byte size
    let ext_result = product % 1073741824;

    let mut new_acc = mix::Word::from_value(acc_result);
    new_acc.sign = sign;

    let mut new_ext = mix::Word::from_value(ext_result);
    new_ext.sign = sign;

    computer.accumulator = new_acc;
    computer.extension = new_ext;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mul() {
    let tests = [
      (
        mix::Word {
          bytes: [1, 1, 1, 1, 1],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [1, 1, 1, 1, 1],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::MUL,
        },
        mix::Word {
          bytes: [0, 1, 2, 3, 4],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [5, 4, 3, 2, 1],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 1, 48], // -[112]
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [2, 43, 12, 63, 39], // ?[2][?][?][?][?]
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(1, 1),
          operation: mix::op_codes::MUL,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 0, 0, 3, 32], // -[224]
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Word {
          bytes: [50, 0, 1, 48, 4], // -[50][0][112][4]
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [2, 0, 0, 0, 0], // -[2][0][0][0][0]
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::new(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::MUL,
        },
        mix::Word {
          bytes: [1, 36, 0, 3, 32], // +[100][0][224]
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [8, 0, 0, 0, 0], // +[8][0][0][0][0]
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (prev_acc, prev_mem, instruction, expected_acc, expected_ext) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = *prev_acc;
      computer.memory[1000] = *prev_mem;

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
      assert_eq!(computer.extension, *expected_ext);
    }
  }
}