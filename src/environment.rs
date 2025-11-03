use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::interpreter::Value;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub enclosing: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
    }

    pub fn with_enclosing(enclosing: EnvRef) -> Self {
        Self { values: HashMap::new(), enclosing: Some(enclosing) }
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.values.insert(name, val);
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), val);
            return Ok(());
        }
        if let Some(ref enc) = self.enclosing {
            return enc.borrow_mut().assign(name, val);
        }
        Err(format!("Variável '{}' não definida.", name))
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(v) = self.values.get(name) { return Ok(v.clone()); }
        if let Some(ref enc) = self.enclosing {
            return enc.borrow().get(name);
        }
        Err(format!("Variável '{}' não definida.", name))
    }
}