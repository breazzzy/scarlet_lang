use core::panic;
use std::{
    cell::RefCell,
    fmt::{Debug, Display, Error},
    rc::Rc,
};

use crate::{
    expression::{Expression, Symbol},
    scope::Scope,
    statement::Statement,
    token::{Literal, Token, TokenType},
};

pub struct Interpreter {
    program_scope: Scope,
}

impl Interpreter {
    pub fn interp(&mut self, stmts: Vec<Statement>) {
        self.program_scope = Scope::new(None);
        for s in stmts {
            self.interpret_statement(s);
        }
    }

    pub fn interpret_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Print(expr) => println!(
                "{}",
                self.interpret_expression(expr)
                    .expect("Expected expression. [Error on print statment]")
            ),
            Statement::Declaration(sym, expr) => self.interpret_declaration(sym, expr),
            Statement::Expression(expr) => println!("Ghost expression"),
            Statement::Assignment(sym, expr) => self.interpret_assignment(sym, expr),
            Statement::StartScope => todo!(),
            Statement::EndScope => todo!(),
            Statement::Block(stmts) => self.interpret_block(stmts),
        }
    }

    pub fn interpret_block(&mut self, stmts: Vec<Statement>) {
        //Create new scope
        self.program_scope = Scope::new(Some(Box::new(self.program_scope.clone())));
        // self.program_scope = block_scope;
        // block_scope.enclosing =
        for stmt in stmts {
            self.interpret_statement(stmt);
        }
        //End block and revert to previous scope
        if let Some(scope) = &self.program_scope.enclosing {
            self.program_scope = *scope.clone();
        } else {
            //Something has gone horribly wrong
            panic!("Scope no longer exists")
            //This should be impossible
        }
    }

    pub fn interpret_expression(&self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Binary(l, operation, r) => self.interpret_binary(l, operation, r),
            Expression::Unary(operation, ex) => Ok(self.interpret_unary(operation, ex)),
            Expression::Literal(a) => self.interpret_literal(a),
            Expression::Grouping(ex) => self.interpret_expression(*ex),
            Expression::Variable(v) => self.interpret_variable(v),
            // Expression::Assignment(sym, expr) => Ok(self.interpret_assignment(sym, expr)),
            _ => panic!("Error on interpreting expression. Unkown expression"),
        }
    }

    pub fn interpret_literal(&self, expr: Literal) -> Result<Value, String> {
        match expr {
            Literal::Str(s) => return Ok(Value::String(s)),
            Literal::Number(n) => return Ok(Value::Number(n)),
            Literal::True => return Ok(Value::Bool(true)),
            Literal::False => return Ok(Value::Bool(false)),
            Literal::Nil => return Ok(Value::Nil),
        }
    }

    fn interpret_unary(&self, operation: Token, ex: Box<Expression>) -> Value {
        let value = self
            .interpret_expression(*ex)
            .expect("Unexpected Value on unary parsing");
        match (operation.token_type, value) {
            (TokenType::Minus, Value::Number(n)) => return Value::Number(-n),
            (TokenType::Not, Value::Bool(b)) => return Value::Bool(!b),
            _ => todo!(),
        }
    }

    fn interpret_binary(
        &self,
        l: Box<Expression>,
        operation: Token,
        r: Box<Expression>,
    ) -> Result<Value, String> {
        let left = self
            .interpret_expression(*l)
            .expect("Error on binary interpret of left value");
        let right = self
            .interpret_expression(*r)
            .expect("Error on binary interpret of Right value");

        match (left, operation.token_type, right) {
            (Value::Number(l), TokenType::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), TokenType::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), TokenType::Aster, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), TokenType::Slash, Value::Number(r)) => Ok(Value::Number(l / r)),
            //Strings
            (Value::String(l), TokenType::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
            //Logic
            (Value::Bool(l),TokenType::Equality, Value::Bool(r)) => Ok(Value::Bool(l == r)),
            (Value::Bool(l),TokenType::NotEqual, Value::Bool(r)) => Ok(Value::Bool(l != r)),

            (_, _, _) => self.interpreter_error(operation, "Binary operation error."),
        }
    }

    fn interpret_declaration(&mut self, variable: Symbol, expr: Option<Expression>) {
        match expr {
            Some(expr) => match self.interpret_expression(expr) {
                Ok(v) => self.program_scope.define_var(variable, v),
                Err(_) => panic!("Error on evaluation expression of definition"),
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
        match self.program_scope.get_var(v) {
            Ok(v) => Ok(v.clone()),
            Err(err) => Err(err),
        }
    }

    pub fn new() -> Interpreter {
        Interpreter {
            program_scope: Scope::new(None),
        }
    }

    // Remove the return result to go back to normal assignment
    // Assignment is currently an expression meaning something like print x = 2; will print 2 and all assign variable x to 2;
    // When assignment is a statment it would throw an error for print x = 2; and assignment would always look like y = 3;
    fn interpret_assignment(&mut self, sym: Symbol, expr: Expression) /* -> Result<Value, String> */
    {
        self.program_scope.assign_var(
            &sym,
            self.interpret_expression(expr)
                .expect("Error interpreting expression on assignment"),
        );
    }

    fn interpreter_error(&self, tok: Token, msg: &str) -> Result<Value, String> {
        Err(format!("Intepreter Error @ {}: {}", tok.line, msg))
    }

    // pub fn interpret_grouping(&self, expr : Expr::Grouping){
    //     return self.evaluate()
    // }
}

#[derive(Clone)]
pub enum Value {
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => f.write_fmt(format_args!("{}", n)),
            Value::String(s) => f.write_fmt(format_args!("{}", s)),
            Value::Bool(b) => f.write_fmt(format_args!("{}", b)),
            Value::Nil => f.write_str("Nil"),
        }
    }
}
