use std::env;
use std::io;
use std::process;

#[derive(Debug)]
enum Pattern {
    SingleChar(char),
    Digit,
    Alphanumeric,
}

fn match_pattern(input_line: &str, pattern: Pattern) -> bool {
    match pattern {
        Pattern::SingleChar(c) => input_line.contains(c),
        Pattern::Digit => input_line.chars().any(|c| c.is_numeric()),
        Pattern::Alphanumeric => input_line.chars().all(|c| c.is_alphanumeric()),
    }
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern_arg = env::args().nth(2).unwrap();
    let pattern = match pattern_arg.as_str() {
        r"\d" => Pattern::Digit,
        r"\w" => Pattern::Alphanumeric,
        s => {
            if s.chars().count() == 1 {
                Pattern::SingleChar(s.chars().next().unwrap())
            } else {
                panic!("Unhandled pattern: {}", s)
            }
        }
    };

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line.trim(), pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
