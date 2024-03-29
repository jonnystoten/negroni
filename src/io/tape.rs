use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct TapeUnit {
  file: File,
}

impl TapeUnit {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filename)
      .unwrap();

    let tape = TapeUnit { file };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    100
  }

  fn word_size() -> usize {
    let words = [mix::Word::zero(); TapeUnit::block_size()];
    bincode::serialized_size(&words[..]).unwrap() as usize
  }
}

impl ActualDevice for TapeUnit {
  fn read(&mut self, _computer: &SlimComputer) -> Vec<mix::Word> {
    let mut buffer = vec![0; TapeUnit::word_size()];
    self.file.read(&mut buffer).unwrap();
    bincode::deserialize(&buffer).unwrap()
  }

  fn write(&mut self, words: &[mix::Word], _computer: &SlimComputer) {
    bincode::serialize_into(&self.file, words).unwrap();
    eprintln!("done write");
    eprintln!("new pos: {}", self.file.seek(SeekFrom::Current(0)).unwrap());
  }

  fn control(&mut self, m: isize, _computer: &SlimComputer) {
    eprintln!(
      "current pos: {}",
      self.file.seek(SeekFrom::Current(0)).unwrap()
    );
    let to_seek = m * TapeUnit::word_size() as isize;
    eprintln!("moving {} blocks, that's {} bytes", m, to_seek);
    self.file.seek(SeekFrom::Current(to_seek as i64)).unwrap();
  }

  fn block_size(&self) -> usize {
    TapeUnit::block_size()
  }
}
