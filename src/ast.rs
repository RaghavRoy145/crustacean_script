#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        initial: Expression,

    }
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Binary(Box<Expression>, BinaryOp, Box<Expression>)
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Multiply,
    Subtract,
    Divide
}