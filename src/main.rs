mod computer;

mod mix;
mod operations;

fn main() {
    let mut computer = computer::Computer::new();

    // set the first instruction manually for now
    computer.memory[0] = mix::Word {
        bytes: [10, 20, 0, 0, 48],
        sign: mix::Sign::Positive,
    };

    let instruction = computer.fetch();
    let operation = instruction.decode();
    operation.execute(&mut computer);

    println!("===MIX COMPUTER===");
    println!("{:?}", computer);
}
