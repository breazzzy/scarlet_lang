use crate::{interpreter::{self, Interpreter, Value}, expression::Symbol, statement::Statement, scope::Scope};
use std::fmt::Debug;


#[derive(Clone)]
pub struct Function{
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Statement>,
    
}

impl Callable for Function{
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String> {
        let mut last = Ok(Value::Nil);
        //Move interpreter to new scope
        interpreter.program_scope = Scope::new(Some(Box::new(interpreter.program_scope.clone())));
        //Map args to params
        self.params.clone().into_iter().enumerate().for_each(|(i,ele)| {
            interpreter.program_scope.define_var(ele, args[i].clone());   
        });
        // println!("{:?}", interpreter.program_scope.values);
        for (_, stmt) in self.body.clone().into_iter().enumerate(){
            if let Some(_) = interpreter.return_val.clone() {
                break;
            }
            match stmt {
                Statement::Expression(e) => {last = interpreter.interp_expression(e)},
                _ => _ = interpreter.interp_statement(stmt),
            }
            // last = interpreter.interp_statement(s);
        }
        //Convert back to previous scope
        if let Some(scope) = interpreter.program_scope.enclosing.clone() {
            interpreter.program_scope = *scope.clone();
        } else {
            //Something has gone horribly wrong
            panic!("Scope no longer exists")
            //This should be impossible
        }
        return last;
    }
}

impl Debug for Function{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function").field("name", &self.name).field("params", &self.params).field("body", &self.body).finish()
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub callable: fn(&mut Interpreter, &[Value]) -> Result<Value, String>,
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .finish()
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String> {
        return (self.callable)(interpreter, args);
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String>;
}
