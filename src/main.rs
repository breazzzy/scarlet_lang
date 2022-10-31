use std::{env, fs};

mod expression;
mod function;
mod interpreter;
mod lexer;
mod parser;
mod scope;
mod statement;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    read_file(path);
}

fn read_file(path: &str) {
    let c = fs::read_to_string(path).expect("Couldn't read file");
    run(c);
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
