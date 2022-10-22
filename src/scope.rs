use std::collections::HashMap;

use crate::{expression::Symbol, interpreter::Value};

pub struct Scope{
    pub values : HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Scope{
        Scope { values: HashMap::new() }
    }

    pub fn define_var(&mut self, sym : Symbol, val : Value){
        if self.values.contains_key(&sym.name) {
            panic!("Cant assign varriable twice")
        }
        println!("{} defined as {}", sym.name.clone(), val);
        self.values.insert(
            sym.name,
            val,
        );
    }

    pub fn get_var(&self, sym : Symbol) -> Result<&Value, String>{
        match self.values.get(&sym.name) {
            Some(v) => Ok(v),
            None => Err(format!("Symbol {} no recognized", sym.name)),
        }
    }
}