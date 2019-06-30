use std::env;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use std::collections::HashMap;

use bincode;

use negroni::computer;
use negroni::mix;

fn main() {
    let args: Vec<String> = env::args().collect();

    let interactive = &args[1] == "--interactive";

    let mut computer = computer::Computer::new();

    let mut input_file = File::open("out.bin").unwrap();
    let size = {
        let result = input_file.seek(SeekFrom::End(0)).unwrap();
        input_file.seek(SeekFrom::Start(0)).unwrap();
        result
    };

    let mut buffer = vec![0; size as usize];
    input_file.read(&mut buffer).unwrap();
    let (words, program_start): (HashMap<usize, mix::Word>, usize) =
        bincode::deserialize(&buffer).unwrap();

    for (address, word) in words {
        computer.memory[address].write(word);
    }

    println!("Setting PC to {}", program_start);
    computer.program_counter = program_start;

    if interactive {
        computer.start_interactive(|computer| {
            println!("===MIX COMPUTER===");
            println!("{:?}", computer);
            let mut stdin = std::io::stdin();
            stdin.read(&mut [0u8]).unwrap();
        });
    } else {
        computer.start();
    }

    for io_device in &computer.io_devices {
        io_device.wait_ready();
    }

    println!("===MIX COMPUTER===");
    println!("{:?}", computer);
}
