use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use crate::interpreter::Interpreter;

pub type NativeFunctionCallback = fn (&mut Interpreter, Vec<Value>) -> Option<Value>;

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

    pub fn get(&self, name: impl Into<String>) -> Option<Value> {
        *self.values.get(&name.into()).unwrap().clone()
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    NativeFunction{
        callback: NativeFunctionCallback
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::NativeFunction { name, .. } => format!("<{}>", name),
            _ => todo!(),
        })
    }
}

impl Value {
    pub fn to_number(self) -> f64 {
        match self {
            Value::Number(n) => n,
            _ => unreachable!()
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            _ => todo!()
        }
    }
}