use std::fmt::Debug;

use crate::{
    statement::Statement,
    token::{Literal, Token},
};

#[derive(Clone)]
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Logical(Box<Expression>, Token, Box<Expression>), // This is here so that logical statments can short circuit
    Unary(Token, Box<Expression>),
    Literal(Literal),
    Grouping(Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    // Assignment(Symbol, Box<Expression>),
    Primary(Symbol),                               //Variable
    Call(Box<Expression>, Token, Vec<Expression>), //Callee, args
    BlockExpr(Vec<Statement>),
    IfExpr(
        Box<Expression>,
        Box<crate::expression::Expression>,
        Box<Option<crate::expression::Expression>>,
    ),
    WhileExpr(Box<Expression>, Box<Expression>),
    BreakExpr,
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(arg0, arg1, arg2) => f
                .debug_tuple("Binary")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Logical(arg0, arg1, arg2) => f
                .debug_tuple("Logical")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Unary(arg0, arg1) => f.debug_tuple("Unary").field(arg0).field(arg1).finish(),
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Grouping(arg0) => f.debug_tuple("Grouping").field(arg0).finish(),
            Self::Ternary(arg0, arg1, arg2) => f
                .debug_tuple("Ternary")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Primary(arg0) => f.debug_tuple("Primary").field(arg0).finish(),
            Self::Call(arg0, arg1, arg2) => f
                .debug_tuple("Call")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::BlockExpr(arg0) => f.debug_tuple("BlockExpr").field(arg0).finish(),
            Self::IfExpr(arg0, arg1, arg2) => f
                .debug_tuple("IfExpr")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::WhileExpr(arg0, arg1) => {
                f.debug_tuple("WhileExpr").field(arg0).field(arg1).finish()
            }
            Self::BreakExpr => write!(f, "BreakExpr"),
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol").field("name", &self.name).finish()
    }
}
