use crate::interpreter::Value;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: EnvRef) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), val);
            Ok(())
        } else if let Some(ref enc) = self.enclosing {
            enc.borrow_mut().assign(name, val)
        } else {
            Err(format!("Variável '{}' não definida", name))
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(v) = self.values.get(name) {
            Ok(v.clone())
        } else if let Some(ref enc) = self.enclosing {
            enc.borrow().get(name)
        } else {
            Err(format!("Variável '{}' não definida", name))
        }
    }

    pub fn get_all_variables(&self) -> Vec<(String, Value)> {
        let mut vars: Vec<(String, Value)> = self.values.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        vars.sort_by(|a, b| a.0.cmp(&b.0));
        vars
    }
}