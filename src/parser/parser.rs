use std::fmt;
use std::iter::Peekable;

use crate::parser::Position;
use crate::errors::{
    Error,
    ReadError
};

use crate::parser::lexer::{
    Lexer,
    Token,
    TType
};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Symbol(String),
    String(String),
    Integer(i32),
    Float(f32)
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Symbol(s) => write!(f, "{} ", s),
            Atom::String(s) => write!(f, "\"{}\" ", s),
            Atom::Integer(n) => write!(f, "{} ", n),
            Atom::Float(n) => write!(f, "{} ", n)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SexpT {
    Atom(Atom),
    List(Vec<Sexp>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sexp {
    pub sexpt: SexpT,
    /* pub line: usize,
    pub column: usize */
    pub pos: Position
}

impl Sexp {
    pub fn new(sexpt: SexpT, line: usize, column: usize) -> Self {
        Self {
            sexpt,
            pos: Position::new(line, column)
        }
    }
}

#[derive(Debug)]
pub struct Parser<'src> {
    input: Peekable<Lexer<'src>>
}

impl<'src> Parser<'src> { pub fn new(input: &'src str) -> Self {
        Self {
            input: Lexer::new(input).peekable()
        }
    }

    pub fn read(&mut self) -> Result<Sexp, Error> {
        match self.input.next() {
            Some(result) => match result {
                Ok(Token {ttype, line, column }) => match ttype {
                    TType::LParen => {
                        let mut content: Vec<Sexp> = Vec::new();

                        loop {
                            match self.input.peek() {
                                Some(result) => match result {
                                    Ok(Token { ttype: TType::RParen, ..} ) => {
                                        self.input.next(); // move after closing parenthesis
                                        break; // we have reached the end of the list
                                    }
                                    Err(_) => return self.read(), // we know that this call will return the error
                                    _ => content.push(self.read()?)
                                }
                                None => return Err(Error::ReadError(ReadError::UnclosedParen(Position(line, column))))
                            }
                        }
                        Ok(Sexp::new(SexpT::List(content), line, column))
                    }
                    TType::RParen => Err(Error::ReadError(ReadError::UnexpectedClosingParen(Position(line, column)))),
                    TType::Symbol(s) => Ok(Sexp::new(SexpT::Atom(Atom::Symbol(s)), line, column)),
                    TType::String(s) => Ok(Sexp::new(SexpT::Atom(Atom::String(s)), line, column)),
                    TType::Integer(i) => Ok(Sexp::new(SexpT::Atom(Atom::Integer(i)), line, column)),
                    TType::Float(f) => Ok(Sexp::new(SexpT::Atom(Atom::Float(f)), line, column))
                }
                Err(err) => Err(err)
            }
            None => Err(Error::ReadError(ReadError::UnexpectedEOF))
        }
    }
}

pub fn independant_sexp(sexpt: SexpT) -> Sexp {
    Sexp::new(sexpt, 0, 0)
}
