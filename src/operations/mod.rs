mod address_transfer;
mod arithmetic;
mod loading;
mod storing;

use crate::computer::Computer;

pub use address_transfer::AddressTransfer;
pub use arithmetic::Addition;
pub use loading::Load;
pub use storing::Store;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
