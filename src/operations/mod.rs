mod address_transfer;
mod load;

use crate::computer::Computer;

pub use address_transfer::AddressTransfer;
pub use load::Load;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
