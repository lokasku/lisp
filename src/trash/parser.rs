#[allow(unused)]

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
    input: String,
    line: usize,
    column: usize,
    curr: usize,
    start: usize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            input: input.to_string().replace("λ", "\\").to_string(),
            line: 1,
            column: 1,
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

    pub fn previous_char(&mut self) -> char {
        self.input.chars().nth(self.curr - 1).unwrap()
    }

    pub fn string(&mut self) -> Expr {
        self.start = self.curr;
        while !self.is_eof() && self.peek() != '"' {
            self.consume();
        }

        let raw = self.input[self.start..self.curr].to_owned();
        println!("STRING: '{}'", raw);

        self.consume(); // move after last `"`
        Expr::Atom(Atom::String(raw))
    }

    pub fn number(&mut self) -> Expr {
        self.start = self.curr - 1;

        if self.previous_char() == '-' || self.previous_char() == '+' {
            self.consume();
        }

        while !self.is_eof() && (self.peek().is_digit(10) || self.peek() == '.') {
            self.consume();
        }

        let number = self.input[self.start..self.curr].to_owned();
        println!("NUMBER: {}", number);

        if number.chars().filter(|c| *c == '.').count() > 1 {
            panic!("A floating-point number can only contain one dot to delimit the integer from the decimal part. ({}:{})", self.line, self.column);
        }

        match number.parse::<i64>() {
            Ok(n) => Expr::Atom(Atom::Integer(n)),
            Err(_) => Expr::Atom(Atom::Float(number.parse::<f64>().unwrap()))
        }
    }

    pub fn symbol(&mut self) -> Expr {
        self.start = self.curr - 1;

        let stop = vec!['(', ')', '\n', '\r', '\t', ' '];

        while !self.is_eof() && !stop.contains(&self.peek()) {
            let c = self.consume();
            // println!("{}", c);
        }

        let symbol = self.input[self.start..self.curr].to_lowercase().to_owned();
        println!("SYMBOL: {}", symbol);

        match symbol.as_str() {
            "lambda" | "\\" => Expr::Atom(Atom::Symbol("lambda".to_owned())),
            _ => Expr::Atom(Atom::Symbol(symbol))
        }
     }

    pub fn sexp(&mut self) -> Expr {
        let mut content: Vec<Expr> = Vec::new();
        let mut level = 0;

        while self.previous_char() != ')' {
            println!("CONSUME: {}", self.previous_char());
            content.push(self.advance_and_read().unwrap());
        }

        Expr::List(content)
    }

    pub fn advance_and_read(&mut self) -> Option<Expr> {
        while self.peek() == ' ' {
            self.consume();
        }
        self.read()
    }

    pub fn read(&mut self) -> Option<Expr> {
        let c = self.consume();
        match c {
            ' ' | '\n' | '\r' | '\t' => None,
            '(' => Some(self.sexp()),
            ')' => panic!("Unexpected closing parenthesis. ({}:{})", self.line, self.column),
            ',' => {
                let next_expr = self.read();
                match next_expr {
                    Some(expr) => Some(Expr::Unquote(Box::new(expr))),
                    None => panic!("Expected an expression after an unquote")
                }
            }
            '\'' => {
                let next_expr = self.read();
                match next_expr {
                    Some(expr) => Some(Expr::Quote(Box::new(expr))),
                    None => panic!("Expected an expression after a quote.")
                }
            }
            '"' => Some(self.string()),
            c => if ((c == '+' || c == '-') && self.peek().is_digit(10)) || c.is_digit(10) {
                Some(self.number())
            } else if c.is_ascii() {
                Some(self.symbol())
            } else {
                panic!("Unexpected token: {} ({}:{})", c, self.line, self.column)
            }
        }
    }
}
