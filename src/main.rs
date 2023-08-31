pub mod parser;
pub mod eval;
pub mod errors;
pub mod builtins;

use std::fs;
use std::env;

use parser::parser::{
    quote,
    Parser
};
use eval::eval;

use crate::errors::{
    ReadError,
    EvalError,
    Error
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons.");

    let mut parser = Parser::new(&content);

    loop {
        let ast = parser.read();
        if let Err(Error::ReadError(ReadError::UnexpectedEOF)) = ast {
            break;
        } else {
            builtins::print(eval(ast));
        }
    }
}
