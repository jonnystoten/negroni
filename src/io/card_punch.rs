use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct CardPunch {
  file: File,
}

impl CardPunch {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(filename)
      .unwrap();

    let tape = CardPunch { file };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    16
  }

  fn line_size() -> usize {
    let chars = ['A'; CardPunch::block_size() + 1];
    bincode::serialized_size(&chars[..]).unwrap() as usize
  }
}

impl ActualDevice for CardPunch {
  fn read(&mut self, _computer: &SlimComputer) -> Vec<mix::Word> {
    panic!("cannot read from a card punch");
  }

  fn write(&mut self, words: &[mix::Word], computer: &SlimComputer) {
    panic!("OUT for card punch not implemented");
  }

  fn control(&mut self, _m: isize, computer: &SlimComputer) {
    panic!("IOC for card punch not implemented")
  }

  fn block_size(&self) -> usize {
    CardPunch::block_size()
  }
}
