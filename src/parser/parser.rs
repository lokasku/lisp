#[allow(unused)]

use crate::parser::lexer::{
    Token,
    TType
};

#[derive(Debug, Clone)]
pub enum Atom {
    Symbol(String),
    String(String),
    Integer(i64),
    Float(f64)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
    Quote(Box<Expr>),
    Unquote(Box<Expr>)
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

    pub fn parse_expr(&mut self) -> Expr {
        let ct = self.consume();

        match ct.ttype {
            TType::LParen => {
    	        let mut args: Vec<Expr> = vec![];
		while !self.is_eof() && self.peek() != TType::RParen {
                    args.push(self.parse_expr());
                }
                self.consume();
		Expr::List(args)
	    }
            TType::Quote => {
                let next_symbol = self.parse_expr();
                Expr::Quote(Box::new(next_symbol))
            }
            TType::Unquote => {
                let next_symbol = self.parse_expr();
                Expr::Unquote(Box::new(next_symbol))
            }
            TType::Symbol(s) => Expr::Atom(Atom::Symbol(s)),
            TType::String(s) => Expr::Atom(Atom::String(s)),
            TType::Integer(i) => Expr::Atom(Atom::Integer(i)),
            TType::Float(f) => Expr::Atom(Atom::Float(f)),
            TType::RParen => panic!("Unexpected closing parenthesis ({:?}:{:?}).", ct.line, ct.column)
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            let expr = self.parse_expr();
            match expr {
                Expr::List(_) => self.output.push(expr),
                _ => panic!("Expected list, found {:?}.", expr)
            }
        }
    }
}
