use std::str::Chars;
use std::iter::Peekable;

use crate::parser::Position;
use crate::errors::{
    Error,
    ReadError
};

#[derive(Debug, PartialEq)]
pub enum TType {
    LParen,
    RParen,
    Symbol(String),
    String(String),
    Integer(i32),
    Float(f32)
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype: TType,
    pub line: usize,
    pub column: usize
}

const SYMBOL_CHARS: [char; 5] = ['-', '+', '*', '|', '~'];

#[derive(Debug)]
pub struct Lexer<'src> {
    input_as_chars: Peekable<Chars<'src>>,
    input_as_str: &'src str,
    curr: usize,
    start: usize,
    line: usize,
    column: usize
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.consume()?;
        match next_char {
            '(' => Some(Ok(Token {
                    ttype: TType::LParen,
                    line: self.line,
                    column: self.column
            })),
            ')' => Some(Ok(Token {
                ttype: TType::RParen,
                line: self.line,
                column: self.column
            })),
            '"' => Some(self.string()),
            c => if ((c == '+' || c == '-') && self.input_as_chars.peek().unwrap().is_digit(10)) || c.is_digit(10) {
                Some(self.number())
            } else if c.is_alphabetic() || SYMBOL_CHARS.contains(&c) {
                Some(self.symbol())
            } else {
                Some(Err(Error::ReadError(ReadError::UnexpectedChar(c, Position::new(self.line, self.column)))))
            }
        }
    }
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input_as_chars: input.chars().peekable(),
            input_as_str: input,
            curr: 0,
            start: 0,
            line: 1,
            column: 0
        }
    }
    pub fn consume(&mut self) -> Option<char> {
        self.curr += 1;
        while let Some(c) = self.input_as_chars.next_if(|c| c.is_whitespace()) {
            match c {
                '\n' | '\r' => {
                    self.line += 1;
                    self.column = 0;
                    self.curr += 1;
                }
                '\t' | ' ' => {
                    self.column += 1;
                    self.curr += 1;
                }
                _ => {}
            }
        }
        self.column += 1;
        self.input_as_chars.next()
    }
    pub fn is_eof(&self) -> bool {
        self.curr >= self.input_as_str.chars().count()
    }
    pub fn build_token(&self, ttype: TType) -> Token {
        Token {
            ttype,
            line: self.line,
            column: self.start
        }
    }
    pub fn string(&mut self) -> Result<Token, Error> {
        self.start = self.curr;
        while self.input_as_chars.peek() != Some(&'"') && !self.is_eof() {
            self.consume();
        }
        if self.is_eof() && self.input_as_chars.peek() != Some(&'"') {
            return Err(Error::ReadError(ReadError::UnclosedString(Position::new(self.line, self.start))));
        }
        let raw = self.input_as_str[self.start..self.curr].to_owned();
        self.consume();

        Ok(self.build_token(TType::String(raw)))
    }
    pub fn symbol(&mut self) -> Result<Token, Error> {
        self.start = self.curr;
        while (
            self.input_as_chars.peek().unwrap().is_ascii_alphanumeric()
            || SYMBOL_CHARS.contains(&self.input_as_chars.peek().unwrap()))
            && !self.is_eof()
        {
            self.consume();
        }
        let raw = self.input_as_str[self.start-1..self.curr].to_owned();
        Ok(self.build_token(TType::Symbol(raw)))
    }
    pub fn number(&mut self) -> Result<Token, Error> {
        self.start = self.curr;
        while (self.input_as_chars.peek().unwrap().is_digit(10) || self.input_as_chars.peek().unwrap() == &'.') && !self.is_eof() {
            self.consume();
        }
        let raw = self.input_as_str[self.start-1..self.curr].to_owned();
        match raw.parse::<i32>() {
            Ok(i) => Ok(self.build_token(TType::Integer(i))),
            Err(_) => match raw.parse::<f32>() {
                Ok(f) => Ok(self.build_token(TType::Float(f))),
                Err(_) => Err(Error::ReadError(ReadError::IncorrectNumber(raw, Position::new(self.line, self.start))))
            }
        }
    }
}
