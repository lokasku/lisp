pub mod parser;
pub mod stream;
pub mod builtins;

use std::fs;
use std::env;

use parser::lexer::Lexer;
use parser::parser::{
    Parser,
    Expr,
    Atom
};
use stream::Stream;
use builtins::read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.")
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons.");

    let mut some_stream = Stream::new(
        Expr::Atom(Atom::Symbol(String::from("e"))),
        String::from("(foo a b c 24) (+ (- -1 -2) -4)")
    );

    dbg!(read(&mut some_stream));
    dbg!(read(&mut some_stream));

    let mut lexer = Lexer::new(String::from(content));
    lexer.lex();

    // dbg!(&lexer.output);

    let mut parser = Parser::new(lexer.output);
    parser.parse();

    // dbg!(&parser.output);
}
