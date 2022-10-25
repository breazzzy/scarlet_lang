use std::{fs, process};

mod expression;
mod interpreter;
mod parser;
mod lexer;
mod scope;
mod statement;
mod token;

use interpreter::Interpreter;
use parser::Parser;
use lexer::Lexer;
use token::Token;

static mut HAD_ERROR: bool = false;

fn main() {
    read_file("test.scarlet");
}

fn read_file(path: &str) {
    let c = fs::read_to_string(path).expect("Couldn't read file");
    run(c);
}

fn error(line: i32, msg: String) {
    println!("[Line {line}] error");
    unsafe {
        HAD_ERROR = true;
    }
}

fn run(src: String) {
    let mut scanner = Lexer::new(&src);
    let mut interpreter: Interpreter = Interpreter::new();
    scanner.scan_tokens();
    // scanner.tokens.into_iter().map(|x| print!("{}", x));
    let mut parser: Parser = Parser::new(scanner.tokens);
    interpreter.interp(parser.parse().expect("Parsing failure"));
    // println!("{:?}", val);
    // let mut tokens: Vec<Token> = scanner.scanTokens();
}
