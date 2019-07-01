use std::fmt::Write;
use std::fs::File;
use std::io::Read;

use bincode;
use clap::App;

use negroni::mix;
use negroni::mixal;

fn main() {
    let matches = App::new("shake")
        .version("0.1")
        .author("Jonny Stoten <jonny@jonnystoten.com>")
        .about("Assembler for MIXAL")
        .args_from_usage(
            "--format=<FORMAT> 'Sets the output format'
             <INPUT>           'Sets the input file to use'
             --debug           'Enables debug output'",
        )
        .get_matches();

    let format = matches.value_of("format").unwrap();
    let filename = matches.value_of("INPUT").unwrap();
    let debug = matches.is_present("debug");

    println!("===SHAKE===");

    let mut file = File::open(filename).unwrap();
    let input = &mut String::new();
    file.read_to_string(input).unwrap();

    if debug {
        lex(input);
    }
    assemble(input, format, debug);
}

fn assemble(input: &String, format: &str, debug: bool) {
    let program = parse(input);

    let mut assembler = mixal::Assembler::new();
    assembler.assemble(program).unwrap();

    if debug {
        let mut words: Vec<(&usize, &mix::Word)> = assembler.words.iter().collect();
        words.sort_by_key(|x| x.0);
        for word in words {
            println!("{:?}", word);
        }
    }

    match format {
        "binary" => {
            let output_file = File::create("out.bin").unwrap();
            bincode::serialize_into(&output_file, &(&assembler.words, &assembler.program_start))
                .unwrap();
            // bincode::serialize_into(&output_file, &assembler.program_start).unwrap();
        }
        "deck" => {
            let mut locations: Vec<&usize> = assembler.words.keys().collect();
            locations.sort();

            let groups = make_groups(locations);
            {
                use std::io::Write;
                let mut stdout = std::io::stdout();
                write!(stdout, "{}\n", loader()).unwrap();
            }
            for group in groups {
                let mut card = String::new();
                write!(card, "SHAKE{}{:04}", group.len(), group[0]).unwrap();
                for location in group {
                    let word = assembler.words[location];
                    let value = word.value();
                    if value >= 0 {
                        write!(card, "{:010}", value).unwrap();
                    } else {
                        let value = -value;
                        write!(card, "{:09}", value / 10).unwrap();
                        let lsb = value % 10;
                        let ch = mix::char_codes::get_char(&((lsb + 10) as u8));
                        write!(card, "{}", ch).unwrap();
                    }
                }
                {
                    use std::io::Write;
                    let mut stdout = std::io::stdout();
                    write!(stdout, "{}\n", card).unwrap();
                }
            }

            {
                use std::io::Write;
                let mut stdout = std::io::stdout();
                write!(stdout, "TRANS0{:04}\n", assembler.program_start).unwrap();
            }
        }
        _ => panic!("unknown format"),
    };
}

fn make_groups(locations: Vec<&usize>) -> Vec<Vec<&usize>> {
    let mut groups = vec![];
    let mut group = vec![];
    let mut last_loc = *locations.first().unwrap() - 1;
    for loc in locations {
        if *loc != last_loc + 1 || group.len() == 7 {
            groups.push(group);
            group = vec![];
        }
        group.push(loc);
        last_loc = *loc;
    }
    groups.push(group);
    groups
}

fn parse(input: &String) -> mixal::Program {
    let mut parser = mixal::Parser::new(input);
    match parser.parse() {
        Ok(program) => program,
        Err(err) => panic!(err),
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

fn loader() -> &'static str {
    // TODO: load this from disk
    " O O6 2 O6    I C O4 3 EH A  F F CF    E   EU 3 IH Z EB   EJ  CA. 2 EU   EH 0 EA
   EU 5A-H Z EB  C U 4AEH 5AEN    E  CLU  ABG 2 EH 0 EB J B. A  9    0    A"
}
