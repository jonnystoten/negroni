use std::env;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use std::collections::HashMap;

use bincode;
use clap::App;

use negroni::computer;
use negroni::mix;

fn main() {
    let matches = App::new("stir")
        .version("0.1")
        .author("Jonny Stoten <jonny@jonnystoten.com>")
        .about("Emulator for MIX")
        .args_from_usage(
            "--format=<FORMAT> 'Sets the input format'
             [INPUT]           'Sets the input file to use'
             --interactive     'Enables interactive debugger'",
        )
        .get_matches();


    let interactive = matches.is_present("interactive");
    let format = matches.value_of("format").unwrap();

    let mut computer = computer::Computer::new();

    match format {
        "binary" => {
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

            eprintln!("Setting PC to {}", program_start);
            computer.program_counter = program_start;
        }
        "deck" => {
            let instruction = mix::Instruction {
                operation: mix::op_codes::IN,
                modification: 16,
                address: mix::Address::zero(),
                index_specification: 0,
            };
            let operation = instruction.decode();
            operation.execute(&mut computer);
            for io_device in &computer.io_devices {
                io_device.wait_ready();
            }
            computer.program_counter = 0;
            computer.jump_address = mix::Address::zero();
        }
        _ => panic!("unknown format"),
    }


    if interactive {
        computer.start_interactive(|computer| {
            if computer.program_counter < 100 {
                // don't step through the loader
                return;
            }
            eprintln!("===MIX COMPUTER===");
            eprintln!("{:?}", computer);
            let mut stdin = std::io::stdin();
            stdin.read(&mut [0u8]).unwrap();
        });
    } else {
        computer.start();
    }

    for io_device in &computer.io_devices {
        io_device.wait_ready();
    }

    eprintln!("===MIX COMPUTER===");
    eprintln!("{:?}", computer);
}
