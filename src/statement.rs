use crate::{expression::{Expression::{Variable, self}, Symbol}, token::Token};

pub enum Statement{
    Print(Expression),
    Expression(Expression),
    Declaration(Symbol, Option<Expression>),
}