use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct CardReader {
  file: File,
}

impl CardReader {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(filename)
      .unwrap();

    let tape = CardReader { file };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    16
  }

  fn line_size() -> usize {
    let chars = ['A'; CardReader::block_size() + 1];
    bincode::serialized_size(&chars[..]).unwrap() as usize
  }
}

impl ActualDevice for CardReader {
  fn read(&mut self, computer: &SlimComputer) -> Vec<mix::Word> {
    panic!("IN for card reader not implemented");
  }

  fn write(&mut self, words: &[mix::Word], computer: &SlimComputer) {
    panic!("cannot write to a card reader");
  }

  fn control(&mut self, _m: isize, computer: &SlimComputer) {
    panic!("IOC for card reader not implemented")
  }

  fn block_size(&self) -> usize {
    CardReader::block_size()
  }
}
