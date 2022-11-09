use std::{fs, path::PathBuf, collections::HashMap};

mod expression;
mod function;
mod interpreter;
mod lexer;
mod parser;
mod scope;
mod statement;
mod token;
mod resolver;

use clap::Parser;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser as scrlt;
use resolver::Resolver;
use token::Token;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    if let Some(p) = args.path {
        read_file(p);
    } else {
        println!("Enter");
        let mut interpreter: Interpreter = Interpreter::new(HashMap::new());
        //Start REPL
        loop {
            // print!(":");
            let mut lines = String::new();
            let _ = std::io::stdin().read_line(&mut lines).unwrap();
            let mut lexer = Lexer::new(&lines);
            lexer.scan_tokens();
            let mut parser = scrlt::new(lexer.tokens);
            interpreter.interp(parser.parse().expect("Parsing failure"))
        }
    }
}

fn read_file(path: PathBuf) {
    let c = fs::read_to_string(path).expect("Couldn't read file");
    run(c);
}

fn run(src: String) {
    let mut scanner = Lexer::new(&src);
    scanner.scan_tokens();
    // scanner.tokens.into_iter().map(|x| print!("{}", x));
    let mut parser: scrlt = scrlt::new(scanner.tokens);

    let stmts = parser.parse().expect("Parsing failure");
    let mut resolver = Resolver::new();

    resolver.resolve(stmts.clone());


    let mut interpreter: Interpreter = Interpreter::new(resolver.lex_scope);


    // let mut resolver = Resolver::new(&interpreter);
    // resolver.resolve(stmts.clone());
    // interpreter = resolver.interpreter;

    interpreter.interp(stmts);
    // println!("{:?}", val);
    // let mut tokens: Vec<Token> = scanner.scanTokens();
}
