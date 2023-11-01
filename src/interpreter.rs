use std::collections::HashMap;
use std::slice::Iter;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use crate::ast::*;
use crate::environment::*;

pub fn interpret(ast: Program) {
    let mut interpreter = Interpreter::new(ast.iter());
    interpreter.define_global_function("println", crate::stdlib::println);
    interpreter.run();
}

#[derive(Debug, Clone)]
pub struct Interpreter<'i> {
    ast: Iter<'i, Statement>,
    environment: Rc<RefCell<Environment>>,
    globals: HashMap<String, Value>,
}

impl<'i> Interpreter<'i> {
    fn new(ast: Iter<'i, Statement>) -> Self {
        Self {
            ast:ast,
            environment: Rc::new(RefCell::new(Environment::new())),
            globals: HashMap::new(),
        }
    }

    fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::LetDeclaration { name, initial } => {
                if initial.is_none() {
                    self.env_mut().set(name, None)
                } else {
                    let initial = initial.unwrap();
                    let value = self.run_expression(initial);
                    self.env_mut().set(name, value)
                }
            },
            Statement::Expression {expression} => {
                self.run_expression(expression);
            },
            _ => todo!("{:?}",statement),
        }
    }

    fn run_expression(&mut self, expression: Expression) -> Option<Value> {
        Some(match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::String(s) => Value::String(s),
            Expression::Identifier(n) => {
                if self.globals.contains_key(&n) {
                    self.globals[&n].clone() 
                } else {
                    return self.env().get(n)
                }
            }
            Expression::Infix(left, op, right) => {
                let left = self.run_expression(*left).unwrap();
                let right = self.run_expression(*right).unwrap();

                match (left, op, right) {
                    (Value::Number(l), Op::Add, Value::Number(r)) => Value::Number(l + r),
                    (Value::Number(l), Op::Multiply, Value::Number(r)) => Value::Number(l * r),
                    (Value::Number(l), Op::Divide, Value::Number(r)) => Value::Number(l / r),
                    (Value::Number(l), Op::Subtract, Value::Number(r)) => Value::Number(l - r),
                    _ => todo!()
                }
            },
            Expression::Call(callable, arguments) => {
                let callable = self.run_expression(*callable).unwrap();
                let arguments: Vec<Value>= arguments.into_iter().map(|a| self.run_expression(a).unwrap()).collect();
                match callable {
                    Value::NativeFunction { callback, .. } => return callback(self, arguments),
                    _ => todo!(),
                }
            },
            _ => todo!("{:?}", expression),
        })
    }

    fn define_global_function(&mut self, name: impl Into<String>, callback: NativeFunctionCallback) {
        let name = name.into();
        self.globals.insert(name.clone(), Value::NativeFunction {
            name: name,
            callback: callback,
        });
    }

    fn env(&self) -> Ref<Environment> {
        RefCell::borrow(&self.environment)
    }

    fn env_mut(&mut self) -> RefMut<Environment> {
        RefCell::borrow_mut(&self.environment)
    }
 
    fn run(&mut self) {
        while let Some(statement) = self.ast.next() {
            self.run_statement(statement.clone());
        }
    }
}