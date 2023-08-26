pub mod lexer;

#[derive(Debug)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self(line, column)
    }
}

#[derive(Debug)]
pub enum ReadError {
    UnexpectedChar(char, Position),
    UnclosedString(Position),
    IncorrectNumber(String, Position)
}
