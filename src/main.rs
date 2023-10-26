use std::fs;
use std::env;
use crate::lexer::Lexer;

mod lexer;
mod token;


fn main() {
    let maybe_file = env::args().nth(1);
    let file = if let Some(f) = maybe_file {
        f
    } else {
        panic!("Expected a file");
    };

    let maybe_contents = fs::read_to_string(file);
    let contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could not read contents")
    };

    let mut lex = Lexer::new(contents);

    while let Some(t) = lex.next() {
        println!("{:?}", t);
    }
}
