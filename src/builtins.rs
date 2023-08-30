use std::unimplemented;

use crate::parser::parser::{
    Sexp,
    SexpT
};
use crate::errors::EvalError;


pub fn print(sexp: Result<Sexp, EvalError>) {
    match sexp {
        Ok(s) => match s.sexpt {
            SexpT::Atom(a) => print!("{}", a),
            SexpT::List(v) => {
                print!("(");
                for s in v {
                    print(Ok(s));
                }
                print!(") ");
            }
        }
        Err(e) => println!("{}", e)
    }
}

pub fn quote(sexp: Sexp) -> Sexp {
    sexp
}
