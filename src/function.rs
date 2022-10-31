use crate::{interpreter::{self, Interpreter, Value}, expression::Symbol, statement::Statement};
use std::fmt::Debug;


pub struct Function{
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Statement>,
    
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
