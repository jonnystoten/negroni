use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Compare<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Compare<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Compare<'a> {
    Compare { instruction }
  }
}

impl<'a> Operation for Compare<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = computer.get_indexed_address_value(self.instruction) as usize;
    let word = computer.memory[address].read().apply_field_spec(self.instruction.modification);

    let register = match self.instruction.operation {
      mix::op_codes::CMPA => computer.accumulator,
      mix::op_codes::CMPX => computer.extension.read(),
      mix::op_codes::CMP1...mix::op_codes::CMP6 => {
        let index = (self.instruction.operation - mix::op_codes::CMP1) as usize;
        computer.indexes[index].cast_to_word()
      }
      _ => panic!("unknown compare operation: {}", self.instruction.operation),
    };

    let register = register.apply_field_spec(self.instruction.modification);

    let w_val = word.value();
    let r_val = register.value();

    let result = if r_val < w_val {
      mix::Comparison::Less
    } else if r_val > w_val {
      mix::Comparison::Greater
    } else {
      mix::Comparison::Equal
    };

    computer.comparison = result;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cmpa() {
    let tests = [
      (
        mix::Word::from_value(10),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPA,
        },
        mix::Comparison::Equal,
      ),
      (
        mix::Word::from_value(5),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPA,
        },
        mix::Comparison::Less,
      ),
      (
        mix::Word::from_value(15),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPA,
        },
        mix::Comparison::Greater,
      ),
      (
        mix::Word::negative_zero(),
        mix::Instruction {
          address: mix::Address::from_value(0),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPA,
        },
        mix::Comparison::Equal,
      ),
    ];

    for (acc_before, instruction, expected_cmp) in &tests {
      let mut computer = Computer::new();
      computer.accumulator = *acc_before;
      computer.memory[0].write(mix::Word::zero());
      computer.memory[2000].write(mix::Word::from_value(10));

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.comparison, *expected_cmp);
    }
  }

  #[test]
  fn test_cmpx() {
    let tests = [
      (
        mix::Word::from_value(10),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPX,
        },
        mix::Comparison::Equal,
      ),
      (
        mix::Word::from_value(5),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPX,
        },
        mix::Comparison::Less,
      ),
      (
        mix::Word::from_value(15),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPX,
        },
        mix::Comparison::Greater,
      ),
      (
        mix::Word::negative_zero(),
        mix::Instruction {
          address: mix::Address::from_value(0),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMPX,
        },
        mix::Comparison::Equal,
      ),
    ];

    for (ext_before, instruction, expected_cmp) in &tests {
      let mut computer = Computer::new();
      computer.extension.write(*ext_before);
      computer.memory[0].write(mix::Word::zero());
      computer.memory[2000].write(mix::Word::from_value(10));

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.comparison, *expected_cmp);
    }
  }

  #[test]
  fn test_cmpi() {
    let tests = [
      (
        1,
        mix::Address::from_value(10),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP1,
        },
        mix::Comparison::Equal,
      ),
      (
        2,
        mix::Address::from_value(5),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP2,
        },
        mix::Comparison::Less,
      ),
      (
        3,
        mix::Address::from_value(15),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP3,
        },
        mix::Comparison::Greater,
      ),
      (
        4,
        mix::Address::negative_zero(),
        mix::Instruction {
          address: mix::Address::from_value(0),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP4,
        },
        mix::Comparison::Equal,
      ),
      (
        5,
        mix::Address::from_value(5),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP5,
        },
        mix::Comparison::Less,
      ),
      (
        6,
        mix::Address::from_value(15),
        mix::Instruction {
          address: mix::Address::from_value(2000),
          index_specification: 0,
          modification: mix::field_spec(0, 5),
          operation: mix::op_codes::CMP6,
        },
        mix::Comparison::Greater,
      ),
    ];

    for (index, reg_before, instruction, expected_cmp) in &tests {
      let mut computer = Computer::new();
      computer.indexes[(index - 1) as usize] = *reg_before;
      computer.memory[0].write(mix::Word::zero());
      computer.memory[2000].write(mix::Word::from_value(10));

      instruction.decode().execute(&mut computer);

      assert_eq!(computer.comparison, *expected_cmp);
    }
  }
}