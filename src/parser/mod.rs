// use std::fmt;

pub mod lexer;
pub mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self(line, column)
    }
}
