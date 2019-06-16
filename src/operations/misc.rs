use crate::computer::Computer;

use super::Operation;

pub struct NoOp {}

impl NoOp {
  pub fn new() -> NoOp {
    NoOp {}
  }
}

impl Operation for NoOp {
  fn execute(&self, _computer: &mut Computer) -> () {}
}
