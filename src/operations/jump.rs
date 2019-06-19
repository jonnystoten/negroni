use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Jump<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Jump<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Jump<'a> {
    Jump { instruction }
  }
}

impl<'a> Operation for Jump<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = computer.get_indexed_address_value(self.instruction) as usize;

    match self.instruction.modification {
      0 => jump(address, computer),                    // JMP
      1 => jump_save_j(address, computer),             // JSJ
      2 => jump_on_overflow(address, computer, true),  // JOV
      3 => jump_on_overflow(address, computer, false), // JNOV
      4 => jump_on_comparison(address, computer, mix::Comparison::Less), // JL
      5 => jump_on_comparison(address, computer, mix::Comparison::Equal), // JE
      6 => jump_on_comparison(address, computer, mix::Comparison::Greater), // JG
      7 => jump_on_not_comparison(address, computer, mix::Comparison::Less), // JGE
      8 => jump_on_not_comparison(address, computer, mix::Comparison::Equal), // JNE
      9 => jump_on_not_comparison(address, computer, mix::Comparison::Greater), // JLE
      _ => panic!(
        "unknown modification for jump operation: {}",
        self.instruction.modification
      ),
    };
  }

  fn should_increment_program_counter(&self) -> bool {
    false
  }
}

pub struct RegisterJump<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> RegisterJump<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> RegisterJump<'a> {
    RegisterJump { instruction }
  }
}

impl<'a> Operation for RegisterJump<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = computer.get_indexed_address_value(self.instruction) as usize;

    let value = match self.instruction.operation {
      mix::op_codes::JAN => computer.accumulator.value(),
      mix::op_codes::JXN => computer.extension.value(),
      mix::op_codes::J1N...mix::op_codes::J6N => {
        let index = (self.instruction.operation - mix::op_codes::J1N) as usize;
        computer.indexes[index].value()
      }
      _ => panic!(
        "unknown opcode for jump operation: {}",
        self.instruction.operation
      ),
    };

    match self.instruction.modification {
      0 => conditional_jump(address, computer, value < 0), // JAN
      1 => conditional_jump(address, computer, value == 0), // JAZ
      2 => conditional_jump(address, computer, value > 0), // JAP
      3 => conditional_jump(address, computer, value >= 0), // JANN
      4 => conditional_jump(address, computer, value != 0), // JANZ
      5 => conditional_jump(address, computer, value <= 0), // JANP
      _ => panic!(
        "unknown modification for jump operation: {}",
        self.instruction.modification
      ),
    };
  }

  fn should_increment_program_counter(&self) -> bool {
    false
  }
}

fn jump(address: usize, computer: &mut Computer) -> () {
  computer.jump_address = mix::Address::from_value(computer.program_counter as isize + 1);
  computer.program_counter = address;
}

fn jump_save_j(address: usize, computer: &mut Computer) -> () {
  computer.program_counter = address;
}

fn conditional_jump(address: usize, computer: &mut Computer, condition: bool) -> () {
  if condition {
    jump(address, computer);
  } else {
    computer.program_counter += 1;
  }
}

fn jump_on_overflow(address: usize, computer: &mut Computer, overflow: bool) -> () {
  conditional_jump(address, computer, computer.overflow == overflow);
  computer.overflow = false;
}

fn jump_on_comparison(address: usize, computer: &mut Computer, comparison: mix::Comparison) -> () {
  conditional_jump(address, computer, computer.comparison == comparison);
}

fn jump_on_not_comparison(
  address: usize,
  computer: &mut Computer,
  comparison: mix::Comparison,
) -> () {
  conditional_jump(address, computer, computer.comparison != comparison);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_jmp() {
    let mut computer = Computer::new();
    computer.program_counter = 100;
    let instruction = mix::Instruction {
      address: mix::Address::from_value(1000),
      index_specification: 0,
      modification: 0,
      operation: mix::op_codes::JMP,
    };
    computer.memory[100].write(mix::Word::from_instruction(instruction));

    computer.fetch_decode_execute();

    assert_eq!(computer.program_counter, 1000);
    assert_eq!(computer.jump_address, mix::Address::from_value(101));
  }

  #[test]
  fn test_jsj() {
    let mut computer = Computer::new();
    computer.program_counter = 100;
    computer.jump_address = mix::Address::from_value(50);
    let instruction = mix::Instruction {
      address: mix::Address::from_value(1000),
      index_specification: 0,
      modification: 1,
      operation: mix::op_codes::JSJ,
    };
    computer.memory[100].write(mix::Word::from_instruction(instruction));

    computer.fetch_decode_execute();

    assert_eq!(computer.program_counter, 1000);
    assert_eq!(computer.jump_address, mix::Address::from_value(50));
  }

  #[test]
  fn test_jov() {
    let tests = [(true, 1000), (false, 101)];

    for (overflow, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.overflow = *overflow;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 2,
        operation: mix::op_codes::JOV,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
      assert_eq!(computer.overflow, false);
    }
  }

  #[test]
  fn test_jnov() {
    let tests = [(true, 101), (false, 1000)];

    for (overflow, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.overflow = *overflow;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::JNOV,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
      assert_eq!(computer.overflow, false);
    }
  }

  #[test]
  fn test_jl() {
    let tests = [
      (mix::Comparison::Less, 1000),
      (mix::Comparison::Equal, 101),
      (mix::Comparison::Greater, 101),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 4,
        operation: mix::op_codes::JL,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_je() {
    let tests = [
      (mix::Comparison::Less, 101),
      (mix::Comparison::Equal, 1000),
      (mix::Comparison::Greater, 101),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 5,
        operation: mix::op_codes::JE,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jg() {
    let tests = [
      (mix::Comparison::Less, 101),
      (mix::Comparison::Equal, 101),
      (mix::Comparison::Greater, 1000),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 6,
        operation: mix::op_codes::JG,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jge() {
    let tests = [
      (mix::Comparison::Less, 101),
      (mix::Comparison::Equal, 1000),
      (mix::Comparison::Greater, 1000),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 7,
        operation: mix::op_codes::JGE,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jne() {
    let tests = [
      (mix::Comparison::Less, 1000),
      (mix::Comparison::Equal, 101),
      (mix::Comparison::Greater, 1000),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 8,
        operation: mix::op_codes::JNE,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jle() {
    let tests = [
      (mix::Comparison::Less, 1000),
      (mix::Comparison::Equal, 1000),
      (mix::Comparison::Greater, 101),
    ];

    for (comparison, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.comparison = *comparison;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 9,
        operation: mix::op_codes::JLE,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jan() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 101),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 0,
        operation: mix::op_codes::JAN,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jaz() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 1000),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 1,
        operation: mix::op_codes::JAZ,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jap() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 101),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 2,
        operation: mix::op_codes::JAP,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jann() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 1000),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::JANN,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_janz() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 101),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 4,
        operation: mix::op_codes::JANZ,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_janp() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 1000),
    ];

    for (acc_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.accumulator = *acc_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 5,
        operation: mix::op_codes::JANP,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxn() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 101),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 0,
        operation: mix::op_codes::JXN,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxz() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 1000),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 1,
        operation: mix::op_codes::JXZ,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxp() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 101),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 2,
        operation: mix::op_codes::JXP,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxnn() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 101),
      (mix::Word::zero(), 1000),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::JXNN,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxnz() {
    let tests = [
      (mix::Word::from_value(5000), 1000),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 101),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 4,
        operation: mix::op_codes::JXNZ,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jxnp() {
    let tests = [
      (mix::Word::from_value(5000), 101),
      (mix::Word::from_value(-5000), 1000),
      (mix::Word::zero(), 1000),
    ];

    for (ext_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.extension = *ext_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 5,
        operation: mix::op_codes::JXNP,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jin() {
    let tests = [
      (mix::Address::from_value(5000), 101),
      (mix::Address::from_value(-5000), 1000),
      (mix::Address::zero(), 101),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[0] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 0,
        operation: mix::op_codes::J1N,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jiz() {
    let tests = [
      (mix::Address::from_value(5000), 101),
      (mix::Address::from_value(-5000), 101),
      (mix::Address::zero(), 1000),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[1] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 1,
        operation: mix::op_codes::J2Z,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jip() {
    let tests = [
      (mix::Address::from_value(5000), 1000),
      (mix::Address::from_value(-5000), 101),
      (mix::Address::zero(), 101),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[2] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 2,
        operation: mix::op_codes::J3P,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jinn() {
    let tests = [
      (mix::Address::from_value(5000), 1000),
      (mix::Address::from_value(-5000), 101),
      (mix::Address::zero(), 1000),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[3] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::J4NN,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jinz() {
    let tests = [
      (mix::Address::from_value(5000), 1000),
      (mix::Address::from_value(-5000), 1000),
      (mix::Address::zero(), 101),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[4] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 4,
        operation: mix::op_codes::J5NZ,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }

  #[test]
  fn test_jinp() {
    let tests = [
      (mix::Address::from_value(5000), 101),
      (mix::Address::from_value(-5000), 1000),
      (mix::Address::zero(), 1000),
    ];

    for (reg_before, expected_pc) in &tests {
      let mut computer = Computer::new();
      computer.program_counter = 100;
      computer.indexes[5] = *reg_before;

      let instruction = mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 5,
        operation: mix::op_codes::J6NP,
      };
      computer.memory[100].write(mix::Word::from_instruction(instruction));

      computer.fetch_decode_execute();

      assert_eq!(computer.program_counter, *expected_pc);
    }
  }
}
