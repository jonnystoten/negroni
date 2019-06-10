pub mod address_transfer;

use crate::computer::Computer;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
