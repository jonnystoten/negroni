use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Store<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Store<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Store<'a> {
    Store { instruction }
  }
}

impl<'a> Operation for Store<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let register = match self.instruction.operation {
      mix::op_codes::STA => computer.accumulator,
      mix::op_codes::STX => computer.extension.read(),
      mix::op_codes::ST1...mix::op_codes::ST6 => {
        let index = (self.instruction.operation - mix::op_codes::ST1) as usize;
        computer.indexes[index].cast_to_word()
      }
      mix::op_codes::STJ => computer.jump_address.cast_to_word(),
      mix::op_codes::STZ => mix::Word::zero(),
      _ => panic!("unknown store operation {}", self.instruction.operation),
    };

    let address = self.instruction.address.value();
    let (left, right) = mix::decode_field_spec(self.instruction.modification);
    let mut left = left;
    println!("left: {:?}", left);
    println!("right: {:?}", right);


    let mut num_bytes = ((right - left) + 1) as usize;

    if num_bytes > 0 && left == 0 {
      num_bytes -= 1;
    }
    println!("num_bytes: {:?}", num_bytes);

    let bytes = get_bytes_to_store(&register, num_bytes);
    let mut word = computer.memory[address as usize].read();
    if left == 0 {
      word.sign = register.sign;
      left += 1;
    }
    for i in 0..num_bytes {
      println!("{:?}", bytes);
      println!("{:?}", i);
      let value = bytes[i];
      word.bytes[left as usize + i - 1] = value;
    }

    computer.memory[address as usize].write(word);
  }
}

fn get_bytes_to_store<'a>(register: &'a mix::Word, count: usize) -> &'a [u8] {
  let offset = 5 - count;
  let start = offset;
  let end = count + offset - 1;
  &register.bytes[start..=end]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sta() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [6, 7, 8, 9, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [6, 7, 8, 9, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(5, 5),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [1, 2, 3, 4, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 2),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [1, 0, 3, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 3),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [1, 9, 0, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 1),
          operation: mix::op_codes::STA,
        },
        mix::Word {
          bytes: [0, 2, 3, 4, 5],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_mem) in &tests {
      let mut computer = Computer::new();
      computer.memory[2000].write(mix::Word {
        bytes: [1, 2, 3, 4, 5],
        sign: mix::Sign::Negative,
      });
      computer.accumulator = mix::Word {
        bytes: [6, 7, 8, 9, 0],
        sign: mix::Sign::Positive,
      };

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.memory[2000].read(), *expected_mem);
    }
  }

  #[test]
  fn test_stx() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [6, 7, 8, 9, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [6, 7, 8, 9, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(5, 5),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [1, 2, 3, 4, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 2),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [1, 0, 3, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 3),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [1, 9, 0, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 1),
          operation: mix::op_codes::STX,
        },
        mix::Word {
          bytes: [0, 2, 3, 4, 5],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_mem) in &tests {
      let mut computer = Computer::new();
      computer.memory[2000].write(mix::Word {
        bytes: [1, 2, 3, 4, 5],
        sign: mix::Sign::Negative,
      });
      computer.extension.write(mix::Word {
        bytes: [6, 7, 8, 9, 0],
        sign: mix::Sign::Positive,
      });

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.memory[2000].read(), *expected_mem);
    }
  }

  #[test]
  fn test_sti() {
    let tests = [
      (
        1,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::ST1,
        },
        mix::Word {
          bytes: [0, 0, 0, 6, 7],
          sign: mix::Sign::Positive,
        },
      ),
      (
        2,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::ST2,
        },
        mix::Word {
          bytes: [0, 0, 0, 6, 7],
          sign: mix::Sign::Negative,
        },
      ),
      (
        3,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(5, 5),
          operation: mix::op_codes::ST3,
        },
        mix::Word {
          bytes: [1, 2, 3, 4, 7],
          sign: mix::Sign::Negative,
        },
      ),
      (
        4,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 2),
          operation: mix::op_codes::ST4,
        },
        mix::Word {
          bytes: [1, 7, 3, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        5,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 3),
          operation: mix::op_codes::ST5,
        },
        mix::Word {
          bytes: [1, 6, 7, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        6,
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 1),
          operation: mix::op_codes::ST6,
        },
        mix::Word {
          bytes: [7, 2, 3, 4, 5],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (index, instruction, expected_mem) in &tests {
      let mut computer = Computer::new();
      computer.memory[2000].write(mix::Word {
        bytes: [1, 2, 3, 4, 5],
        sign: mix::Sign::Negative,
      });
      computer.indexes[(index - 1) as usize] = mix::Address {
        bytes: [6, 7],
        sign: mix::Sign::Positive,
      };

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.memory[2000].read(), *expected_mem);
    }
  }

  #[test]
  fn test_stj() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [0, 0, 0, 6, 7],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [0, 0, 0, 6, 7],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(5, 5),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [1, 2, 3, 4, 7],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 2),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [1, 7, 3, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 3),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [1, 6, 7, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 1),
          operation: mix::op_codes::STJ,
        },
        mix::Word {
          bytes: [7, 2, 3, 4, 5],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_mem) in &tests {
      let mut computer = Computer::new();
      computer.memory[2000].write(mix::Word {
        bytes: [1, 2, 3, 4, 5],
        sign: mix::Sign::Negative,
      });
      computer.jump_address = mix::Address {
        bytes: [6, 7],
        sign: mix::Sign::Positive,
      };

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.memory[2000].read(), *expected_mem);
    }
  }

  #[test]
  fn test_stz() {
    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Positive,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(1, 5),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [0, 0, 0, 0, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(5, 5),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [1, 2, 3, 4, 0],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 2),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [1, 0, 3, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(2, 3),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [1, 0, 0, 4, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 1),
          operation: mix::op_codes::STZ,
        },
        mix::Word {
          bytes: [0, 2, 3, 4, 5],
          sign: mix::Sign::Positive,
        },
      ),
    ];

    for (instruction, expected_mem) in &tests {
      let mut computer = Computer::new();
      computer.memory[2000].write(mix::Word {
        bytes: [1, 2, 3, 4, 5],
        sign: mix::Sign::Negative,
      });
      computer.jump_address = mix::Address {
        bytes: [6, 7],
        sign: mix::Sign::Positive,
      };

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.memory[2000].read(), *expected_mem);
    }
  }
}
