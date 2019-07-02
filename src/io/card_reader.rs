use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

use super::io_device::IoDevice;
use super::{ActualDevice, SlimComputer};

use crate::mix;

use bincode;

pub struct CardReader {
  reader: BufReader<File>,
}

impl CardReader {
  pub fn new(filename: &str) -> IoDevice {
    let file = OpenOptions::new().read(true).open(filename).unwrap();
    let reader = BufReader::new(file);

    let tape = CardReader { reader };
    IoDevice::new(Box::new(tape))
  }

  const fn block_size() -> usize {
    16
  }
}

impl ActualDevice for CardReader {
  fn read(&mut self, _computer: &SlimComputer) -> Vec<mix::Word> {
    let mut line = String::new();
    self.reader.read_line(&mut line).unwrap();
    line.pop();
    eprintln!("READ {:?}", line);

    let mut remaining = &line[..];

    let mut words = vec![];
    let mut new_s;
    for _ in 0..self.block_size() {
      if remaining.len() < 5 {
        new_s = format!("{:5}", remaining);
        remaining = &new_s;
      }
      let (left, right) = remaining.split_at(5);
      remaining = right;
      words.push(mix::Word::from_char_code(left));
    }

    words
  }

  fn write(&mut self, _words: &[mix::Word], _computer: &SlimComputer) {
    panic!("cannot write to a card reader");
  }

  fn control(&mut self, _m: isize, _computer: &SlimComputer) {
    panic!("no IOC for card reader")
  }

  fn block_size(&self) -> usize {
    CardReader::block_size()
  }
}
