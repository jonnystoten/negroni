use crate::computer::Computer;

use crate::mix;
use crate::operations::Operation;

pub struct ConvertToNumeric {}

impl ConvertToNumeric {
  pub fn new() -> ConvertToNumeric {
    ConvertToNumeric {}
  }
}

impl Operation for ConvertToNumeric {
  fn execute(&self, computer: &mut Computer) -> () {
    let mut result = 0;
    for i in 0..5 {
      let acc_b = computer.accumulator.bytes[5 - 1 - i];
      let ext_b = computer.extension.read().bytes[5 - 1 - i];

      result += (acc_b % 10) as isize * 10isize.pow((5 + i) as u32);
      result += (ext_b % 10) as isize * 10isize.pow(i as u32);
    }

    let sign = computer.accumulator.sign;
    if mix::Word::fits_in_word(result) {
      computer.accumulator = mix::Word::from_value(result);
    } else {
      computer.overflow = true;
      computer.accumulator = mix::Word::from_value_with_overflow(result);
    }
    computer.accumulator.sign = sign;
  }
}
