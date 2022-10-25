use crate::{
    expression::{
        Expression::{self},
        Symbol,
    },
};

pub enum Statement {
    Print(Expression),                       // print x
    Expression(Expression),                  // 2+2 // x+y
    Declaration(Symbol, Option<Expression>), // let x = 2
    Assignment(Symbol, Expression),          // x = 2
    Block(Vec<Statement>),
}

//Each statment is created in the parser and then interpreted by the interpreter
