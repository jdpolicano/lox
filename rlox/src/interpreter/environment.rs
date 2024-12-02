use crate::interpreter::primitive::LoxObject;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LoxObject>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn new_rc(parent: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(parent)))
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            parent: self.parent.clone(),
        }
    }

    pub fn define(&mut self, k: String, v: LoxObject) {
        self.values.insert(k, v);
    }

    pub fn get(&mut self, k: &str) -> Option<LoxObject> {
        if let Some(v) = self.values.get(k) {
            return Some(v.clone());
        }

        if let Some(ref p) = self.parent {
            p.borrow_mut().get(k)
        } else {
            None
        }
    }

    pub fn assign(&mut self, k: String, v: LoxObject) -> Result<(), ()> {
        if self.values.contains_key(&k) {
            self.values.get_mut(&k).map(|x| *x = v);
            Ok(())
        } else if let Some(ref p) = self.parent {
            p.borrow_mut().assign(k, v)
        } else {
            Err(())
        }
    }

    pub fn print_map(&self) {
        let msg: String = self.values.iter().map(|(k, v)| format!("{k}{v}")).collect();
        println!("msg {}", msg);
    }
}
