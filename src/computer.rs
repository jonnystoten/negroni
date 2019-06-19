use std::sync::Arc;
use std::fmt;
use std::vec::Vec;

use std::sync::RwLock;

use crate::io;
use crate::mix;

pub struct MemoryCell {
  lock: RwLock<mix::Word>,
}

impl MemoryCell {
  fn new(word: mix::Word) -> MemoryCell {
    MemoryCell {
      lock: RwLock::new(word)
    }
  }
  
  pub fn read(&self) -> mix::Word {
    *self.lock.read().unwrap()
  }
  
  pub fn write(&self, word: mix::Word) {
    let mut mem = self.lock.write().unwrap();
    *mem = word;
  }
}

pub struct Memory {
  memory: Vec<MemoryCell>,
}

pub struct Computer {
  pub running: bool,
  pub program_counter: usize,
  pub accumulator: mix::Word,
  pub extension: mix::Word,
  pub indexes: [mix::Address; 6],
  pub jump_address: mix::Address,
  pub memory: Arc<Vec<MemoryCell>>,
  pub overflow: bool,
  pub comparison: mix::Comparison,
  pub io_devices: Vec<io::IoDevice>,
}

impl Computer {
  pub fn new() -> Computer {
    let raw_memory = [mix::Word {
      bytes: [0, 0, 0, 0, 0],
      sign: mix::Sign::Positive,
    }; 4000];
    
    let memory: Vec<MemoryCell> = raw_memory.iter().map(|x| MemoryCell::new(*x)).collect();
    let memory = Arc::new(memory);

    let indexes = [mix::Address {
      bytes: [0, 0],
      sign: mix::Sign::Positive,
    }; 6];

    let mut io_devices: Vec<io::IoDevice> = Vec::with_capacity(21);
    for i in 0..8 {
      io_devices.push(io::TapeUnit::new(&format!("tape{}.dat", i)));
    }

    // io_devices[3].set_busy();
    // io_devices[3]
    //   .send(io::IoMessage {
    //     operation: 37,
    //     address: 1000,
    //   })
    //   .unwrap();
    // io_devices[3].wait_ready();

    let computer = Computer {
      running: false,
      program_counter: 0,
      accumulator: mix::Word {
        bytes: [0, 0, 0, 0, 0],
        sign: mix::Sign::Positive,
      },
      extension: mix::Word {
        bytes: [0, 0, 0, 0, 0],
        sign: mix::Sign::Positive,
      },
      indexes,
      jump_address: mix::Address {
        bytes: [0, 0],
        sign: mix::Sign::Positive,
      },
      memory,
      overflow: false,
      comparison: mix::Comparison::Equal,
      io_devices,
    };
    
    for io in computer.io_devices.iter() {
      io.start(&computer);
    }
    
    computer
  }

  pub fn start(&mut self) -> () {
    self.running = true;
    while self.running {
      self.fetch_decode_execute();
      if self.program_counter >= self.memory.len() {
        self.running = false;
      }
    }
  }

  pub fn fetch_decode_execute(&mut self) -> () {
    let instruction = self.fetch();
    let operation = instruction.decode();
    operation.execute(self);
    if operation.should_increment_program_counter() {
      self.program_counter += 1;
    }
  }

  fn fetch(&self) -> mix::Instruction {
    let word = self.memory[self.program_counter].read();

    mix::Instruction::from_word(word)
  }

  pub fn get_indexed_address_value(&self, instruction: &mix::Instruction) -> isize {
    let index = instruction.index_specification as usize;
    if index > 6 {
      panic!("index spec out of range: {}", index);
    }

    let value = instruction.address.value();
    if index == 0 {
      return value;
    }

    let index_value = self.indexes[index - 1].value();
    value + index_value
  }
}

impl fmt::Debug for Computer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\
Computer {{
  PC:         {:?}
  rA:         {:?}
  rX:         {:?}
  rI1:        {:?}
  rI2:        {:?}
  rI3:        {:?}
  rI4:        {:?}
  rI5:        {:?}
  rI6:        {:?}
  rJ:         {:?}
  Overflow:   {:?}
  Comparison: {:?}
}}",
      self.program_counter,
      self.accumulator.value(),
      self.extension.value(),
      self.indexes[0].value(),
      self.indexes[1].value(),
      self.indexes[2].value(),
      self.indexes[3].value(),
      self.indexes[4].value(),
      self.indexes[5].value(),
      self.jump_address.value(),
      self.overflow,
      self.comparison,
    )
  }
}
