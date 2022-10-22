use core::panic;
use std::fmt::{Debug, Display};

use crate::{token::{Literal, Token, TokenType}, expression::{Expression, Symbol}, statement::Statement, scope::Scope};



pub struct Interpreter{
    program_scope : Scope,
}

impl Interpreter{

    pub fn interp(&mut self, stmts : Vec<Statement>){
        self.program_scope = Scope::new();
        for s in stmts{
            self.interpret_statement(s);
        }
    }

    pub fn interpret_statement(&mut self, stmt : Statement){
        match stmt {
            Statement::Print(e) => println!("{}", self.interpret_expression(e).expect("Expected expression. [Error on print statment]")),
            Statement::Declaration(n, e) => self.interpret_declaration(n,e),
            Statement::Expression(e) => println!("Ghost expression"),
        }
    }

    pub fn interpret_expression(&self, expr : Expression) -> Result<Value, String>{
        match expr {
            Expression::Binary(l, operation, r) => Ok(self.interpret_binary(l,operation,r)),
            Expression::Unary(operation, ex) => Ok(self.interpret_unary(operation,ex)),
            Expression::Literal(a) => Ok(self.interpret_literal(a)),
            Expression::Grouping(ex) => self.interpret_expression(*ex),
            Expression::Variable(v) => self.interpret_variable(v),
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

    fn interpret_unary(&self, operation: Token, ex: Box<Expression>) -> Value {
        let value = self.interpret_expression(*ex).expect("Unexpected Value on unary parsing");
        match (operation.token_type, value) {
            (TokenType::Minus, Value::Number(n)) => return Value::Number(-n),
            (TokenType::Not, Value::Bool(b)) => return Value::Bool(!b),

            _ => todo!(),
        }
    }

    fn interpret_binary(&self, l: Box<Expression>, operation: Token, r: Box<Expression>) -> Value {
        let left = self.interpret_expression(*l).expect("Error on binary interpret of left value");
        let right = self.interpret_expression(*r).expect("Error on binary interpret of Right value");
    
    match (left, operation.token_type, right) {
        (Value::Number(l), TokenType::Plus, Value::Number(r)) => Value::Number(l + r),
        (Value::Number(l), TokenType::Minus, Value::Number(r)) => Value::Number(l - r),
        (Value::Number(l), TokenType::Aster, Value::Number(r)) => Value::Number(l * r),
        (Value::Number(l), TokenType::Slash, Value::Number(r)) => Value::Number(l / r),

        (_,_,_)=> todo!(),
    }}

    fn interpret_declaration(&mut self, variable : Symbol, expr : Option<Expression>) {
        match expr {
            Some(expr) => {
                match self.interpret_expression(expr) {
                    Ok(v) => self.program_scope.define_var(variable, v),
                    Err(_) => panic!("Error on evaluation expression of definition"),
                }
            },
            // println!("{} as {}", variable.name, self.interpret_expression(expr).expect("Error interpreting expression after declaration")),
            None => self.program_scope.define_var(variable, Value::Nil),
        }
        // match variable {
            // Expression::Variable(v) => println!("Variable declared {}", v.name),
            // _ => panic!("Interp Error: Invalid variable declaration")
        // }
    }

    fn interpret_variable(&self, v: Symbol) -> Result<Value, String> {
        match self.program_scope.get_var(v){
            Ok(v) => Ok(v.clone()),
            Err(err) => Err(err),
        }
    }

    pub fn new() -> Interpreter {
        Interpreter { program_scope: Scope::new() }
    }

    // pub fn interpret_grouping(&self, expr : Expr::Grouping){
    //     return self.evaluate()
    // }
}


#[derive(Clone)]
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

impl Display for Value{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{}",n)),
            Value::String(s) => f.write_fmt(format_args!("{}", s)),
            Value::Bool(b) => f.write_fmt(format_args!("{}", b)),
            Value::Nil => f.write_str("Nil"),
        }
    }
}