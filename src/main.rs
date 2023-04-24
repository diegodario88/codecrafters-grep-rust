use std::env;
use std::io;
use std::process;

#[derive(Debug)]
enum Pattern {
    SingleChar(char),
    Digit,
    Alphanumeric,
    Group(String),
}

fn match_pattern(input_line: &str, pattern: Pattern) -> bool {
    match pattern {
        Pattern::SingleChar(c) => input_line.contains(c),
        Pattern::Digit => input_line.chars().any(|c| c.is_numeric()),
        Pattern::Alphanumeric => input_line.chars().all(|c| c.is_alphanumeric()),
        Pattern::Group(s) => {
            let group_without_brackets = s.trim_start_matches('[').trim_end_matches(']');

            // Handle negative character groups (denoted by a leading '^' character).
            // If the group is negative, then return true only if the input_line does not contain any of the characters in the group.
            if group_without_brackets.starts_with('^') {
                group_without_brackets
                    .chars()
                    .all(|c| !input_line.contains(c))
            }
            // Otherwise, return true only if the input_line contains at least one character in the group.
            else {
                group_without_brackets
                    .chars()
                    .any(|c| input_line.contains(c))
            }
        }
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
        s if s.chars().count() == 1 => Pattern::SingleChar(s.chars().next().unwrap()),
        s if s.starts_with("[") && s.ends_with("]") => Pattern::Group(String::from(s)),
        s => panic!("Unhandled pattern: {}", s),
    };

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line.trim(), pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_pattern() {
        assert!(match_pattern("apple", Pattern::SingleChar('a')));
        assert!(!match_pattern("apple", Pattern::SingleChar('b')));

        assert!(match_pattern("123", Pattern::Digit));
        assert!(!match_pattern("abc", Pattern::Digit));

        assert!(match_pattern("apple123", Pattern::Alphanumeric));
        assert!(!match_pattern("apple.123", Pattern::Alphanumeric));

        assert!(match_pattern("apple", Pattern::Group(String::from("abc"))));
        assert!(!match_pattern("dog", Pattern::Group(String::from("abc"))));

        assert!(match_pattern("dog", Pattern::Group(String::from("^abc"))));
        assert!(!match_pattern("cab", Pattern::Group(String::from("^abc"))));
    }
}
