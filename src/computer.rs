use std::fmt;

use crate::mix;

pub struct Computer {
  pub accumulator: mix::Word,
  pub extension: mix::Word,
  pub indexes: [mix::Address; 6],
  pub memory: [mix::Word; 4000],
}

impl Computer {
  pub fn new() -> Computer {
    let memory = [mix::Word {
      bytes: [0, 0, 0, 0, 0],
      sign: mix::Sign::Positive,
    }; 4000];

    let indexes = [mix::Address {
      bytes: [0, 0],
      sign: mix::Sign::Positive,
    }; 6];

    Computer {
      accumulator: mix::Word {
        bytes: [0, 0, 0, 0, 0],
        sign: mix::Sign::Positive,
      },
      extension: mix::Word {
        bytes: [0, 0, 0, 0, 0],
        sign: mix::Sign::Positive,
      },
      indexes,
      memory,
    }
  }

  pub fn fetch(&self) -> mix::Instruction {
    let word = self.memory[0];

    mix::Instruction::from_word(word)
  }
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\
Computer {{
  A:  {:?}
  X:  {:?}
  I1: {:?}
  I2: {:?}
  I3: {:?}
  I4: {:?}
  I5: {:?}
  I6: {:?}
}}",
      self.accumulator.value(),
      self.extension.value(),
      self.indexes[0].value(),
      self.indexes[1].value(),
      self.indexes[2].value(),
      self.indexes[3].value(),
      self.indexes[4].value(),
      self.indexes[5].value(),
    )
  }
}
