use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use super::internal_device::InternalDevice;
use super::io_device::IoDevice;
use super::ActualDevice;

pub struct TapeUnit {
  file: File,
}

impl TapeUnit {
  pub fn new(filename: &str) -> IoDevice {
    let file = File::create(filename).unwrap();

    let tape = TapeUnit { file };
    InternalDevice::new(Box::new(tape))
  }
}

impl ActualDevice for TapeUnit {
  fn read(&mut self) -> Vec<u8> {
    let mut buffer = vec![0; self.block_size()];
    self.file.read(&mut buffer).unwrap();

    buffer
  }

  fn write(&mut self, bytes: &[u8]) {
    self.file.write(&bytes).unwrap();
  }

  fn control(&mut self, m: isize) {
    let to_seek = m * self.block_size() as isize;
    self.file.seek(SeekFrom::Current(to_seek as i64)).unwrap();
  }

  fn block_size(&self) -> usize {
    100
  }
}
