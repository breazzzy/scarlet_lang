use core::panic;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::{
    expression::{Expression, Symbol},
    function::{Callable, Function, NativeFunction},
    scope::Scope,
    statement::Statement,
    token::{Literal, Token, TokenType},
};

#[derive(Clone)]
pub struct Interpreter {
    pub program_scope: Scope,      //Scope currently being used by interpreter
    pub return_val: Option<Value>, //Current return value
    /*Hash Map of all user defined functions
    This is used to get closures to work. Each function has an id, when the function is called it
    changes the value its id correlates to in this map.  */
    pub function_map: HashMap<u64, Function>,
    pub f_count: u64, //Counter for next function id.
    pub global: Scope,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut global_map: HashMap<String, Value> = HashMap::new();
        global_map.insert(
            "pow".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "pow".to_string(),
                arity: 2,
                callable: |_, args| match (args[0].clone(), args[1].clone()) {
                    (Value::Number(base), Value::Number(pow)) => Ok(Value::Number(base.powf(pow))),
                    (_, _) => panic!("Pow function can only take numbers as arguments"),
                },
            }),
        );

        global_map.insert(
            "min".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "min".to_string(),
                arity: 2,
                callable: |_, args| match (args[0].clone(), args[1].clone()) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f64::min(a, b))),
                    (_, _) => panic!("Min function can only take numbers as arguments"),
                },
            }),
        );
        global_map.insert(
            "max".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "max".to_string(),
                arity: 2,
                callable: |_, args| match (args[0].clone(), args[1].clone()) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(f64::max(a, b))),
                    (_, _) => panic!("Max function can only take numbers as arguments"),
                },
            }),
        );
        global_map.insert(
            "abs".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "abs".to_string(),
                arity: 1,
                callable: |_, args| match args[0].clone() {
                    Value::Number(a) => Ok(Value::Number(a.abs())),
                    _ => panic!("Abs function can only take a number as an argument"),
                },
            }),
        );
        global_map.insert(
            "print".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "print".to_string(),
                arity: 1,
                callable: |_, args| {
                    print!("{}", args[0].clone());
                    return Ok(Value::Nil);
                },
            }),
        );
        global_map.insert(
            "println".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "println".to_string(),
                arity: 1,
                callable: |_, args| {
                    println!("{}", args[0].clone());
                    return Ok(Value::Nil);
                },
            }),
        );
        let scope = Scope::new(None);
        let mut global = Scope::new(None);
        global.load(global_map);
        // scope.load(global);
        Interpreter {
            program_scope: scope,
            return_val: None,
            function_map: HashMap::new(),
            f_count: 0,
            global: global,
        }
    }

    pub fn interp(&mut self, stmts: Vec<Statement>) {
        // self.program_scope = Scope::new(None);
        for s in stmts {
            match self.interp_statement(s) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
        }
    }

    pub fn interp_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Declaration(sym, expr) => self.interp_declaration(sym, expr),
            Statement::Expression(expr) => {
                let _ = self.interp_expression(expr)?;
                Ok(())
            }
            Statement::Assignment(sym, expr) => self.interp_assignment(sym, expr),
            Statement::FuncDclaration(name, params, body) => {
                self.interp_funcdecl(name, params, body)
            }
            Statement::Return(expr) => {
                self.return_val = Some(self.interp_expression(expr)?);
                Ok(())
            }
            // Statement::Block(stmts) => self.interp_block(stmts),
            // Statement::While(condition, body) => self.interp_while(condition, body),
        }
    }

    pub fn interp_expression(&mut self, expr: Expression) -> Result<Value, String> {
        // println!("{:?}", expr);
        match expr {
            Expression::BreakExpr => {
                return Ok(Value::Break);
            }
            Expression::ContinueExpr => {
                println!("cont");
                return Ok(Value::Continue);
            }
            Expression::Binary(l, operation, r) => self.interp_binary(l, operation, r),
            Expression::Unary(operation, ex) => self.interp_unary(operation, ex),
            Expression::Literal(a) => self.interp_literal(a),
            Expression::Grouping(ex) => self.interp_expression(*ex),
            Expression::Primary(v) => self.interp_variable(v),
            Expression::Ternary(i, r0, r1) => self.interp_ternary(i, r0, r1),
            Expression::Logical(r, o, l) => self.interp_logical(*r, o, *l),
            Expression::Call(callee, t, args) => self.interp_call(callee, t, args),
            Expression::BlockExpr(stmts) => self.interp_blockexpr(stmts),
            Expression::IfExpr(conditon, then, elses) => self.inetrp_ifexpr(conditon, then, elses),
            Expression::WhileExpr(conditon, body) => self.interp_whileexpr(conditon, body),
            Expression::LoopExpr(body) => self.interp_loopexpr(body),
            // Expression::BreakExpr() => Ok(Value::Break),
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

    fn interp_unary(&mut self, operation: Token, ex: Box<Expression>) -> Result<Value, String> {
        let value = self
            .interp_expression(*ex)
            .expect("Unexpected Value on unary parsing");
        match (operation.token_type, value) {
            (TokenType::Minus, Value::Number(n)) => return Ok(Value::Number(-n)),
            (TokenType::Not, Value::Bool(b)) => return Ok(Value::Bool(!b)),
            _ => Err("Unexpected unary token. (Only ! and - accepted)".to_string()),
        }
    }

    fn interp_binary(
        &mut self,
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
            (Value::Number(l), TokenType::Slash, Value::Number(r)) => match r {
                r if r == 0.0 => panic!("Divide by zero!"),
                _ => Ok(Value::Number(l / r)),
            },
            //Strings
            (Value::String(l), TokenType::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
            (Value::String(l), TokenType::Plus, Value::Number(r)) => {
                Ok(Value::String(l + &r.to_string()))
            }
            //Logic
            (Value::Bool(l), TokenType::Equality, Value::Bool(r)) => Ok(Value::Bool(l == r)),
            (Value::Bool(l), TokenType::NotEqual, Value::Bool(r)) => Ok(Value::Bool(l != r)),
            //Equality
            (Value::Number(l), TokenType::Equality, Value::Number(r)) => Ok(Value::Bool(l == r)),
            (Value::Number(l), TokenType::NotEqual, Value::Number(r)) => Ok(Value::Bool(l != r)),
            (Value::Number(l), TokenType::LessEqual, Value::Number(r)) => Ok(Value::Bool(l <= r)),
            (Value::Number(l), TokenType::GreaterEqual, Value::Number(r)) => {
                Ok(Value::Bool(l >= r))
            }
            (Value::Number(l), TokenType::Less, Value::Number(r)) => Ok(Value::Bool(l < r)),
            (Value::Number(l), TokenType::Greater, Value::Number(r)) => Ok(Value::Bool(l > r)),

            (_, _, _) => Err("Binary expression error.".to_string()),
        }
    }

    fn interp_declaration(
        &mut self,
        variable: Symbol,
        expr: Option<Expression>,
    ) -> Result<(), String> {
        match expr {
            Some(expr) => match self.interp_expression(expr) {
                Ok(v) => self.program_scope.define_var(variable, v),
                Err(e) => return Err(e),
            },
            // println!("{} as {}", variable.name, self.interpret_expression(expr).expect("Error interpreting expression after declaration")),
            None => self.program_scope.define_var(variable, Value::Nil),
        }
        Ok(())
    }

    fn interp_variable(&self, v: Symbol) -> Result<Value, String> {
        match self.program_scope.get_var(v.clone()) {
            Ok(v) => Ok(v.clone()),
            Err(err) => match self.global.get_var(v.clone()) {
                Ok(v) => Ok(v.clone()),
                Err(_) => Err(err),
            },
        }
    }
    // Remove the return result to go back to normal assignment
    // Assignment is currently an expression meaning something like print x = 2; will print 2 and all assign variable x to 2;
    // When assignment is a statment it would throw an error for print x = 2; and assignment would always look like y = 3;
    fn interp_assignment(&mut self, sym: Symbol, expr: Expression) -> Result<(), String> {
        match self.interp_expression(expr) {
            Ok(v) => {
                self.program_scope.assign_var(&sym, v);
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }

    // fn interpreter_error(&self, tok: Token, msg: &str) -> Result<Value, String> {
    //     Err(format!("Intepreter Error @ {}: {}", tok.line, msg))
    // }

    fn interp_ternary(
        &mut self,
        i: Box<Expression>,
        r0: Box<Expression>,
        r1: Box<Expression>,
    ) -> Result<Value, String> {
        let interp_i = self.interp_expression(*i);
        match interp_i {
            Ok(v) => match v {
                Value::Bool(b) => match b {
                    true => return self.interp_expression(*r0),
                    false => return self.interp_expression(*r1),
                },
                _ => Err(
                    "First expression of Ternary expression must be boolean expression".to_string(),
                ),
            },
            Err(e) => Err(e),
        }
    }

    fn interp_logical(
        &mut self,
        left_expr: Expression,
        o: Token,
        right_expr: Expression,
    ) -> Result<Value, String> {
        match o.token_type {
            TokenType::And => {
                match self
                    .interp_expression(left_expr)
                    .expect("Error in logical statement")
                {
                    Value::Bool(v) => match v {
                        true => {
                            match self
                                .interp_expression(right_expr)
                                .expect("Error in logical statement")
                            {
                                Value::Bool(v) => match v {
                                    true => return Ok(Value::Bool(true)),
                                    false => return Ok(Value::Bool(false)),
                                },
                                _ => panic!("Error logical expressions should amount to bool"),
                            }
                        }
                        false => return Ok(Value::Bool(false)),
                    },
                    _ => panic!("Error logical expressions should amount to bool"),
                }
            }
            TokenType::Or => {
                let left = self
                    .interp_expression(left_expr)
                    .expect("Error in logical statement.");
                let right = self
                    .interp_expression(right_expr)
                    .expect("Error in logical statement.");

                match (left, right) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
                    _ => Err("Error logical expressions should amount to bool".to_string()),
                }
            }
            _ => Err("Logical operators are 'and', 'or'".to_string()), // Shouldn't be possible
        }
    }

    fn interp_funcdecl(
        &mut self,
        name: Symbol,
        params: Vec<Symbol>,
        body: Expression,
    ) -> Result<(), String> {
        let mut stmts = vec![];
        match body {
            Expression::BlockExpr(a) => stmts = a,
            _ => panic!("Funciton body must be a block. surrounded by {{ }}"),
        }
        let func = Function::new(name.clone(), params,
            stmts,
            self.program_scope.clone(),
            self.f_count,
        );
        let func_value = Value::Function(self.f_count);
        self.function_map.insert(self.f_count, func);
        self.f_count = self.f_count + 1;
        self.program_scope.define_var(name, func_value);
        Ok(())
    }

    fn interp_call(
        &mut self,
        callee: Box<Expression>,
        t: Token,
        args: Vec<Expression>,
    ) -> Result<Value, String> {
        // let callee = self.interp_expression(*callee).expect("Error on callee");
        let arguments: Vec<Value> = args
            .iter()
            .map(|a| {
                self.interp_expression(a.clone())
                    .expect("Error interpreting call arguments")
            })
            .collect();
        self.call(callee, t, arguments)
        // let func =
        // self.program_scope.get_func();
        // return func.call()
    }

    fn call(
        &mut self,
        callee_expr: Box<Expression>,
        loc: Token,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        let callee = self.interp_expression(*callee_expr)?;
        let fval;
        match match_callable(self, callee) {
            Some(mut f) => {
                if args.len() != f.arity() {
                    panic!("Argument length exceeds paramter length");
                } else {
                    fval = f.call(self, &args)?;
                }
            }
            None => todo!(),
        }
        let return_val = self.return_val.clone();
        self.return_val = None;
        match return_val {
            Some(val) => Ok(val.clone()),
            None => Ok(fval),
        }

        //Create new scope
    }

    fn interp_blockexpr(&mut self, stmts: Vec<Statement>) -> Result<Value, String> {
        self.program_scope = Scope::new(Some(Box::new(self.program_scope.clone())));
        let mut last: Value = Value::Nil;
        for stmt in stmts {
            match stmt {
                Statement::Expression(ex) => {
                    last = self.interp_expression(ex)?;
                    match last {
                        Value::Break => return Ok(Value::Break),
                        Value::Continue => return Ok(Value::Continue),
                        _ => (),
                    }
                }
                _ => {
                    self.interp_statement(stmt)?;
                    last = Value::Nil
                }
            }
        }
        //End block and revert to previous scope
        if let Some(scope) = &self.program_scope.enclosing {
            self.program_scope = *scope.clone();
        } else {
            //Something has gone horribly wrong
            panic!("Scope no longer exists")
            //This should be impossible
        }
        return Ok(last);
    }

    fn inetrp_ifexpr(
        &mut self,
        conditon: Box<Expression>,
        then: Box<Expression>,
        elses: Box<Option<Expression>>,
    ) -> Result<Value, String> {
        match self
            .interp_expression(*conditon)
            .expect("Error interpreting if statement")
        {
            Value::Bool(b) => match b {
                true => {
                    let r = self.interp_expression(*then)?;
                    return Ok(r);
                }
                false => match *elses {
                    Some(expr) => self.interp_expression(expr),
                    None => return Ok(Value::Nil),
                },
            },
            _ => return Err("Condition of if statement does not amount to boolean".to_string()), //Error expression does not amount to true false value
        }
    }

    fn interp_whileexpr(
        &mut self,
        conditon: Box<Expression>,
        body: Box<Expression>,
    ) -> Result<Value, String> {
        let mut last = Value::Break;
        while let Value::Bool(v) = self.interp_expression(*conditon.clone())? {
            if let true = v {
                last = self.interp_expression(*body.clone())?;
                // println!("Result of interp: {:?}", last);
                match last {
                    Value::Break => break,
                    Value::Continue => continue,
                    _ => (),
                }
            } else {
                return Ok(last);
            }
        }
        return Ok(last);
    }

    fn interp_loopexpr(&mut self, body: Box<Expression>) -> Result<Value, String> {
        let mut last = Value::Break;
        while true {
            last = self.interp_expression(*body.clone())?;
            match last {
                Value::Break => break,
                Value::Continue => {
                    println!("continue");
                    continue;
                }
                _ => (),
            }
        }
        return Ok(last);
    }
}

fn match_callable(Interpreter: &mut Interpreter, val: Value) -> Option<Box<dyn Callable>> {
    match val {
        Value::NativeFunction(f) => Some(Box::new(f)),
        Value::Function(f) => {
            let function = Interpreter.function_map.get(&f)?.clone();
            Some(Box::new(function))
        }
        _ => todo!(),
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    NativeFunction(NativeFunction),
    Function(u64),
    Nil,
    Break,
    Continue,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::NativeFunction(arg0) => f.debug_tuple("NativeFunction").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
            Self::Nil => write!(f, "Nil"),
            Self::Break => write!(f, "Break"),
            Self::Continue => write!(f, "Continue"),
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
            Value::NativeFunction(n) => f.write_fmt(format_args!("{}", n.name)),
            Value::Break => todo!(),
            Value::Continue => todo!(),
            Value::Function(fu) => f.write_fmt(format_args!("{}", fu)),
        }
    }
}
