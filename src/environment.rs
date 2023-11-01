use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, name: impl Into<String>, value: Option<Value>) {
        self.values.insert(name.into(), value);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
}

impl Value {
    pub fn to_number(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => unreachable!()
        }
    }
}