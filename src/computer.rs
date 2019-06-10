use std::fmt;

use crate::instruction::Instruction;
use crate::words;

pub struct Computer {
  pub accumulator: words::Word,
  pub memory: [words::Word; 4000],
}

impl Computer {
  pub fn new() -> Computer {
    let mut memory = [words::Word {
      bytes: [0, 0, 0, 0, 0],
      sign: words::Sign::Positive,
    }; 4000];

    memory[0] = words::Word {
      bytes: [10, 20, 0, 0, 48],
      sign: words::Sign::Positive,
    };

    Computer {
      accumulator: words::Word {
        bytes: [0, 0, 0, 0, 0],
        sign: words::Sign::Positive,
      },
      memory,
    }
  }

  pub fn fetch(&self) -> Instruction {
    let word = self.memory[0];

    Instruction::from_word(word)
  }
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\
Computer {{
  A: {:?}
}}",
      self.accumulator
    )
  }
}
