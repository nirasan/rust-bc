use crate::object::Object;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    store: HashMap<String, Rc<Object>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{
            store: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<Object>> {
        self.store.get(key).and_then(|v| Some(v.clone()))
    }

    pub fn set(&mut self, key: String, val: Rc<Object>) -> Option<Rc<Object>> {
        self.store.insert(key, val.clone());
        return Some(val);
    }
}