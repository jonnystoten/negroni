use std::env;

use std::fs::File;
use std::io::Read;

use bincode;

use negroni::mix;
use negroni::mixal;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("===SHAKE===");

    let mut file = File::open(filename).unwrap();
    let input = &mut String::new();
    file.read_to_string(input).unwrap();

    lex(input);
    parse(input);
}

fn parse(input: &String) {
    let mut parser = mixal::Parser::new(input);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(err) => panic!(err),
    };

    let mut assembler = mixal::Assembler::new();
    assembler.assemble(program).unwrap();

    let output_file = File::create("out.bin").unwrap();
    bincode::serialize_into(&output_file, &(&assembler.words, &assembler.program_start)).unwrap();
    // bincode::serialize_into(&output_file, &assembler.program_start).unwrap();

    let mut words: Vec<(&usize, &mix::Word)> = assembler.words.iter().collect();
    words.sort_by_key(|x| x.0);
    for word in words {
        println!("{:?}", word);
    }
}

fn lex(input: &String) {
    let mut debug = String::new();
    let mut lexer = mixal::Lexer::new(input);
    loop {
        let lexeme = lexer.scan();
        // println!("{:?}", lexeme);

        if lexeme.token == mixal::Token::ILLEGAL {
            println!(
                "ERROR: unexpected token {} ({}:{})",
                lexeme.literal, lexeme.line, lexeme.col
            );
            break;
        }

        if lexeme.token == mixal::Token::EOF {
            println!("[EOF]");
            break;
        }

        if lexeme.token == mixal::Token::EOL {
            println!("{}[EOL]", debug);
            debug = String::new();
        } else {
            debug = format!("{}[{}]", debug, lexeme.literal);
        }
    }
}
