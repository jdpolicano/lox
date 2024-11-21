use crate::token::Literal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum EnvironmentError {
    AssignUndefinedVariable(String),
    ReadUndefinedVariable(String),
}

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn define(&mut self, k: String, v: Literal) {
        self.values.insert(k, v);
    }

    pub fn get(&mut self, k: &str) -> Option<Literal> {
        if let Some(v) = self.values.get(k) {
            return Some(v.clone());
        }

        if let Some(ref p) = self.parent {
            p.borrow_mut().get(k)
        } else {
            None
        }
    }

    pub fn assign(&mut self, k: String, v: Literal) -> Result<(), ()> {
        if self.values.contains_key(&k) {
            self.values.get_mut(&k).map(|x| *x = v);
            Ok(())
        } else if let Some(ref p) = self.parent {
            p.borrow_mut().assign(k, v)
        } else {
            Err(())
        }
    }
}
