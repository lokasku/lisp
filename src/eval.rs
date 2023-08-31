use crate::parser::Position;
use crate::parser::parser::{
    Sexp,
    SexpT,
    Atom
};
use crate::errors::{
    EvalError,
    Error
};
use crate::builtins;

const PRIMITIVES: [&str; 8] = ["car", "cdr", "cons", "atom", "quote", "lambda", "cond", "eq"];

fn args_checker(name: String, expected: usize, args: Vec<Sexp>, pos: Position) -> Result<(), Error> {
    let amount = args.len() - 1; // operator is not included
    match amount == expected {
        true => Ok(()),
        false => Err(Error::EvalError(EvalError::ArityMismatch(name, amount, expected, pos)))
    }
}

pub fn eval(ast: Result<Sexp, Error>) -> Result<Sexp, Error> {
    println!("CALL TO EVAL");
    match ast {
        Ok(ref sexp @ Sexp { ref sexpt, pos }) => match sexpt {
            SexpT::Atom(atom) => match atom {
                Atom::Symbol(s) => {
                    if PRIMITIVES.contains(&s.as_str()) {
                        return Ok(sexp.clone())
                    }
                    Err(Error::EvalError(EvalError::UnboundSymbol(s.to_owned(), pos)))
                }
                _ => Ok(sexp.clone()) }
            SexpT::List(v) => match v.get(0) {             // Vec<Sexp>
                Some(Sexp { sexpt, pos }) => match sexpt { // SexpT
                    SexpT::Atom(atom) => match atom {      // SexpT::Atom
                        Atom::Symbol(sn) => match sn.as_str() {
                            "eval" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                eval(Ok(v.get(1).unwrap().clone()))
                            }
                            "quote" => {
                                if let Err(e) = args_checker(sn.to_owned(), 1, v.clone(), *pos) {
                                    return Err(e)
                                }
                                Ok(builtins::quote(v.get(1).unwrap().clone()))
                            }
                            _ => Err(Error::EvalError(EvalError::UnboundSymbol(sn.to_owned(), *pos)))
                        }
                        _ => Err(Error::EvalError(EvalError::IllegalFunctionCall(*pos)))

                    }
                    _ => todo!()
                }
                None => Ok(sexp.clone()),
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}
