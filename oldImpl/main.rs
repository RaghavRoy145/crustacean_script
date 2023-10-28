use std::fs;
use std::env;

mod lexer;
mod token;
mod parser;
mod ast;


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

    let mut lex = lexer::Lexer::new(contents);
    let mut p = parser::Parser::new(lex);
    let prog = p.parse();
    println!("{:?}", prog)
}
