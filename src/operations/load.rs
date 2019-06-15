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

    let instruction = mix::Instruction {
      address: mix::Address::new(2000),
      index_specification: 0,
      modification: mix::field_spec(0, 5),
      operation: 8,
    };

    instruction.decode().execute(&mut computer);

    assert_eq!(
      computer.accumulator,
      mix::Word {
        bytes: [1, 14, 3, 5, 4],
        sign: mix::Sign::Negative,
      }
    );
  }
}
