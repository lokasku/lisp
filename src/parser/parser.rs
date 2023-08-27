use std::iter::Peekable;

use crate::parser::{
    ReadError,
    Position
};
use crate::parser::lexer::{
    Lexer,
    Token,
    TType
};

#[derive(Debug)]
pub enum Atom {
    Symbol(String),
    String(String),
    Integer(i32),
    Float(f32)
}

#[derive(Debug)]
pub enum SexpT {
    Atom(Atom),
    List(Vec<Sexp>)
}

#[derive(Debug)]
pub struct Sexp {
    sexpt: SexpT,
    line: usize,
    column: usize
}

impl Sexp {
    pub fn new(sexpt: SexpT, line: usize, column: usize) -> Self {
        Self { sexpt, line, column }
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

    pub fn read(&mut self) -> Result<Sexp, ReadError> {
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
                                None => return Err(ReadError::UnclosedParen(Position(line, column)))
                            }
                        }

                        Ok(Sexp::new(SexpT::List(content), line, column))
                    }
                    TType::RParen => Err(ReadError::UnexpectedClosingParen(Position(line, column))),
                    TType::Symbol(s) => Ok(Sexp::new(SexpT::Atom(Atom::Symbol(s)), line, column)),
                    TType::String(s) => Ok(Sexp::new(SexpT::Atom(Atom::String(s)), line, column)),
                    TType::Integer(i) => Ok(Sexp::new(SexpT::Atom(Atom::Integer(i)), line, column)),
                    TType::Float(f) => Ok(Sexp::new(SexpT::Atom(Atom::Float(f)), line, column)),
                }
                Err(err) => Err(err)
            }
            None => Err(ReadError::UnexpectedEOF)
        }
    }
}
