use std::fmt;
use logos::{
    Logos,
    Lexer as LogosLexer,
    Span
};


#[derive(Logos, Debug, Clone, PartialEq, Default)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = TType)]
pub enum TType {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("'")]
    Quote,

    #[token(",")]
    Unquote,

    #[regex(r"[a-zA-Z\-\+\*][0-9a-zA-Z\-\+\*]*")]
    Symbol,

    #[regex("\"([^\"]*)\"")]
    String,

    #[regex(r"-?\d+", priority = 2)]
    Int,

    #[regex(r"-?\d+\.\d+")]
    Float,

    #[default]
    Error
}

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Quote,
    Unquote,
    Symbol(String),
    String(String),
    Int(i64),
    Float(f64)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Quote => write!(f, "'"),
            Token::Unquote => write!(f, ","),
            Token::Symbol(s) | Token::String(s) => write!(f, "{}", s.to_uppercase()),
            Token::Int(i) => write!(f, "i{}", i),
            Token::Float(d) => write!(f, "f{}", d)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidChar(Span, String),
    IntParsingError(Span),
    FloatParsingError(Span)
}

pub struct Lexer<'src> {
    pub input: LogosLexer<'src, TType>,
    pub errors: Vec<Error>
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input: LogosLexer::new(input),
            errors: Vec::new()
        }
    }

    pub fn to_token(&mut self, ttype: TType) -> Result<Token, Error> {
        match ttype {
            TType::Error => Err(Error::InvalidChar(self.input.span(), self.input.slice().to_owned())),
            TType::LParen => Ok(Token::LParen),
            TType::RParen => Ok(Token::RParen),
            TType::Quote => Ok(Token::Quote),
            TType::Unquote => Ok(Token::Unquote),
            TType::Symbol => Ok(Token::Symbol(String::from(self.input.slice()))),
            TType::String => Ok(Token::String(String::from(self.input.slice()))),
            TType::Int => match self.input.slice().parse::<i64>() {
                Ok(i) => Ok(Token::Int(i)),
                Err(_) => Err(Error::IntParsingError(self.input.span()))
            }
            TType::Float => match self.input.slice().parse::<f64>() {
                Ok(f) => Ok(Token::Float(f)),
                Err(_) => Err(Error::FloatParsingError(self.input.span()))
            }
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = loop {
            let token = self.input.next()?;
            match token {
                Ok(token) => break self.to_token(token),
                Err(err) => if let Err(err) = self.to_token(err) {
                    self.errors.push(err);
                }
            }
        };
        match token {
            Ok(tk) => Some(tk),
            Err(_) => None
        }
    }
}
