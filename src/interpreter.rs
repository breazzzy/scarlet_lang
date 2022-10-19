use std::fmt::Debug;

use crate::{parser::Expr, token::{Literal, Token, TokenType}};



pub struct Interpreter{

}

impl Interpreter{

    pub fn interp(&self, expr : Expr) -> Result<Value, String>{
        match expr {
            Expr::Binary(l, operation, r) => Ok(self.interpret_binary(l,operation,r)),
            Expr::Unary(operation, ex) => Ok(self.interpret_unary(operation,ex)),
            Expr::Literal(a) => Ok(self.interpret_literal(a)),
            Expr::Grouping(ex) => self.interp(*ex),
        }
    }

    pub fn interpret_literal(&self, expr : Literal) -> Value{
        match expr{
            Literal::Identifier(_) => todo!(),
            Literal::Str(s) => return Value::String(s),
            Literal::Number(n) => return Value::Number(n),
            Literal::True => return Value::Bool(true),
            Literal::False => return Value::Bool(false),
            Literal::Nil => return Value::Nil,
        }
    }

    fn interpret_unary(&self, operation: Token, ex: Box<Expr>) -> Value {
        let value = self.interp(*ex).expect("Unexpected Value on unary parsing");
        match (operation.token_type, value) {
            (TokenType::Minus, Value::Number(n)) => return Value::Number(-n),
            (TokenType::Not, Value::Bool(b)) => return Value::Bool(!b),

            _ => todo!(),
        }
    }

    fn interpret_binary(&self, l: Box<Expr>, operation: Token, r: Box<Expr>) -> Value {
        let left = self.interp(*l).expect("Error on binary interpret of left value");
        let right = self.interp(*r).expect("Error on binary interpret of Right value");
    
    match (left, operation.token_type, right) {
        (Value::Number(l), TokenType::Plus, Value::Number(r)) => Value::Number(l + r),
        (Value::Number(l), TokenType::Minus, Value::Number(r)) => Value::Number(l - r),
        (Value::Number(l), TokenType::Aster, Value::Number(r)) => Value::Number(l * r),
        (Value::Number(l), TokenType::Slash, Value::Number(r)) => Value::Number(l / r),

        (_,_,_)=> todo!(),
    }}

    // pub fn interpret_grouping(&self, expr : Expr::Grouping){
    //     return self.evaluate()
    // }
}

pub enum Value{
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Nil => write!(f, "Nil"),
        }
    }
}