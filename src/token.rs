use logos::{Lexer, Logos};

pub fn generate(input: &str) -> Vec<Result<Token, ()>> {
    Token::lexer(input).collect()
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut string = lex.slice().to_string();
    
    if string.starts_with("\"") && string.ends_with("\""){
        string.remove(0);
        string.remove(string.len() - 1);
    }
    Some(string)
}

fn to_float(lex:&mut Lexer<Token>) -> Option<f64> {
    Some(lex.slice().parse().ok()?)
}

#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[regex(r"[a-zA-Z_?]+", to_string)]
    Identifier(String),
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    String(String),
    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Number(f64),
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    // #[error]
    // #[regex(r"[\t\n\f]+", logos::skip)]
    // Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_recognise_reserved_keywords() {
        let mut lexer = Token::lexer("fn let true false if else while");
        assert_eq!(lexer.next(), Some(Ok(Token::Fn)));
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::True)));
        assert_eq!(lexer.next(), Some(Ok(Token::False)));
        assert_eq!(lexer.next(), Some(Ok(Token::If)));
        assert_eq!(lexer.next(), Some(Ok(Token::Else)));
        assert_eq!(lexer.next(), Some(Ok(Token::While)));
    }

    #[test]

    fn it_can_recognise_symbols() {
        let mut lexer = Token::lexer("( ) { } +-*/=");

        assert_eq!(lexer.next(), Some(Ok(Token::LeftParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::RightParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::LeftBrace)));
        assert_eq!(lexer.next(), Some(Ok(Token::RightBrace)));
        assert_eq!(lexer.next(), Some(Ok(Token::Plus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Minus)));
        assert_eq!(lexer.next(), Some(Ok(Token::Asterisk)));
        assert_eq!(lexer.next(), Some(Ok(Token::Slash)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
    }

    #[test]
    fn it_can_recognise_identifiers() {
        let mut lexer = Token::lexer("hello_world HelloWorld hello_world? helloWorld");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("hello_world".to_owned()))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("HelloWorld".to_owned()))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("hello_world?".to_owned()))));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier("helloWorld".to_owned()))));
    }

    #[test]
    fn it_can_recognise_numbers() {
        let mut lexer = Token::lexer("12345 6789.01");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(12345.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(6789.01))))
    }

    #[test]
    fn it_can_recognise_strings() {
        let mut lexer = Token::lexer(r##""testing" "testing with \"" "testing\n""##);

        assert_eq!(lexer.next(), Some(Ok(Token::String(r##"testing"##.to_owned()))));
        assert_eq!(lexer.next(), Some(Ok(Token::String(r##"testing with \""##.to_owned()))));
        assert_eq!(lexer.next(), Some(Ok(Token::String(r##"testing\n"##.to_owned()))));
    }
}