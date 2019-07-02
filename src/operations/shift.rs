use crate::computer::Computer;

use super::Operation;
use crate::mix;

pub struct Shift<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Shift<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Shift<'a> {
    Shift { instruction }
  }
}

impl<'a> Operation for Shift<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let mut m = computer.get_indexed_address_value(self.instruction);

    if self.instruction.modification % 2 == 1 {
      // SRA, SRAX, SRC
      m = -m;
    }

    match self.instruction.modification {
      0 | 1 => {
        let bytes = computer.accumulator.bytes;
        let new_bytes = shift(bytes, m);
        computer.accumulator.bytes = new_bytes;
      }
      2 | 3 => {
        let mut bytes = [0; 10];
        for i in 0..5 {
          bytes[i] = computer.accumulator.bytes[i];
        }
        for i in 0..5 {
          bytes[5 + i] = computer.extension.read().bytes[i];
        }
        let new_bytes = shift_all(bytes, m);

        let mut acc_bytes = [0; 5];
        let mut ext_bytes = [0; 5];
        for i in 0..5 {
          acc_bytes[i] = new_bytes[i];
        }
        for i in 0..5 {
          ext_bytes[i] = new_bytes[5 + i];
        }
        computer.accumulator.bytes = acc_bytes;
        let ext = computer.extension.read();
        computer.extension.write(mix::Word {
          sign: ext.sign,
          bytes: ext_bytes,
        });
      }
      4 | 5 => {

        let mut bytes = [0; 10];
        for i in 0..5 {
          bytes[i] = computer.accumulator.bytes[i];
        }
        for i in 0..5 {
          bytes[5 + i] = computer.extension.read().bytes[i];
        }
        let new_bytes = shift_circle(bytes, m);

        let mut acc_bytes = [0; 5];
        let mut ext_bytes = [0; 5];
        for i in 0..5 {
          acc_bytes[i] = new_bytes[i];
        }
        for i in 0..5 {
          ext_bytes[i] = new_bytes[5 + i];
        }
        computer.accumulator.bytes = acc_bytes;
        let ext = computer.extension.read();
        computer.extension.write(mix::Word {
          sign: ext.sign,
          bytes: ext_bytes,
        });
      }
      _ => panic!(
        "unknown modification for shift operation: {}",
        self.instruction.modification
      ),
    };
  }
}

fn shift(bytes: [u8; 5], m: isize) -> [u8; 5] {
  let size = bytes.len();
  let mut new_bytes = [0; 5];

  for i in 0..size {
    let j = (i as isize) + m;
    if j < 0 || j >= size as isize {
      new_bytes[i] = 0;
    } else {
      new_bytes[i] = bytes[j as usize];
    }
  }

  new_bytes
}

fn shift_all(bytes: [u8; 10], m: isize) -> [u8; 10] {
  let size = bytes.len();
  let mut new_bytes = [0; 10];

  for i in 0..size {
    let j = (i as isize) + m;
    if j < 0 || j >= size as isize {
      new_bytes[i] = 0;
    } else {
      new_bytes[i] = bytes[j as usize];
    }
  }

  new_bytes
}

fn shift_circle(bytes: [u8; 10], m: isize) -> [u8; 10] {
  let size = bytes.len();
  let mut new_bytes = [0; 10];

  for i in 0..size {
    let j = (i as isize) + m % size as isize;
    if j < 0 {
      new_bytes[i] = bytes[(j + (size as isize)) as usize];
    } else if j >= size as isize {
      new_bytes[i] = bytes[(j - (size as isize)) as usize];
    } else {
      new_bytes[i] = bytes[j as usize];
    }
  }

  new_bytes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_shifts() {
    let mut computer = Computer::new();
    computer.accumulator = mix::Word {
      bytes: [1, 2, 3, 4, 5],
      sign: mix::Sign::Positive,
    };
    computer.extension.write(mix::Word {
      bytes: [6, 7, 8, 9, 0],
      sign: mix::Sign::Negative,
    });

    let tests = [
      (
        mix::Instruction {
          address: mix::Address::from_value(1),
          index_specification: 0,
          modification: 3,
          operation: mix::op_codes::SRAX,
        },
        mix::Word {
          bytes: [0, 1, 2, 3, 4],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [5, 6, 7, 8, 9],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2),
          index_specification: 0,
          modification: 0,
          operation: mix::op_codes::SLA,
        },
        mix::Word {
          bytes: [2, 3, 4, 0, 0],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [5, 6, 7, 8, 9],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(4),
          index_specification: 0,
          modification: 5,
          operation: mix::op_codes::SRC,
        },
        mix::Word {
          bytes: [6, 7, 8, 9, 2],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [3, 4, 0, 0, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(2),
          index_specification: 0,
          modification: 1,
          operation: mix::op_codes::SRA,
        },
        mix::Word {
          bytes: [0, 0, 6, 7, 8],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [3, 4, 0, 0, 5],
          sign: mix::Sign::Negative,
        },
      ),
      (
        mix::Instruction {
          address: mix::Address::from_value(501),
          index_specification: 0,
          modification: 4,
          operation: mix::op_codes::SLC,
        },
        mix::Word {
          bytes: [0, 6, 7, 8, 3],
          sign: mix::Sign::Positive,
        },
        mix::Word {
          bytes: [4, 0, 0, 5, 0],
          sign: mix::Sign::Negative,
        },
      ),
    ];

    for (instruction, expected_acc, expected_ext) in &tests {
      eprintln!("yo");
      instruction.decode().execute(&mut computer);

      assert_eq!(computer.accumulator, *expected_acc);
      assert_eq!(computer.extension.read(), *expected_ext);
    }
  }
}
