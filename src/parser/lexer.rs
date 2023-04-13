#![allow(unused)]

// Today, Thursday 13 April 2023 at 21:43:09, I am 15 y.o. and I make my first step in the world of programming language development.

pub enum TType {
    LParen,
    RParen,
    Quote,
    Integer(i64),
    String(String),
    Float(f64),
    Symbol(String),
    Macro,
    Func,
    Lambda
}

pub struct Token {
    ttype: TType,
    line: usize,
    column: usize
}

impl Token {
    pub fn new(ttype: TType, line: usize, column: usize) -> Self {
        Self {ttype, line, column}
    }
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    line: usize,
    column: usize,
    curr: usize,
    start: usize
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            output: vec![],
            line: 1,
            column: 1,
            curr: 0,
            start: 0
        }
    }

    pub fn advance(&mut self) -> char {
        self.curr += 1;
        self.column += 1;
        //                               vvv to pass on the index character 0
        self.input.chars().nth(self.curr - 1).unwrap()
    }

    pub fn peek(&mut self) -> char {
        self.input.chars().nth(self.curr).expect("Picking from the void.")
    }
}
