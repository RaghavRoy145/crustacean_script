#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Assign,
    Let,
    String,
    Number
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {kind, literal}
    }
}
