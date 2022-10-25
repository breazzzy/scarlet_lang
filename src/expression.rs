use std::fmt::Debug;

use crate::token::{Literal, Token};

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Literal(Literal),
    Grouping(Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    // Assignment(Symbol, Box<Expression>),
    Variable(Symbol),
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(arg0, arg1, arg2) => f.debug_tuple("Binary").field(arg0).field(arg1).field(arg2).finish(),
            Self::Unary(arg0, arg1) => f.debug_tuple("Unary").field(arg0).field(arg1).finish(),
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Grouping(arg0) => f.debug_tuple("Grouping").field(arg0).finish(),
            Self::Ternary(arg0, arg1, arg2) => f.debug_tuple("Ternary").field(arg0).field(arg1).field(arg2).finish(),
            Self::Variable(arg0) => f.debug_tuple("Variable").field(arg0).finish(),
        }
    }
}

pub struct Symbol {
    pub name: String,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol").field("name", &self.name).finish()
    }
}
