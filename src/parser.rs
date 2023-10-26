use crate::lexer::Lexer;
use crate::ast::Statement;
use crate::token::TokenKind;

pub type Program = Vec<Statement>;
pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self{lexer}
    }

    pub fn parse(&mut self) -> Program {
        let statements = Vec::<Statement>::new();
        while let Some(token) = self.lexer.next() {
            match token.kind {  
                TokenKind::Let => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected an identifier");
                    };
                }
                _ => unimplemented!()
            }
        }
        statements
    }
}