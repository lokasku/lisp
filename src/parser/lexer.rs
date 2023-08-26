use std::io::Read;

use crate::parser::{
    ReadError,
    Position
};

#[derive(Debug)]
pub enum TType {
    LParen,
    RParen,
    Symbol(String),
    String(String),
    Integer(i32),
    Float(f32)
}

#[derive(Debug)]
pub struct Token {
    ttype: TType,
    line: usize,
    column: usize
}

#[derive(Debug)]
pub struct Lexer<'src> {
    input: &'src str,
    curr: usize,
    start: usize,
    line: usize,
    column: usize
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.consume()?;
        match next_char {
            '"' => Some(self.string()),
            c => if c.is_digit(10) {
                Some(self.number())
            } else {
                Some(Err(ReadError::UnexpectedChar(c, Position::new(self.line, self.column))))
            }
        }
    }
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input,
            curr: 0,
            start: 0,
            line: 1,
            column: 1
        }
    }
    pub fn consume(&mut self) -> Option<char> {
        self.curr += 1;
        match self.input.chars().nth(self.curr - 1) {
            Some(c) => match c {
                '\n' | '\r' => {
                    self.line += 1;
                    self.consume()
                }
                '\t' | ' ' => {
                    self.column += 1;
                    self.consume()
                }
                _ => {
                    self.column += 1;
                    Some(c)
                }
            }
            None => None
        }
    }
    pub fn peek(&self, k: usize) -> Option<char> {
        match self.input.chars().nth(self.curr + k) {
            Some(char) => Some(char),
            None => None
        }
    }
    pub fn is_eof(&self) -> bool {
        self.curr >= self.input.chars().count()
    }
    pub fn token(&self, ttype: TType) -> Token {
        Token {
            ttype,
            line: self.line,
            column: self.start
        }
    }
    pub fn string(&mut self) -> Result<Token, ReadError> {
        self.start = self.curr;
        while self.peek(0) != Some('"') && !self.is_eof() {
            self.consume();
        }
        if self.is_eof() && self.peek(0) != Some('"') {
            return Err(ReadError::UnclosedString(Position::new(self.line, self.column)));
        }
        let raw = self.input[self.start..self.curr].to_owned();
        self.consume();

        Ok(self.token(TType::String(raw)))
    }
    pub fn number(&mut self) -> Result<Token, ReadError> {
        self.start = self.curr;
        while self.peek(0).unwrap().is_digit(10) && !self.is_eof() {
            self.consume();
        }
        if !self.is_eof() && self.peek(0) == Some('.') {
            self.consume();
        }
        while self.peek(0).unwrap().is_digit(10) && !self.is_eof() {
            self.consume();
        }
        let raw = self.input[self.start-1..self.curr].to_owned();
        println!("{}", raw);
        match raw.parse::<i32>() {
            Ok(i) => Ok(self.token(TType::Integer(i))),
            Err(_) => Ok(self.token(TType::Float(raw.parse::<f32>().unwrap())))
        }
    }
}
