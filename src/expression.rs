use crate::token::{Token, Literal};

pub enum Expression{
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Literal(Literal),
    Grouping(Box<Expression>),
    // Assignment(Symbol, Box<Expression>),
    Variable(Symbol),
}

pub struct Symbol{
    pub name: String,
}