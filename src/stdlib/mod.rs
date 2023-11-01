use crate::interpreter::Interpreter;
use crate::environment::Value;

pub fn println(_: &mut Interpreter, args: Vec<Value>) -> Option<Value> {
    let arg = args.get(0).unwrap().clone();
    println!("{}", arg.to_string());
    None
}