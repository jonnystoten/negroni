mod computer;

mod io;
mod mix;
mod operations;

fn main() {
    let mut computer = computer::Computer::new();

    // set the first instruction manually for now
    computer.memory[0].write(mix::Word {
        bytes: [10, 20, 0, 0, 48],
        sign: mix::Sign::Positive,
    });

    computer.start();

    println!("===MIX COMPUTER===");
    println!("{:?}", computer);
}
