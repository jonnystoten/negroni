mod address_transfer;
mod arithmetic;
mod comparison;
mod conversion;
mod io;
mod jump;
mod loading;
mod misc;
mod storing;

use crate::computer::Computer;

pub use address_transfer::{Enter, Increase};
pub use arithmetic::{Addition, Division, Multiplication};
pub use comparison::Compare;
pub use conversion::{ConvertToCharacters, ConvertToNumeric};
pub use io::Io;
pub use jump::{IoJump, Jump, RegisterJump};
pub use loading::Load;
pub use misc::{Halt, NoOp};
pub use storing::Store;

pub trait Operation {
  fn execute(&self, computer: &mut Computer) -> ();

  fn should_increment_program_counter(&self) -> bool {
    true
  }
}
