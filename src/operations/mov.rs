use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Move<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Move<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Move<'a> {
    Move { instruction }
  }
}

impl<'a> Operation for Move<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let src = computer.get_indexed_address_value(self.instruction) as usize;

    let dest = computer.indexes[0].value() as usize;
    let num = self.instruction.modification as usize;

    for i in 0..num {
      computer.memory[dest + i].write(computer.memory[src + i].read());
    }
    
    computer.indexes[0] = mix::Address::from_value((dest + num) as isize);
  }
}
