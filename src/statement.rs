use crate::expression::{
    Expression::{self},
    Symbol,
};

use std::fmt::Debug;

#[derive(Clone)]
pub enum Statement /*StatementType */ {
    // Print(Expression),                       // print x
    Expression(Expression),                  // 2+2 // x+y
    Declaration(Symbol, Option<Expression>), // let x = 2
    Assignment(Symbol, Expression),          // x = 2
    FuncDclaration(Symbol, Vec<Symbol>, Expression),
    Return(Expression),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(arg0) => f.debug_tuple("Expression").field(arg0).finish(),
            Self::Declaration(arg0, arg1) => f.debug_tuple("Declaration").field(arg0).field(arg1).finish(),
            Self::Assignment(arg0, arg1) => f.debug_tuple("Assignment").field(arg0).field(arg1).finish(),
            Self::FuncDclaration(arg0, arg1, arg2) => f.debug_tuple("FuncDclaration").field(arg0).field(arg1).field(arg2).finish(),
            Self::Return(arg0) => f.debug_tuple("Return").field(arg0).finish(),
        }
    }
}

// struct Statement{
//     line : usize,
//     stmt_type : StatementType,
// }

//Each statment is created in the parser and then interpreted by the interpreter
