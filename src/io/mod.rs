// mod internal_device;
mod io_device;
mod disk;
mod tape;

pub use io_device::IoDevice;
pub use disk::DiskUnit;
pub use tape::TapeUnit;

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
