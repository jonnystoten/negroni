// mod internal_device;
mod io_device;
mod card_reader;
mod card_punch;
mod disk;
mod tape;
mod line_printer;

pub use io_device::IoDevice;
pub use card_reader::CardReader;
pub use card_punch::CardPunch;
pub use disk::DiskUnit;
pub use tape::TapeUnit;
pub use line_printer::LinePrinter;

use crate::computer;
use crate::mix;
use std::sync::Arc;

pub struct IoMessage {
  pub operation: u8,
  pub address: isize,
}

pub struct SlimComputer {
  memory: Arc<Vec<computer::MemoryCell>>,
  extension: Arc<computer::MemoryCell>,
}

pub trait ActualDevice {
  fn read(&mut self, computer: &SlimComputer) -> Vec<mix::Word>;
  fn write(&mut self, bytes: &[mix::Word], computer: &SlimComputer);
  fn control(&mut self, m: isize, computer: &SlimComputer);
  fn block_size(&self) -> usize;
}
