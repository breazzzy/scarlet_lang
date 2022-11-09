use std::collections::HashMap;

use crate::{expression::Symbol, interpreter::Value};

#[derive(Clone)]
pub struct Scope {
    pub values: HashMap<String, Value>,
    // pub funcs: HashMap<String, Function>,
    pub enclosing: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(enclosing: Option<Box<Scope>>) -> Scope {
        Scope {
            values: HashMap::new(),
            // funcs: HashMap::new(),
            enclosing: enclosing,
        }
    }

    pub fn define_var(&mut self, sym: Symbol, val: Value) /*-> Result<(), String>*/
    {
        if self.contains_key(&sym) {
            panic!("Cant define variable twice. [CONSIDER GETTING BETTER ERROR REPORTING THIS SHOULD BE EASY]")
        }
        self.values.insert(sym.name, val);
    }

    pub fn load(&mut self, loading: HashMap<String, Value>) {
        self.values.extend(loading);
    }

    pub fn contains_key(&self, key: &Symbol) -> bool {
        if self.values.contains_key(&key.name) {
            return true;
        } else {
            false
        }
    }

    pub(crate) fn get_at(&self, sym: Symbol, d: usize) -> Value {
        if let Some(val) = self.ancestor(d).values.get(&sym.name){

            return val.clone();
        }else{
            panic!("Error getting value {} in lex scope. Expcted @ depth {}", sym.name, d);
        }
    }

    pub fn assign_at(&mut self, sym: Symbol, val : Value, d: usize){
        self.ancestor_mut(d).values.insert(sym.name.clone(), val);
    }

    pub fn ancestor(&self, dist : usize) -> Scope{
        let mut ret = self.clone();
        for _ in 0..dist{
            if let Some(e) = ret.enclosing.clone(){
            ret = (*e).clone();
            }else{
                panic!("Enclosing scope not found");
            }
        }
        return ret.clone();
    }

    pub fn ancestor_mut(&mut self, dist: usize) -> &mut Scope{
        let mut ret = self;
        for _ in 0..dist{
            if let Some(e) = &mut ret.enclosing{
                ret = &mut (*e);
            }else{
                panic!();
            }
        }
        return ret;
    }
}
