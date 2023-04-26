#![allow(unused)]

// Today, Thursday 13 April 2023 at 21:43:09, I am 15 y.o. and I make my first step in the world of programming language development.

#[derive(Debug, Clone, PartialEq)]
pub enum TType {
    LParen,
    RParen,
    Quote,
    Unquote,
    Integer(i64),
    String(String),
    Float(f64),
    Symbol(String),
    Macro,
    Define,
    Lambda,
    Var
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TType,
    pub line: usize,
    pub column: usize
}

impl Token {
    pub fn new(ttype: TType, line: usize, column: usize) -> Self {
        Self {ttype, line, column}
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pub output: Vec<Token>,
    line: usize,
    column: usize,
    curr: usize,
    start: usize
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.to_string().replace("λ", "\\").to_string(),
            output: vec![],
            line: 1,
            column: 0,
            curr: 0,
            start: 0
        }
    }

    pub fn consume(&mut self) -> char {
        self.curr += 1;
        let current_char = self.input.chars().nth(self.curr - 1).unwrap();
        match current_char {
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            _ => self.column += 1
        }
        current_char
    }

    pub fn peek(&mut self) -> char {
        self.input.chars().nth(self.curr).expect("Unexpected EOF.")
    }

    pub fn is_eof(&self) -> bool {
        self.curr >= self.input.chars().count()
    }

    pub fn current_char(&self) -> char {
        self.input.chars().nth(self.curr - 1).unwrap()
    }

    pub fn add_token(&mut self, ttype: TType) {
        self.output.push(Token {
            ttype,
            line: self.line,
            column: self.column
        });
    }

    pub fn string(&mut self) {
        self.start = self.curr;
        while !self.is_eof() && self.peek() != '"' {
            self.consume();
        }
        let raw = self.input[self.start..self.curr].to_owned();
        self.consume();
        self.add_token(TType::String(raw));
    }

    pub fn number(&mut self) {
        self.start = self.curr - 1;

        if self.current_char() == '-' || self.current_char() == '+' {
            self.consume();
        }

        while !self.is_eof() && (self.peek().is_digit(10) || self.peek() == '.') {
            self.consume();
        }
        
        let number = self.input[self.start..self.curr].to_owned();

        match number.parse::<i64>() {
            Ok(n) => self.add_token(TType::Integer(n)),
            Err(_) => self.add_token(TType::Float(number.parse::<f64>().unwrap()))
        }
    }

    pub fn identifier(&mut self) {
        self.start = self.curr - 1;
        let stop = vec!['(', ')', '\n', '\r', '\t', ' '];
        while !self.is_eof() && !stop.contains(&self.peek()) {
            self.consume();
        }

        let symbol = self.input[self.start..self.curr].to_owned();
        
        match symbol.as_str() {
            "define" => self.add_token(TType::Define),
            "macro" => self.add_token(TType::Macro),
            "var" => self.add_token(TType::Var),
            "lambda" | "\\" => self.add_token(TType::Lambda),
            _ => self.add_token(TType::Symbol(symbol))
        }
    }

    pub fn tokenize(&mut self) {
        match self.consume() {
            ' ' | '\n' | '\r' | '\t' => {}
            '(' => self.add_token(TType::LParen),
            ')' => self.add_token(TType::RParen),
            '\'' => {
                if self.peek() == ' ' {
                    panic!("It seems that there is a space between the quotation mark and what you are quoting.Remove that space.");
                }
                self.add_token(TType::Quote)
            }
            ',' => self.add_token(TType::Unquote),
            '"' => self.string(),
            c => if ((c == '+' || c == '-') && self.peek().is_digit(10)) || c.is_digit(10) {
                self.number();
            } else if c.is_ascii() {
                self.identifier();
            } else {
                panic!("Unexpected token: {} ({}:{})", c, self.line, self.column)
            }
        }
    }

    pub fn lex(&mut self) {
        while !self.is_eof() {
            self.tokenize();
        }
    }
}
