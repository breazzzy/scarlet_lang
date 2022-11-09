use crate::{
    expression::Symbol,
    interpreter::{Interpreter, Value},
    scope::Scope,
    statement::Statement,
};
use std::{fmt::Debug, collections::HashMap};

#[derive(Clone)]
pub struct Function {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Statement>,
    pub closure: Scope,
    pub f_id: u64,
}

impl Function {
    pub fn new(
        name: Symbol,
        params: Vec<Symbol>,
        body: Vec<Statement>,
        closure: Scope,
        f_id: u64,
    ) -> Function {
        Function {
            name,
            params,
            body,
            closure,
            f_id,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&mut self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String> {
        // println!(
        //     "Calling {} with f_id {} with closure {:?}",
        //     self.name.name, self.f_id, self.closure.values
        // );
        let mut last = Ok(Value::Nil);

        let args_map : HashMap<_,_> = self.params.iter().zip(args.iter()).map(|(param,arg)|{
            (
                param.name.clone(),
                    arg.clone(),
            )
        }).collect();

        //Clone old scope so we can return to it later.
        let mut old_scope = interpreter.program_scope.clone();
        //Create new scope by cloning this functions saved closure\\

        let mut func_scope = self.closure.clone();

        func_scope.values.extend(args_map);

        old_scope.values.insert(self.name.name.clone(), Value::Function(self.f_id));
        old_scope.values.extend(self.closure.values.clone());
        func_scope.enclosing = Some(Box::new(old_scope.clone()));

        //Move interpreter to new scope
        interpreter.program_scope = func_scope.clone();

        // println!("{:?}", interpreter.program_scope.values);
        for (_, stmt) in self.body.clone().into_iter().enumerate() {
            if let Some(_) = interpreter.return_val.clone() {
                break;
            }
            match stmt {
                Statement::Expression(e) => last = interpreter.interp_expression(e),
                _ => _ = interpreter.interp_statement(stmt),
            }
        }
        // println!("After function call the scope looks like {:?}",interpreter.program_scope.enclosing.clone().expect("").values);
        //Update closure
        self.closure = *interpreter.program_scope.enclosing.clone().expect("");
        
        //Update reference to function in interpreter. This assures that the function is called with the updated closure next time its called.
        interpreter.function_map.insert(self.f_id, self.clone());
        // old_scope.assign_var(&self.name, Value::Function(self.clone()));
        interpreter.program_scope = old_scope.clone(); // Set interpreter back to old scope
        // println!("AT END OF FUNCTION PROGRAM SCOPE IS EQUAL TO {:?}", interpreter.program_scope.values);
        return last;
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
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

    fn call(&mut self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String> {
        return (self.callable)(interpreter, args);
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&mut self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String>;
}
