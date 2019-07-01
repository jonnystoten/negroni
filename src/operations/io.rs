use crate::computer::Computer;

use super::Operation;

use crate::io;
use crate::mix;

pub struct Io<'a> {
  instruction: &'a mix::Instruction,
}

impl<'a> Io<'a> {
  pub fn new(instruction: &'a mix::Instruction) -> Io<'a> {
    Io { instruction }
  }
}

impl<'a> Operation for Io<'a> {
  fn execute(&self, computer: &mut Computer) -> () {
    let address = computer.get_indexed_address_value(self.instruction);
    let device = &computer.io_devices[self.instruction.modification as usize];

    device.wait_ready();
    device.set_busy();
    device
      .send(io::IoMessage {
        operation: self.instruction.operation,
        address,
      })
      .unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tape_roundtrip() {
    let mut computer = Computer::new();

    for i in 0..100 {
      computer.memory[1000 + i].write(mix::Word::from_value(i as isize));
    }

    let instructions = [
      mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::OUT,
      },
      mix::Instruction {
        address: mix::Address::from_value(-1),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::IOC,
      },
      mix::Instruction {
        address: mix::Address::from_value(2000),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::IN,
      },
      mix::Instruction {
        address: mix::Address::from_value(3),
        index_specification: 0,
        modification: 3,
        operation: mix::op_codes::JBUS,
      },
      mix::Instruction {
        address: mix::Address::zero(),
        index_specification: 0,
        modification: 2,
        operation: mix::op_codes::HLT,
      },
    ];

    for (i, instruction) in instructions.iter().enumerate() {
      computer.memory[i].write(mix::Word::from_instruction(instruction));
    }
    computer.start();

    for i in 0..100 {
      assert_eq!(
        computer.memory[2000 + i].read(),
        mix::Word::from_value(i as isize)
      );
    }
  }

  #[test]
  fn test_disk_roundtrip() {
    let mut computer = Computer::new();

    for i in 0..100 {
      computer.memory[1000 + i].write(mix::Word::from_value(i as isize));
    }

    computer.extension.write(mix::Word::from_value(500));

    let instructions = [
      mix::Instruction {
        address: mix::Address::from_value(1000),
        index_specification: 0,
        modification: 11,
        operation: mix::op_codes::OUT,
      },
      mix::Instruction {
        address: mix::Address::from_value(2000),
        index_specification: 0,
        modification: 11,
        operation: mix::op_codes::IN,
      },
    ];

    for instruction in instructions.iter() {
      instruction.decode().execute(&mut computer);
    }

    // TODO: replace this with a JBUS when it's implemented
    for io in computer.io_devices {
      io.wait_ready();
    }

    for i in 0..100 {
      assert_eq!(
        computer.memory[2000 + i].read(),
        mix::Word::from_value(i as isize)
      );
    }
  }
}
