use crate::{
    expression::{
        Expression::{self, Variable},
        Symbol,
    },
    token::Token,
};

pub enum Statement {
    Print(Expression),                       // print x
    Expression(Expression),                  // 2+2 // x+y
    Declaration(Symbol, Option<Expression>), // let x = 2
    Assignment(Symbol, Expression),          // x = 2
    Block(Vec<Statement>),
    StartScope,
    EndScope,
}

//Each statment is created in the parser and then interpreted by the interpreter
