use crate::computer::Computer;

use crate::operations::Operation;

pub struct ConvertToCharacters {}

impl ConvertToCharacters {
  pub fn new() -> ConvertToCharacters {
    ConvertToCharacters {}
  }
}

impl Operation for ConvertToCharacters {
  fn execute(&self, computer: &mut Computer) -> () {
    let mut value = computer.accumulator.value();
    if value < 0 {
      value = -value;
    }

    let mut extension = computer.extension.read();

    for i in 0..10 {
      let b = (value % 10 + 30) as u8;
      value /= 10;
      if i < 5 {
        extension.bytes[5 - i - 1] = b;
      } else {
        computer.accumulator.bytes[10 - i - 1] = b;
      }
    }

    computer.extension.write(extension);
  }
}
