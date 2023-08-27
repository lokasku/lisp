pub mod lexer;
pub mod parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self(line, column)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadError {
    UnexpectedChar(char, Position),
    UnclosedString(Position),
    IncorrectNumber(String, Position),

    UnclosedParen(Position),
    UnexpectedClosingParen(Position),
    UnexpectedEOF
}
