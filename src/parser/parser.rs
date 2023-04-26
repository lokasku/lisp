#![allow(unused)]

use crate::parser::lexer::{
    Token,
    TType
};

#[derive(Debug)]
pub enum  Expr {
    List(Vec<Expr>),
    Operator(String),
    String(String),
    Integer(i64),
    Float(f64),
    Define,
    Macro,
    Var,
    Lambda,
    Symbol(String) 
}

#[derive(Debug)]
pub struct Parser {
    input: Vec<Token>,
    pub output: Vec<Expr>,
    curr: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![],
            curr: 0
        }
    }

    pub fn consume(&mut self) -> Token {
        self.curr += 1;
        self.input.iter().nth(self.curr - 1).unwrap().clone()
    }
    
    pub fn consume_and_check(&mut self, predicate: TType) -> Token {
        self.curr += 1;
        let current_token = self.input.iter().nth(self.curr - 1).unwrap();
        if current_token.ttype == predicate {
            current_token.clone()
        } else {
            panic!("Expected {:?}, {:?} find. ({:?}:{:?})",
                predicate,
                current_token.ttype,
                current_token.line,
                current_token.column);
        }
    }

    pub fn peek(&self) -> TType {
        let curr = self.input.iter().nth(self.curr).unwrap().clone();
        curr.ttype
    }

    pub fn is_eof(&self) -> bool {
        self.curr >= self.input.len()
    }


    pub fn parse_one(&mut self) -> Expr {
        let ct = self.consume();
        
        match ct.ttype {
            TType::LParen => {
                let mut content = vec![];
                while !self.is_eof() && self.peek() != TType::RParen {
                    content.push(self.parse_one());
                }

                if self.is_eof() {
                    panic!("Unclosed parenthesis ({}:{}).", ct.line, ct.column);
                }

                self.consume(); // move after the right paren
                Expr::List(content)
            }
            TType::Quote => {
                let next_expr = self.parse_one();
                Expr::List(vec![Expr::Symbol("quote".to_owned()), next_expr])
            }
            TType::Unquote => {
                let next_expr = self.parse_one();
                Expr::List(vec![Expr::Symbol("unquote".to_owned()), next_expr])
            }
            TType::Integer(i) => Expr::Integer(i),
            TType::String(s) => Expr::String(s),
            TType::Float(f) => Expr::Float(f),
            TType::Symbol(s) => Expr::Symbol(s),
            TType::Macro => Expr::Macro,
            TType::Define => Expr::Define,
            TType::Var => Expr::Var,
            TType::Lambda => Expr::Lambda,
            TType::RParen => panic!("RPAREN BRO!")
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            let expr = self.parse_one();
            match expr {
                Expr::List(_) => self.output.push(expr),
                _ => panic!("Expected List, found {:?}.", expr)
            }
        }
    }
}
