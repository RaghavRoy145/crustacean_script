use crate::token::Token;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    FunctionDeclaration {
        name: Identifier,
        params: Vec<Parameter>,
        body: Block,
    },
    LetDeclaration {
        name: Identifier,
        initial: Option<Expression>,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>
    },
    Expression {
        expression: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    InterpolatedString(String),
    Bool(bool),
    Identifier(Identifier),
    Infix(Box<Expression>, Op, Box<Expression>),
    Prefix(Op, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Closure(Vec<Parameter>, Vec<Statement>)
}

impl Expression {
    pub fn some(self) -> Option<Self> {
        Some(self)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn string(self) -> String {
        match self {
            Expression::Identifier(s) | Expression::String(s) => s,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}