use std::fmt;

use crate::parser::Position;
use crate::parser::parser::SexpT;

#[derive(Debug, PartialEq, Clone)]
pub enum ReadError {
    UnexpectedChar(char, Position),
    UnclosedString(Position),
    IncorrectNumber(String, Position),

    UnclosedParen(Position),
    UnexpectedClosingParen(Position),
    UnexpectedEOF
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(c, pos) => write!(f, "Unexpected character : '{c}' ({}:{})", pos.0, pos.1),
            Self::UnclosedString(pos) => write!(f, "Unclosed string ({}:{})", pos.0, pos.1),
            Self::IncorrectNumber(n, pos) => write!(f, "Incorrect number : {n} ({}:{})", pos.0, pos.1),
            Self::UnclosedParen(pos) => write!(f, "Unclosed parenthesis ({}:{})", pos.0, pos.1),
            Self::UnexpectedClosingParen(pos) => write!(f, "Unexpected closing parenthesis ({}:{})", pos.0, pos.1),
            Self::UnexpectedEOF => write!(f, "Unexpected end of file.")
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    IllegalFunctionCall(Position),
    ArityMismatch(String, usize, usize, Position),
    UnboundSymbol(String, Position),
    TypeMismatch(SexpT, String, Position),
    ReadError(ReadError)
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalFunctionCall(pos) => write!(f, "Illegal function call ({}:{})", pos.0, pos.1),
            Self::UnboundSymbol(sname, pos) => write!(f, "The symbol {sname} is unbound ({}:{})", pos.0, pos.1),
            Self::ArityMismatch(fname, given, expected, pos) =>
                write!(
                    f,
                    "{fname} expected {expected} arguments, {given} were given ({}:{})",
                    pos.0, pos.1
                ),
            Self::TypeMismatch(given, expected, pos) => write!(f, "{:?} is not a {expected} ({}:{})", given, pos.0, pos.1),
            Self::ReadError(re) => ReadError::fmt(re, f)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ReadError(ReadError),
    EvalError(EvalError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadError(re) => ReadError::fmt(re, f),
            Self::EvalError(er) => EvalError::fmt(er, f)
        }
    }
}
