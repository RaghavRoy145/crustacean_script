use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens.iter());

    parser.read();
    parser.read();

    let mut program: Program = Vec::new();

    while let Some(statement) = parser.next()? {
        program.push(statement);
    }

    Ok(program)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn token(token: Token) -> Self {
        match token {
            Token::Asterisk | Token::Slash => Self::Product,
            Token::Plus | Token::Minus => Self::Sum,
            Token::LeftParen => Self::Call,
            _ => Self::Lowest,
        }
    }
}

struct Parser<'p> {
    tokens: Iter<'p, Token>,
    current: Token,
    peek: Token,
}

impl<'p> Parser<'p> {
    fn new(tokens: Iter<'p, Token>) -> Self {
        Self {
            current: Token::Eof,
            peek: Token::Eof,
            tokens: tokens,
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current {
            Token::Fn => self.parse_fn(true),
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            _ => Ok(Statement::Expression{expression: self.parse_expression(Precedence::Lowest)?})
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseError> {
        let mut left = match self.current.clone() {
            Token::String(s) => {
                self.expect_token_and_read(Token::String("".to_string()))?;
                Expression::String(s.to_string())
            },

            Token::InterpolatedString(s) => {
                self.expect_token_and_read(Token::InterpolatedString("".to_string()))?;
                Expression::InterpolatedString(s.to_string())
            }
        };
    }

    fn expect_identifier_and_read(&mut self) -> Result<Token, ParseError> {
        self.expect_token_and_read(Token::Identifier("".to_string()))
    }

    fn current_is(&self, token: Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(&token)
    }

    fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.tokens.next() {
            token.clone()
        } else {
            Token::Eof
        };
    }

    fn next(&mut self) -> Result<Option<Statement>, ParseError> {
        if self.current == Token::Eof {
            return Ok(None)
        }

        Ok(Some(self.parse_statement()?))
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {0:?}.")]
    UnexpectedToken(Token),
}

#[cfg(test)]

mod tests {
    use super::*;
}