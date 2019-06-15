mod address_transfer;
mod loading;

use crate::computer::Computer;

pub use address_transfer::AddressTransfer;
pub use loading::Load;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();
}
