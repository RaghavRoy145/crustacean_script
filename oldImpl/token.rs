#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Assign,
    Let,

    String,
    Number,

    Plus,
    Multiply,
    Subtract,
    Divide
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {kind, literal}
    }
}
