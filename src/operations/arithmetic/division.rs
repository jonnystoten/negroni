use crate::computer::Computer;

use crate::mix;
use crate::operations::Operation;

pub struct Division<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Division<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Division<'a> {
    Division { instruction }
  }
}

impl<'a> Operation for Division<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = computer.get_indexed_address_value(self.instruction) as usize;

    let word = computer.memory[address].read();
    let word = word.apply_field_spec(self.instruction.modification);

    if computer.accumulator.value().abs() >= word.value().abs() {
      println!("UNDEFINED BEHAVIOUR");
      println!(
        "{} >= {}",
        computer.accumulator.value().abs(),
        word.value().abs()
      );
      // the values of rA and rX is undefined behaviour (pg. 131) - we'll just zero them
      computer.accumulator = mix::Word::zero();
      computer.extension.write(mix::Word::zero());
      computer.overflow = true;
      return;
    }

    let rax = computer.accumulator.value() * 1073741824 + computer.extension.read().value().abs();

    let sign = if word.sign == computer.accumulator.sign {
      mix::Sign::Positive
    } else {
      mix::Sign::Negative
    };

    let quotient = rax / word.value();
    let remainder = rax.abs() % word.value().abs();

    let mut new_acc = mix::Word::from_value(quotient);
    new_acc.sign = sign;

    let mut new_ext = mix::Word::from_value(remainder);
    new_ext.sign = computer.accumulator.sign;

    computer.accumulator = new_acc;
    computer.extension.write(new_ext);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_div() {
    let tests = [
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 17],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 3],
          sign: mix::Sign::Positive,
        },
        mix::Instruction {
          address: mix::Address::from_value(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::DIV,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 5],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 2],
          sign: mix::Sign::Positive,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [0, 0, 0, 0, 0], // -[0]
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [19, 19, 0, 3, 1], // +[1235][0][3][1]
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 2, 0], // -[0][0][0][2][0]
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::from_value(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::DIV,
        },
        mix::Word {
          bytes: [0, 9, 41, 32, 1], // +[0][617][?][?]
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 1, 1], // -[0][0][0][?][1]
          sign: mix::Sign::Negative,
        },
        false,
      ),
      (
        mix::Word {
          bytes: [5, 4, 3, 2, 1],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 9, 8, 7, 6],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::from_value(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::DIV,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        true,
      ),
      (
        mix::Word {
          bytes: [5, 4, 3, 2, 1],
          sign: mix::Sign::Negative,
        },
        mix::Word {
          bytes: [0, 9, 8, 7, 6],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 20],
          sign: mix::Sign::Negative,
        },
        mix::Instruction {
          address: mix::Address::from_value(1000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::DIV,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
        true,
      ),
    ];

    for (prev_acc, prev_ext, prev_mem, instruction, expected_acc, expected_ext, expected_ov) in
      &tests
    {
      let mut computer = Computer::new();
      computer.accumulator = *prev_acc;
      computer.extension.write(*prev_ext);
      computer.memory[1000].write(*prev_mem);

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
      assert_eq!(computer.extension.read(), *expected_ext);
      assert_eq!(computer.overflow, *expected_ov);
    }
  }
}
