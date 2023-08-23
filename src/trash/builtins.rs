use crate::stream::Stream;
use crate::parser::lexer::Lexer;
use crate::parser::parser::{
    Parser,
    Expr,
    Atom
};

/*pub fn read(stream: &mut Stream) -> Expr {
    let mut stream_lexer = Lexer::new(stream.content.clone());
    stream_lexer.lex_expr();

    stream.content.drain(..&stream_lexer.column - 1);
    stream.content = stream.content.trim().to_owned();

    let mut stream_parser = Parser::new(stream_lexer.output);
    stream_parser.parse_expr()
    Expr::Atom(Atom::Integer(2))
}*/
