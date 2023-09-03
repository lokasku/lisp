use crate::parser::parser::Parser;
use crate::builtins::print;
use crate::eval::eval;
use crate::errors::{
    Error,
    ReadError
};

use std::io;
use std::io::Write;

pub fn repl() {
    println!("Written by Lokasku.");
    println!("Take a look at https://github.com/Lokasku");

    loop {
        let mut input = String::new();

        print!("\n* ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut input);

        let mut parser = Parser::new(input.as_str());
        let ast = parser.read();
        if let Err(Error::ReadError(ReadError::UnexpectedEOF)) = ast {
            continue;
        }
        print(eval(ast));
    }
}
