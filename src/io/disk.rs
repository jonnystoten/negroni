use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct DiskUnit {
  file: File,
}

impl DiskUnit {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filename)
      .unwrap();

    let tape = DiskUnit { file };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    100
  }

  fn word_size() -> usize {
    let words = [mix::Word::zero(); DiskUnit::block_size()];
    bincode::serialized_size(&words[..]).unwrap() as usize
  }
}

impl ActualDevice for DiskUnit {
  fn read(&mut self, computer: &SlimComputer) -> Vec<mix::Word> {
    let block = computer.extension.read().value();
    self
      .file
      .seek(SeekFrom::Start(block as u64 * DiskUnit::word_size() as u64))
      .unwrap();

    let mut buffer = vec![0; DiskUnit::word_size()];
    self.file.read(&mut buffer).unwrap();
    bincode::deserialize(&buffer).unwrap()
  }

  fn write(&mut self, words: &[mix::Word], computer: &SlimComputer) {
    let block = computer.extension.read().value();
    self
      .file
      .seek(SeekFrom::Start(block as u64 * DiskUnit::word_size() as u64))
      .unwrap();

    bincode::serialize_into(&self.file, words).unwrap();
    println!("done write");
    println!("new pos: {}", self.file.seek(SeekFrom::Current(0)).unwrap());
  }

  fn control(&mut self, _m: isize, computer: &SlimComputer) {
    // TODO: does this actually save time later?
    let block = computer.extension.read().value();
    self
      .file
      .seek(SeekFrom::Start(block as u64 * DiskUnit::word_size() as u64))
      .unwrap();
  }

  fn block_size(&self) -> usize {
    DiskUnit::block_size()
  }
}
