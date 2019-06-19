// mod internal_device;
mod io_device;
mod tape;

pub use io_device::IoDevice;
pub use tape::TapeUnit;

use crate::mix;

pub struct IoMessage {
  pub operation: u8,
  pub address: isize,
}

// TODO: these should send/recv arrays of mix words
pub trait ActualDevice {
  fn read(&mut self) -> Vec<mix::Word>;
  fn write(&mut self, bytes: &[mix::Word]);
  fn control(&mut self, m: isize);
  fn block_size(&self) -> usize;
}
