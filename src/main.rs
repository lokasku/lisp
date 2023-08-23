mod lexer;

use std::fs;
use std::env;
use logos::Logos;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("You must provide filecode.");
    }

    let content = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons.");

    let mut lexer = Lexer::new(&content);
    for i in 0..30 {
        dbg!(&lexer.next());
        dbg!(&lexer.errors);
    }
}
