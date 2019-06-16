mod address_transfer;
mod arithmetic;
mod comparison;
mod loading;
mod misc;
mod storing;

use crate::computer::Computer;

pub use address_transfer::Enter;
pub use address_transfer::Increase;
pub use arithmetic::Addition;
pub use arithmetic::Division;
pub use arithmetic::Multiplication;
pub use comparison::Compare;
pub use loading::Load;
pub use misc::NoOp;
pub use storing::Store;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
