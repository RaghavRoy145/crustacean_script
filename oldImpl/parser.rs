use crate::lexer::Lexer;
use crate::ast::{Statement, Expression, BinaryOp};
use crate::token::{TokenKind, Token};

pub type Program = Vec<Statement>;
pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self{lexer}
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::<Statement>::new();
        while let Some(token) = self.lexer.next() {
            match token.kind {  
                TokenKind::Let => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected an identifier");
                    };

                    if ! matches!(self.lexer.peek(), Some(Token { kind: TokenKind::Assign, ..})) {
                        panic!("Expected an '=' for assignment");
                    }
                    self.lexer.next();
                    let expression = self.parse_expression();
                    statements.push(Statement::Let { 
                        name: identifier.literal, 
                        initial: expression  
                    })

                },

                _ => unimplemented!()
            }
        }
        statements
    }

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut lhs = match self.lexer.next() {
            Some(Token {kind: TokenKind::Number, literal }) => {
                Expression::Number(literal.parse().unwrap())
            },
            _ => unimplemented!(),
        };

        loop {
            let infix = if let Some(infix) = self.lexer.peek() {
                infix
            } else {
                break;
            };

            if let Some((lbp, rbp)) = infix_binding_power(infix.kind) {
                if lbp < bp {
                    break
                }
                let op = self.lexer.next().unwrap().kind;
                let rhs = self.parse_expression(rbp);
                lhs = make_infix_expression(lhs, op, rhs);
                continue;
            }
            break;
        }
        lhs
    }
}

pub fn infix_binding_power(kind: TokenKind) -> Option<(u8,u8)> {
    let bp = match kind {
        TokenKind::Multiply |  TokenKind::Divide => (8,9),
        TokenKind::Plus | TokenKind::Subtract => (6,7),
        _ => return None,
    };

    Some(bp)
}

pub fn make_infix_expression(lhs: Expression, operator: TokenKind, rhs: Expression) -> Expression {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    match operator {
        TokenKind::Plus => Expression::Binary(lhs, BinaryOp::Plus, rhs),
        TokenKind::Multiply => Expression::Binary(lhs, BinaryOp::Multiply, rhs),
        TokenKind::Subtract => Expression::Binary(lhs, BinaryOp::Subtract, rhs),
        TokenKind::Divide => Expression::Binary(lhs, BinaryOp::Divide, rhs),
        _ => unimplemented!()
    }
}