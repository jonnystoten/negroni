mod address_transfer;
mod arithmetic;
mod loading;
mod storing;

use crate::computer::Computer;

pub use address_transfer::Enter;
pub use address_transfer::Increase;
pub use arithmetic::Addition;
pub use arithmetic::Multiplication;
pub use arithmetic::Division;
pub use loading::Load;
pub use storing::Store;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
