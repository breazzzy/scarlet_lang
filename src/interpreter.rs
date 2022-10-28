use core::panic;
use std::{
    fmt::{Debug, Display},
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
            self.interp_statement(s);
        }
    }

    pub fn interp_statement(&mut self, stmt: Statement) /*-> Result<(), String>*/{
        match stmt {
            Statement::Print(expr) => println!(
                "{}",
                self.interp_expression(expr)
                    .expect("Expected expression. [Error on print statment]")
            ),
            Statement::Declaration(sym, expr) => self.interp_declaration(sym, expr),
            Statement::Expression(expr) => println!("Interpreter: Ghost expression {:?}", expr),
            Statement::Assignment(sym, expr) => self.interp_assignment(sym, expr),
            Statement::Block(stmts) => self.interp_block(stmts),
            Statement::If(c, t, e) => self.interp_if(c,*t,e),
        }
    }

    pub fn interp_block(&mut self, stmts: Vec<Statement>) {
        //Create new scope
        self.program_scope = Scope::new(Some(Box::new(self.program_scope.clone())));
        // self.program_scope = block_scope;
        // block_scope.enclosing =
        for stmt in stmts {
            self.interp_statement(stmt);
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

    pub fn interp_expression(&self, expr: Expression) -> Result<Value, String> {
        match expr {
            Expression::Binary(l, operation, r) => self.interp_binary(l, operation, r),
            Expression::Unary(operation, ex) => Ok(self.interp_unary(operation, ex)),
            Expression::Literal(a) => self.interp_literal(a),
            Expression::Grouping(ex) => self.interp_expression(*ex),
            Expression::Variable(v) => self.interp_variable(v),
            Expression::Ternary(i, r0, r1) => self.interp_ternary(i,r0,r1),
            Expression::Logical(r, o, l) => self.interp_logical(*r,o,*l),
            // Expression::Assignment(sym, expr) => Ok(self.interpret_assignment(sym, expr)),
            // _ => panic!("Error on interpreting expression. Unkown expression"),
        }
    }

    pub fn interp_literal(&self, expr: Literal) -> Result<Value, String> {
        match expr {
            Literal::Str(s) => return Ok(Value::String(s)),
            Literal::Number(n) => return Ok(Value::Number(n)),
            Literal::True => return Ok(Value::Bool(true)),
            Literal::False => return Ok(Value::Bool(false)),
            Literal::Nil => return Ok(Value::Nil),
        }
    }

    fn interp_unary(&self, operation: Token, ex: Box<Expression>) -> Value {
        let value = self
            .interp_expression(*ex)
            .expect("Unexpected Value on unary parsing");
        match (operation.token_type, value) {
            (TokenType::Minus, Value::Number(n)) => return Value::Number(-n),
            (TokenType::Not, Value::Bool(b)) => return Value::Bool(!b),
            _ => todo!(),
        }
    }

    fn interp_binary(
        &self,
        l: Box<Expression>,
        operation: Token,
        r: Box<Expression>,
    ) -> Result<Value, String> {
        let left = self
            .interp_expression(*l)
            .expect("Error on binary interpret of left value");
        let right = self
            .interp_expression(*r)
            .expect("Error on binary interpret of Right value");

        match (left, operation.token_type, right) {
            //Numbers
            (Value::Number(l), TokenType::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), TokenType::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), TokenType::Aster, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), TokenType::Slash, Value::Number(r)) => {
                match r{
                    r if r == 0.0 => panic!("Divide by zero!"),
                    _ => Ok(Value::Number(l / r))
                }
            },
            //Strings
            (Value::String(l), TokenType::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
            (Value::String(l), TokenType::Plus, Value::Number(r)) => Ok(Value::String(l + &r.to_string())),
            //Logic
            (Value::Bool(l),TokenType::Equality, Value::Bool(r)) => Ok(Value::Bool(l == r)),
            (Value::Bool(l),TokenType::NotEqual, Value::Bool(r)) => Ok(Value::Bool(l != r)),
            //Equality
            (Value::Number(l), TokenType::Equality, Value::Number(r)) => Ok(Value::Bool(l == r)),
            (Value::Number(l), TokenType::NotEqual, Value::Number(r)) => Ok(Value::Bool(l != r)),
            (Value::Number(l), TokenType::LessEqual, Value::Number(r)) => Ok(Value::Bool(l <= r)),
            (Value::Number(l), TokenType::GreaterEqual, Value::Number(r)) => Ok(Value::Bool(l >= r)),
            (Value::Number(l), TokenType::Less, Value::Number(r)) => Ok(Value::Bool(l < r)),
            (Value::Number(l), TokenType::Greater, Value::Number(r)) => Ok(Value::Bool(l > r)),

            (_, _, _) => self.interpreter_error(operation, "Binary operation error."),
        }
    }

    fn interp_declaration(&mut self, variable: Symbol, expr: Option<Expression>){
        match expr {
            Some(expr) => match self.interp_expression(expr) {
                Ok(v) => self.program_scope.define_var(variable, v),
                Err(e) => panic!("{}",e),
            },
            // println!("{} as {}", variable.name, self.interpret_expression(expr).expect("Error interpreting expression after declaration")),
            None => self.program_scope.define_var(variable, Value::Nil),
        }
        // match variable {
        // Expression::Variable(v) => println!("Variable declared {}", v.name),
        // _ => panic!("Interp Error: Invalid variable declaration")
        // }
    }

    fn interp_variable(&self, v: Symbol) -> Result<Value, String> {
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
    fn interp_assignment(&mut self, sym: Symbol, expr: Expression) /* -> Result<Value, String> */
    {
        self.program_scope.assign_var(
            &sym,
            self.interp_expression(expr)
                .expect("Error interpreting expression on assignment"),
        );
    }

    fn interpreter_error(&self, tok: Token, msg: &str) -> Result<Value, String> {
        Err(format!("Intepreter Error @ {}: {}", tok.line, msg))
    }

    fn interp_ternary(&self, i: Box<Expression>, r0: Box<Expression>, r1: Box<Expression>) -> Result<Value, String> {
        let interp_i = self.interp_expression(*i);
        match interp_i{
            Ok(v) => {
                match  v {
                    Value::Bool(b) => {
                        match b {
                            true => return self.interp_expression(*r0),
                            false => return self.interp_expression(*r1),
                        }
                    },
                    _ => Err("First expression of Ternary expression must be boolean expression".to_string()) 
                }
            },
            Err(e) => Err(e),
        }
    }

    fn interp_if(&mut self, p: Expression, t: Statement, e: Box<Option<Statement>>) {
        match self.interp_expression(p).expect("Error interpreting if statement"){
            Value::Bool(b) => {
                match b {
                    true => self.interp_statement(t),
                    false => {
                        match *e {
                            Some(stmt) => self.interp_statement(stmt),
                            None => return,
                        }
                    },
                }
            },
            _ => panic!("Condition of if statement does not amount to boolean"),//Error expression does not amount to true false value
        }
    }

    fn interp_logical(&self, left_expr: Expression, o: Token, right_expr: Expression) -> Result<Value, String> {
        match o.token_type {
            TokenType::And => {
                match self.interp_expression(left_expr).expect("Error in logical statement"){
                    Value::Bool(v) => match v {
                        true => {
                            match self.interp_expression(right_expr).expect("Error in logical statement"){
                                Value::Bool(v) => match v{
                                    true => return Ok(Value::Bool(true)),
                                    false => return Ok(Value::Bool(false)),
                                },
                                _ => panic!("Error logical expressions should amount to bool"),
                            }
                        },
                        false => return Ok(Value::Bool(false)),
                    },
                    _ => panic!("Error logical expressions should amount to bool"),
                }
            }
            TokenType::Or => {
                let left = self.interp_expression(left_expr).expect("Error in logical statement.");
                let right = self.interp_expression(right_expr).expect("Error in logical statement.");

                match (left,right){
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
                    _ => todo!(),
                }

            }
            _ => todo!(),
        }
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
