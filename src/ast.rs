use crate::token::Token;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

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
}