#![allow(unused)]

use std::thread::current;

use crate::parser::lexer::{
    Token,
    TType
};

#[derive(Debug)]
pub enum  Expr {
    List(Vec<Expr>),
    String(String),
    Integer(i64),
    Float(f64),
    Define,
    Macro,
    Var,
    Cond,
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
    
    pub fn consume_and_check(&mut self, predicate: TType) -> TType {
        self.curr += 1;
        let current_token = self.input.iter().nth(self.curr - 1).unwrap();
        if current_token.ttype == predicate {
            current_token.ttype.clone()
        } else {
            panic!("Expected {:?}, found {:?}. ({:?}:{:?})",
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
                let operator = self.consume();

                match operator.ttype {
                    TType::Symbol(s) => match s.as_str() {
                        "macro" | "define" | "lambda" => {
                            let mut content = vec![]; // (macro/define
                            let op = s.as_str();
                        
                            if op == "macro" {
                                content.push(Expr::Macro)
                            } else if op == "define" {
                                content.push(Expr::Define)
                            } else {
                                content.push(Expr::Lambda)
                            }
                            
                            if op != "lambda" {
                                let name = self.parse_one(); // (... name
        
                                if let Expr::Symbol(s) = name {
                                    content.push(Expr::Symbol(s))
                                } else {
                                    panic!("Expected a symbol as name, found {:?} ({}:{}).", name, ct.line, ct.column)
                                }
                            }
    
                            self.consume_and_check(TType::LParen); //  (.. .. (
                            
                            let mut args = vec![];
                            while !self.is_eof() && self.peek() != TType::RParen { // (.. .. (a1 a2 ... aK
                                let arg = self.parse_one();
                                if let Expr::Symbol(s) = arg {
                                    args.push(Expr::Symbol(s))
                                } else {
                                    panic!("Expected symbol as parameter, found {:?} ({:?}:{:?}).", arg, ct.line, ct.column)
                                }
                            }
    
                            content.push(Expr::List(args));
    
                            self.consume_and_check(TType::RParen); // (.. .. (..)
    
                            while !self.is_eof() && self.peek() != TType::RParen { // (.. .. (..) exp1 exp2 ... expK
                                let expr = self.parse_one();
                                content.push(expr);
                            }
    
                            self.consume_and_check(TType::RParen); // (.. .. (..) ..)
    
                            Expr::List(content)
                        }
                        "var" => {
                            let mut content = vec![Expr::Var]; // (var
                        
                            let name = self.parse_one(); // (.. name
                            if let Expr::Symbol(s) = name {
                                content.push(Expr::Symbol(s));
                            } else {
                                panic!("Expected a symbol as name, found {:?} ({}:{}).", name, ct.line, ct.column);
                            }

                            content.push(self.parse_one()); // (.. .. exp
                            self.consume_and_check(TType::RParen); // (.. .. ..)
                            Expr::List(content)
                        }
                        "cond" => {
                            let mut content = vec![Expr::Cond];
                                                        
                            while !self.is_eof() && self.peek() != TType::RParen {
                                self.consume_and_check(TType::LParen); // (.. (

                                let boolean = self.parse_one(); // (.. (bool
                                let exp = self.parse_one(); // (.. (.. expr

                                content.push(Expr::List(vec![boolean, exp]));

                                self.consume_and_check(TType::RParen); // (.. (.. ..)
                            }

                            self.consume_and_check(TType::RParen); // (.. (.. ..)
                                                                   //       .... )
                            Expr::List(content)
                        }
                        o => {
                            let mut content = vec![Expr::Symbol(o.to_owned())]; // (op

                            while !self.is_eof() && self.peek() != TType::RParen { // (.. arg1 arg2
                                content.push(self.parse_one());
                            }

                            self.consume(); // (.. ....)
                            Expr::List(content)
                        }
                    }
                    o => panic!("Expected an operator, found {:?} ({:?}:{:?}).", o, ct.line, ct.column)
                }
            }
            TType::Symbol(s) => Expr::Symbol(s),
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
            TType::RParen => panic!("Unexpected closing parenthesis ({:?}:{:?}).", ct.line, ct.column)
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            let expr = self.parse_one();
            match expr {
                Expr::List(_) => self.output.push(expr),
                _ => panic!("Expected list, found {:?}.", expr)
            }
        }
    }
}