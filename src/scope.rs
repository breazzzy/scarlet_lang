use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{expression::Symbol, interpreter::Value};

#[derive(Clone)]
pub struct Scope {
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(enclosing: Option<Box<Scope>>) -> Scope {
        Scope {
            values: HashMap::new(),
            enclosing: enclosing,
        }
    }

    pub fn define_var(&mut self, sym: Symbol, val: Value) /*  -> Result<Value, String> */
    {
        if self.contains_key(&sym) {
            //    return Err("Cant define variable twice".to_string());
            panic!("Cant define variable twice")
        }
        // println!("{} defined as {}", sym.name.clone(), val);
        self.values.insert(sym.name, val);
        // Ok(val)
    }

    pub fn contains_key(&self, key: &Symbol) -> bool {
        if self.values.contains_key(&key.name) {
            return true;
        } else {
            if let Some(enclosing_scope) = &self.enclosing {
                return enclosing_scope.contains_key(key);
            } else {
                return false;
            }
        }
    }

    pub fn get_var(&self, sym: Symbol) -> Result<&Value, String> {
        match self.values.get(&sym.name) {
            Some(v) => Ok(v),
            None => match &self.enclosing {
                Some(e) => e.get_var(sym),
                None => Err(format!("Symbol {} not recognized", sym.name)),
            },
        }
    }

    pub fn assign_var(&mut self, sym: &Symbol, v: Value) /*-> Result<Value, String>*/
    {
        if self.values.contains_key(&sym.name) {
            self.values.insert(sym.name.clone(), v);
            // Ok(v)
        } else {
            if let Some(enclosing_scope) = &mut self.enclosing {
                enclosing_scope.assign_var(&sym, v);
                return;
            }
            // panic!("Cannot recognize symbol {}. Did you forget to define {}", sym.name,sym.name);
            // Err(format!("Cannot recognize symbol {}. Did you forget to define {}", sym.name,sym.name))
            panic!(
                "Cannot recognize symbol {}. Did you forget to define {}",
                sym.name, sym.name
            );
        }
    }
}
