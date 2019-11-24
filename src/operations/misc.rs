use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct NoOp {}

impl NoOp {
  pub fn new() -> NoOp {
    NoOp {}
  }
}

impl Operation for NoOp {
  fn execute(&self, _computer: &mut Computer) -> () {}
}

pub struct Halt<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Halt<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Halt<'a> {
    Halt {instruction}
  }
}

impl<'a> Operation for Halt<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    if self.instruction.address.value() != 0 {
      panic!("HLT with code {}", self.instruction.address.value());
    }
    computer.running = false;
  }
}
