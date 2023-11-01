use std::slice::Iter;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use crate::ast::*;
use crate::environment::*;

pub fn interpret(ast: Program) {
    let mut interpreter = Interpreter::new(ast.iter());
    interpreter.run();
}

#[derive(Debug, Clone)]
struct Interpreter<'i> {
    ast: Iter<'i, Statement>,
    environment: Rc<RefCell<Environment>>,
}

impl<'i> Interpreter<'i> {
    fn new(ast: Iter<'i, Statement>) -> Self {
        Self {
            ast:ast,
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    fn run_statement(&mut self, statement: Statement) {
        match statement {
            Statement::LetDeclaration { name, initial } => {
                if initial.is_none() {
                    self.env_mut().set(name, None)
                } else {
                    let initial = initial.unwrap();
                    let value = self.run_expression(intial);
                    self.env_mut().set(name, value)
                }
            },
            _ => todo!("{:?}",statement),
        }
    }

    fn run_expression(&mut self, expression: Expression) -> Option<Value> {
        Some(match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::Infix(left, op, right) => {
                let left = self.run_expression(*left).unwrap();
                let right = self.run_expression(*right).unwrap();

                match (left, op, right) {
                    (Value::Number(l), Op::Add, Value::Number(r)) => Value::Number(l + r),
                    (Value::Number(l), Op::Multiply, Value::Number(r)) => Value::Number(l * r),
                    (Value::Number(l), Op::Dividing, Value::Number(r)) => Value::Number(l / r),
                    (Value::Number(l), Op::Subtracting, Value::Number(r)) => Value::Number(l - r),
                    _ => todo!()
                }
            }
            _ => todo!("{:?}", expression),
        })
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