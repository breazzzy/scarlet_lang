use std::{fs, process};

mod scanner;
mod token;
mod parser;
mod interpreter;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::Token;

static mut HAD_ERROR: bool = false;


fn main() {
    println!("Hello, world!");
    read_file("test.placehold");
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
    let mut scanner = Scanner::new(&src);
    let mut interpreter : Interpreter = Interpreter {  };
    scanner.scan_tokens();
    let mut parser : Parser = Parser::new(scanner.tokens);
    let val = interpreter.interp(parser.parse()).expect("Pls");
    println!("{:?}", val);
    // let mut tokens: Vec<Token> = scanner.scanTokens();
    
}
