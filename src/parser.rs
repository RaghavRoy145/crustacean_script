use crate::ast::*;
use crate::token::Token;

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
            },
            Token::Number(n) => {
                self.expect_token_and_read(Token::Number(0.0))?;
                Expression::Number(n)
            },
            Token::True => {
                self.expect_token_and_read(Token::True)?;
                Expression::Bool(true)
            },
            Token::False => {
                self.expect_token_and_read(Token::False)?;
                Expression::Bool(false)
            },
            Token::Identifier(s) => {
                self.expect_identifier_and_read()?;
                Expression::Identifier(s)
            },
            Token::Fn => {
                let (params, body) = match self.parse_fn(false)? {
                    Statement::FunctionDeclaration{params, body, ..} => (params, body),
                    _ => unreachable!()
                };
                Expression::Closure(params, body)
            },
            t @ Token::Minus | t @ Token::Bang => {
                self.expect_token_and_read(t.clone())?;
                Expression::Prefix(Op::token(t), self.parse_expression(Precedence::Prefix)?.boxed())
            },
            _ => todo!("{:?}", self.current.clone())
        };
        while !self.current_is(Token::Eof) && precedence::token(self.current.clone()) {
            if let Some(expression) = self.parse_postfix_expression(left.clone())? {
                left = expression;
            } else if let Some(expression) = self.parse_infix_expression(left.clone())? {
                left = expression
            } else {
                break
            }
        }

        Ok(left)
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect_token_and_read(Token::LeftBrace)?;
        let mut block = Vec::New();
        while !self.current_is(Token::RightBrace) {
            block.push(self.parse_statement()?);
        }
        self.expect_token_and_read(Token::RightBrace)?;
        Ok(block)
    }

    fn parse_postfix_expression(&mut self, left: Expression) -> Result<Option<Expression>, ParseError> {
        Ok(match self.current {
            Token::LeftParen => {
                self.expect_token_and_read(Token::LeftParen)?;
                let mut args = Vec::new();

                while !self.current_is(Token::RightParen) {
                    args.push(self.parse_expression(Precedence::Lowest)?);
                    
                    if self.current_is(Token::Comma) {
                        self.read();
                    }
                }

                self.expect_token_and_read(Token::RightParen)?;
                Some(Expression::Call(Box::new(left), args))
            },
            _ => None
        })
    }

    fn parse_let(&mut self) -> Result<Statement, ParseError> {
        self.expect_token_and_read(Token::Let)?;

        let name: Identifier = self.expect_identifier_and_read()?.into();
        let initial: Option<Expression> = if self.current_is(Token::Assign) {
            self.expect_token_and_read(Token::Assign)?;
            Some(self.parse_expression(Precedence::Lowest)?)
        } else {
            None
        };

        Ok(Statement::LetDeclaration {
            name: name,
            initial: initial,
        })
    }

    fn parse_fn(&mut self, with_identifier: bool) -> Result<Statement, ParseError> {
        self.expect_token_and_read(Token::Fn)?;
        let name: Identifier = if with_identifier { expect_identifier_and_read()?.into()
        } else {
            String::from("<Closure>")
        };

        self.expect_token_and_read(Token::LeftParen)?;
        let mut params: Vec<Paramater> = Vec::new();    

        while !self.current_is(Token::RightParen) {
            if self.current_is(Token::Comma) {
                self.expect_token_and_read(Token::Comma)?;
            }
            let param: String = self.expect_identifier_and_read()?.into();
            params.push(Parameter {name: param})
        }
        self.expect_token_and_read(Token::RightParen)?;
        let body: Vec<Statement> = self.parse_block()?;
        Ok(Statement::FunctionDeclaration {
            name: name,
            params: params,
            body: body,
        })
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        self.expect_token_and_read(Token::If)?;

        let condition = self.parse_expression(Precedence::Lowest)?;
        let then = self.parse_block()?;
        let otherwise = if self.current_is(Token::Else) {
            self.expect_token_and_read(Token::Else)?;
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Statement::If{condition, then, otherwise})
    }

    fn expect_token(&mut self, token: Token) -> Result<Token, ParseError> {
        if self.current_is(token) {
            Ok(self.current.clone())
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        }
    }

    fn expect_token_and_read(&mut self, token: Token) -> Result<Token, ParseError> {
        let result = self.expect_token(token)?;
        self.read();
        Ok(result)
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