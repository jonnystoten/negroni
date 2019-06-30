use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct LinePrinter {
  file: File,
}

impl LinePrinter {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(filename)
      .unwrap();

    let tape = LinePrinter { file };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    24
  }

  fn line_size() -> usize {
    let chars = ['A'; LinePrinter::block_size() + 1];
    bincode::serialized_size(&chars[..]).unwrap() as usize
  }
}

impl ActualDevice for LinePrinter {
  fn read(&mut self, computer: &SlimComputer) -> Vec<mix::Word> {
    panic!("cannot read from a line printer");
  }

  fn write(&mut self, words: &[mix::Word], computer: &SlimComputer) {
    let mut line = String::new();
    for word in words.iter() {
      let word_str = word.to_char_code();
      line.push_str(&word_str);
    }
    line.push('\n');

    self.file.write(line.as_bytes()).unwrap();
  }

  fn control(&mut self, _m: isize, computer: &SlimComputer) {
    // panic!("IOC for line printer not implemented")
  }

  fn block_size(&self) -> usize {
    LinePrinter::block_size()
  }
}
